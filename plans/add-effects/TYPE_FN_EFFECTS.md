---
task: 2.1
status: in_progress
---

# Task 2.1: Extend Type::Fn with Effect Row

## Goal

Add `effects: EffectRow` field to `Type::Fn` in `compiler-core/src/type_.rs`.

## Design

`EffectRow` is a new struct representing the set of algebraic effects a function may perform.
`EffectLabel` identifies an effect by module+name.

```rust
pub struct EffectLabel { pub module: EcoString, pub name: EcoString }
pub struct EffectRow   { pub effects: Vec<EffectLabel>, pub var: Option<u64> }
```

`var: None` = closed row (pure or fully concrete effect set).
`var: Some(id)` = open row (polymorphic, can be extended during unification in task 2.2).

Both derive `Default` so `EffectRow::default()` = empty closed row (pure function).

## Files Changed

- `compiler-core/src/type_.rs` — add structs, add field, update same_as/generalise/unify_unbound_type
- `compiler-core/src/type_/prelude.rs` — update `fn_()` constructor
- `compiler-core/src/exhaustiveness.rs` — preserve effects on Fn reconstruction
- `compiler-core/src/metadata.rs` — preserve effects on Fn reconstruction
- `language-server/src/engine.rs` — add effects to construction in get_function_type
- `compiler-core/src/type_/printer.rs` — add `..` and update test construction
- `compiler-core/src/type_/pretty.rs` — add `..` and update test construction
- All other files — add `..` to exhaustive pattern matches

## Status

Complete. All 3769 tests pass.
