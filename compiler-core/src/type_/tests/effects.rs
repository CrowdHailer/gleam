use crate::assert_module_infer;

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
