use crate::{
    assert_module_error,
    assert_module_infer,
    type_::Type,
    type_::tests::compile_module,
};

/// Operations of a pub effect definition are registered as public callable values.
#[test]
fn effect_operations_registered_as_values() {
    assert_module_infer!(
        r#"
pub effect Store(a) {
  Get() -> a
  Set(a) -> Nil
}
"#,
        vec![("Get", "fn() -> a"), ("Set", "fn(a) -> Nil")]
    );
}

/// A private effect's operations are not exported.
#[test]
fn private_effect_operations_not_exported() {
    assert_module_infer!(
        r#"
effect Logger {
  Log(String) -> Nil
}
"#,
        vec![]
    );
}

/// Effect operations with multiple arguments.
#[test]
fn effect_operation_multiple_arguments() {
    assert_module_infer!(
        r#"
pub effect Http {
  Get(String, Int) -> String
}
"#,
        vec![("Get", "fn(String, Int) -> String")]
    );
}

/// Effect operations can be called from functions; the effect propagates to the
/// enclosing function's row (verified indirectly by the function's inferred type).
#[test]
fn effect_operation_callable_from_function() {
    assert_module_infer!(
        r#"
pub effect Store(a) {
  Get() -> a
}

pub fn fetch() -> Int {
  Get()
}
"#,
        vec![("Get", "fn() -> a"), ("fetch", "fn() -> Int")]
    );
}

/// When a function calls an effect operation, the effect is added to the
/// enclosing function's effect row.
#[test]
fn effect_propagates_to_enclosing_function_row() {
    let src = r#"
pub effect Store(a) {
  Get() -> a
}

pub fn fetch() -> Int {
  Get()
}
"#;
    let module = compile_module("test_module", src, None, vec![])
        .into_result()
        .expect("should infer successfully");

    let fetch_type = module
        .type_info
        .values
        .get("fetch")
        .expect("fetch should be registered")
        .type_
        .clone();

    match fetch_type.as_ref() {
        Type::Fn { effects, .. } => {
            assert!(
                effects.effects.iter().any(|e| e.name == "Store"),
                "expected Store in effect row, got {:?}",
                effects
            );
        }
        other => panic!("expected Fn type, got {:?}", other),
    }
}

/// Effect propagates transitively: a function calling an effectful function
/// also picks up those effects.
#[test]
fn effect_propagates_transitively() {
    let src = r#"
pub effect Store(a) {
  Get() -> a
}

pub fn fetch() -> Int {
  Get()
}

pub fn indirect() -> Int {
  fetch()
}
"#;
    let module = compile_module("test_module", src, None, vec![])
        .into_result()
        .expect("should infer successfully");

    let indirect_type = module
        .type_info
        .values
        .get("indirect")
        .expect("indirect should be registered")
        .type_
        .clone();

    match indirect_type.as_ref() {
        Type::Fn { effects, .. } => {
            assert!(
                effects.effects.iter().any(|e| e.name == "Store"),
                "expected Store in effect row, got {:?}",
                effects
            );
        }
        other => panic!("expected Fn type, got {:?}", other),
    }
}

/// A function that performs no effects has an empty effect row.
#[test]
fn pure_function_has_empty_effect_row() {
    let src = r#"
pub fn add(x: Int, y: Int) -> Int {
  x + y
}
"#;
    let module = compile_module("test_module", src, None, vec![])
        .into_result()
        .expect("should infer successfully");

    let add_type = module
        .type_info
        .values
        .get("add")
        .expect("add should be registered")
        .type_
        .clone();

    match add_type.as_ref() {
        Type::Fn { effects, .. } => {
            assert!(
                effects.is_empty(),
                "expected empty effect row for pure function, got {:?}",
                effects
            );
        }
        other => panic!("expected Fn type, got {:?}", other),
    }
}

/// A handle expression with only a Return clause type-checks and returns the
/// value produced by the Return clause body.
#[test]
fn handle_return_clause_only() {
    assert_module_infer!(
        r#"
pub effect Store(a) {
  Get() -> a
}

pub fn run() -> Int {
  handle 42 with Nil {
    Return(v) -> v
  }
}
"#,
        vec![("Get", "fn() -> a"), ("run", "fn() -> Int")]
    );
}

/// A handle expression with one effect clause type-checks correctly.
#[test]
fn handle_with_one_effect_clause() {
    assert_module_infer!(
        r#"
pub effect Store(a) {
  Get() -> a
  Set(a) -> Nil
}

pub fn run() -> Int {
  handle Get() with Nil {
    Store.Get(resume) -> resume(0, Nil)
    Return(v) -> v
  }
}
"#,
        vec![
            ("Get", "fn() -> a"),
            ("Set", "fn(a) -> Nil"),
            ("run", "fn() -> Int")
        ]
    );
}

/// A handle clause referencing an operation that does not exist produces an error.
#[test]
fn handle_clause_unknown_operation() {
    assert_module_error!(
        r#"
pub effect Store(a) {
  Get() -> a
}

pub fn run() -> Int {
  handle Get() with Nil {
    Store.DoesNotExist(resume) -> resume(0, Nil)
    Return(v) -> v
  }
}
"#
    );
}

/// A handle clause with the wrong number of arguments produces an error.
#[test]
fn handle_clause_wrong_arity() {
    assert_module_error!(
        r#"
pub effect Store(a) {
  Get() -> a
}

pub fn run() -> Int {
  handle Get() with Nil {
    Store.Get(extra_arg, resume) -> resume(0, Nil)
    Return(v) -> v
  }
}
"#
    );
}
