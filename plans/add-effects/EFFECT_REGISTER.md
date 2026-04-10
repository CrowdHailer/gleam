---
task: 2.3a
status: complete
---

# Task 2.3a: Infer Effect Signature and Register in Type Environment

## Goal

For each `EffectDefinition`, register its operations as callable values in the type
environment, with the effect encoded in each operation's `EffectRow`.

## Design

### Registration flow

`register_effect` is called from `analyse` after type-alias registration and before
function registration (so functions can call effect operations).

For `effect Store(a) { Get() -> a; Set(a) -> Nil }`:

1. A fresh `Hydrator` is created; `make_type_vars` seeds it with the effect's type
   parameters (e.g. `a`), so they are properly generalised.
2. An `EffectRow { effects: [EffectLabel { module, name: "Store" }], var: None }` is
   built — a closed single-effect row.
3. For each operation the hydrator resolves arg/return `TypeAst` → `Arc<Type>`.
4. `fn_with_effects(args, return_, row)` builds the function type; `type_::generalise`
   converts any remaining unbound vars to generic ones.
5. The operation is registered via both `insert_variable` (scope) and
   `insert_module_value` (module interface), with the same publicity as the effect.

### Naming convention

Operations use `Named::CustomTypeVariant` for case-checking (UpperCamelCase), consistent
with the proposal syntax.

## Files Changed

- `compiler-core/src/analyse.rs`
  - Added `EffectDefinition`, `EffectLabel`, `EffectRow` to imports.
  - Added `register_effect` method.
  - Called `register_effect` for each `definitions.effect_definitions` in `analyse`.
- `compiler-core/src/type_/tests/effects.rs` — 4 new unit tests.
- `compiler-core/src/type_/tests.rs` — added `mod effects`.

## Status

Complete. 3778 tests pass (4 new).

## Notes for next tasks

- **2.3b** ("When an effect is performed inside a function, add that effect to the
  enclosing function's row") is largely already handled: `infer_call` calls
  `self.current_effects.merge_from(callee_effects)`, and each operation now carries
  its effect row. Calling `Get()` will automatically propagate `{Store}` to the
  enclosing function's row with no additional code changes needed — verify with a test.
- The preregistered function type in `analyse.rs` (used for mutual-recursion) still uses
  `fn_()` (empty effects). When an effectful function calls itself recursively, the
  `unify(preregistered_type, inferred_type)` in `generalise_function` may fail because
  the preregistered type has no effects. This may need to be fixed before 2.4 tests are
  written if recursive effectful functions are tested.
