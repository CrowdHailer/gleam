---
task: 3.2a, 3.2b
status: complete
---

# Task 3.2a: Translate effect calls into `yield { type: 'EffectName.Operation', args: [...] }`

## Goal

When a Gleam function calls an effect operation (e.g. `Get()` or `Emit(msg)`), the JS backend
must emit a `yield` expression so the generator suspends and delivers the effect request to
the runner loop.

## Approach

Added `effect_name: Option<EcoString>` to `ValueConstructorVariant::ModuleFn`.  
Set to `Some(effect_name)` in `register_effect` (`analyse.rs`), `None` everywhere else.
The `#[serde(default)]` attribute ensures existing `.cache` files can still deserialize.

In `call_with_doc_arguments` (`javascript/expression.rs`), added a new match arm before the
catch-all that fires when the callee is a `Var` with `ModuleFn { effect_name: Some(ename) }`:

```js
yield { type: "EffectName.OpName", args: [arg1, arg2] }
```

`wrap_return` adds `return` at tail position, so tail-position effect calls become
`return yield { ... }`.

## Files Changed

- `compiler-core/src/type_.rs` — added `effect_name: Option<EcoString>` to `ModuleFn`
- `compiler-core/src/analyse.rs` — set `effect_name: Some(ename)` in `register_effect`, `None` elsewhere
- `compiler-core/src/metadata.rs` — pass `effect_name` through the metadata serializer
- `compiler-core/src/inline.rs` — add `effect_name: None` to `InlinableValueConstructor::Function`
- `compiler-core/src/javascript/expression.rs` — new match arm in `call_with_doc_arguments`
- Various test fixtures: `type_/tests.rs`, `metadata/tests.rs`

## Tests

3 new/updated snapshot tests:
- `effectful_function_is_generator` — `Get()` → `yield { type: "Store.Get", args: [] }`
- `private_effectful_function_is_generator` — `Emit(msg)` → `yield { type: "Log.Emit", args: [msg] }`
- `effect_operation_with_multiple_args` — `Request(url, port)` → `yield { type: "Http.Request", args: [url, port] }`

3793 tests pass total.

## Task 3.2b: Translate calls to effectful functions into `yield*` delegation

### Goal

When a Gleam function calls another effectful Gleam function (one compiled as a
generator), the JS backend must emit `yield*` so that yielded effect objects
propagate to the enclosing runner loop.

### Approach

Added a free function `callee_is_generator(fun: &TypedExpr) -> bool` in
`javascript/expression.rs` that checks `fun.type_()`:

```rust
fn callee_is_generator(fun: &TypedExpr) -> bool {
    match fun.type_().as_ref() {
        Type::Fn { effects, .. } => !effects.effects.is_empty(),
        _ => false,
    }
}
```

Only concrete named effects (`effects.effects.is_empty() == false`) trigger
`yield*` — open row variables (`var: Some(_)`) are type-inference artefacts on
pure recursive functions.

In the catch-all arm of `call_with_doc_arguments`, computed `delegate =
callee_is_generator(fun)` before compiling the callee, then emitted
`yield* fun(args)` instead of `fun(args)` when `delegate` is true.

### Files Changed

- `compiler-core/src/javascript/expression.rs` — `callee_is_generator` helper
  + updated catch-all in `call_with_doc_arguments`
- `compiler-core/src/javascript/tests/functions.rs` — 2 new tests
- Snapshot files for the 3 updated/new tests

### Tests

Updated snapshot:
- `private_effectful_function_is_generator` — call to `log_something` now uses `yield*`

2 new snapshots:
- `effectful_call_uses_yield_star_delegation` — `inner()` → `yield* inner()`
- `calling_effectful_from_another_effectful_uses_yield_star` — multiple `yield*` calls

3795 tests pass total.
