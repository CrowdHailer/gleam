---
task: 2.2
status: complete
---

# Task 2.2: Effect Row Unification

## Goal

Implement effect row unification and propagate effects from callee to caller during type inference.

## Design

Follows the same save/restore pattern as the existing `purity` tracking in `ExprTyper`.

### Core pieces

- `EffectRow::merge_from(&mut self, other: &EffectRow)` — merges another row's effects in, deduplicating; adopts open-row var if not already set.
- `unify_effect_rows(row1, row2) -> Result<(), ()>` — equal rows OK; either open → OK; two different closed rows → Err.
- `ExprTyper::current_effects: EffectRow` — accumulates effects during body type-checking.
- In `infer_fn` (anonymous functions): save/clear/restore `current_effects`; build type with `fn_with_effects(...)`.
- In `infer_call`: after purity merge, call `self.current_effects.merge_from(callee_effects)`.
- In `pipe.rs`: same merge in each purity-merge site.
- In `analyse.rs` (top-level functions): use `fn_with_effects(..., expr_typer.current_effects)` instead of `fn_()`.
- In `environment.rs` `unify`: `Type::Fn` branch now extracts and calls `unify_effect_rows`.

## Files Changed

- `compiler-core/src/type_.rs` — add `EffectRow::merge_from`
- `compiler-core/src/type_/environment.rs` — add `unify_effect_rows`, wire into `unify` for `Type::Fn`, add 5 unit tests
- `compiler-core/src/type_/expression.rs` — add `current_effects` field, init, save/restore in `infer_fn`, accumulate in `infer_call`
- `compiler-core/src/type_/pipe.rs` — merge effects at each purity-merge site
- `compiler-core/src/analyse.rs` — use `fn_with_effects` when building top-level function types

## Status

Complete. 3774 tests pass (5 new unit tests for `unify_effect_rows`).

## Notes for next tasks

- Task 2.3b will add actual effects to `current_effects` when an effect operation is performed.
- The preregistered type in `analyse.rs` (line ~750) still uses `fn_()` (empty effects). Once 2.3b populates real effects, the `unify(preregistered_type, type_)` call will fail unless the preregistered type uses an open effect row. That fix belongs in 2.3a/2.3b.
