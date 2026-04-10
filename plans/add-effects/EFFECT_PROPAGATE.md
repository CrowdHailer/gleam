---
task: 2.3b
status: complete
---

# Task 2.3b: Propagate Performed Effects to Enclosing Function's Row

## Goal

When an effect operation is called inside a function body, the function's inferred
`Type::Fn` should carry that effect in its `effects: EffectRow` field.

## Root Cause Investigation

The mechanism was already in place at the call site (`infer_call` calls
`self.current_effects.merge_from(callee_effects)`), but three bugs prevented the
effects from reaching `type_info.values`:

### Bug 1 — `instantiate` dropped effects (environment.rs)

`Environment::instantiate` matched `Type::Fn { arguments, return_, .. }` and
reconstructed the type with `fn_()` (empty effects).  Fixed by matching on `effects`
and using `fn_with_effects(..., effects.clone())`.

### Bug 2 — pre-registration used a closed empty row (analyse.rs)

`register_value_from_function` registered functions with `fn_()` (empty effects,
closed row).  Later, `unify(preregistered_type, inferred_type)` failed because
`unify_effect_rows` rejects two closed rows with differing effect sets.

Fixed by registering with an open row `EffectRow { effects: [], var: Some(fresh_uid) }`.
An open row passes `unify_effect_rows` unconditionally.

### Bug 3 — scope was updated with the stale preregistered type (analyse.rs)

After inference, `insert_variable` was called with `preregistered_type.clone()` (still
empty effects).  `generalise_function` reads from scope, so the module interface ended
up with an empty effect row.

Fixed by:
1. Cloning `type_` before passing to `unify` (so it is not moved).
2. Passing `type_` (the inferred type with effects) to `insert_variable`.

## Files Changed

- `compiler-core/src/type_/environment.rs`
  - `instantiate`: `Type::Fn` arm now uses `fn_with_effects(..., effects.clone())`.
- `compiler-core/src/analyse.rs`
  - `register_value_from_function` (line ~1659): open effect row in preregistered type.
  - `infer_function` (line ~756): `type_.clone()` in unify call.
  - `infer_function` (line ~801): `type_` (not `preregistered_type`) in `insert_variable`.
- `compiler-core/src/type_/tests/effects.rs`
  - 3 new tests: `effect_propagates_to_enclosing_function_row`,
    `effect_propagates_transitively`, `pure_function_has_empty_effect_row`.

## Snapshot Updates

Introducing the open-row UID shifted all subsequent type variable IDs by one,
changing some printed generic names (e.g. `M` → `O`) in 23 Erlang/JS snapshot tests.
All snapshots were accepted with `INSTA_UPDATE=always`.

## Status

Complete. 3781 tests pass (3 new effects tests).

## Notes for next tasks

- **2.4a** can now assume that any `HandleExpression` body's inferred type carries the
  correct effect row for all operations performed inside it.
- The open-row approach for preregistered types means a recursive effectful function
  call is typed as "any effects" during the recursive call.  This is sound but loses
  precision for self-recursive calls.  Task 2.4 may want to revisit if it becomes
  an issue.
