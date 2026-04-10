---
task: 2.4a
status: complete
---

# Task 2.4a: Type-check HandleExpression; verify handler clauses match declared effects

## Goal

Replace the `TypedExpr::Todo` stub in `UntypedExpr::Handle` arm with real type inference:
- Infer the computation expression
- Look up each clause's effect operation in scope
- Verify the operation is a real effect operation with the matching effect name
- Bind clause arguments to the correct types
- Bind `resume` as an unbound var (2.4b will assign the real type)
- Type-check each clause body in a new scope
- Type-check the return clause (binding the return value)
- Return a `TypedExpr::Handle` with the return clause body's type

## New Types Needed

- `TypedEffectClause` in `ast.rs`
- `TypedEffectReturnClause` in `ast.rs`
- `TypedExpr::Handle` variant in `ast/typed.rs`
- Error variants in `type_/error.rs`:
  - `HandleClauseNotAnEffect { location, name }` - operation not an effect
  - `HandleClauseEffectMismatch { location, clause_effect, actual_effect }` - wrong effect prefix
  - `HandleClauseWrongArity { location, expected, got }` - wrong number of arguments

## Files to Change

- `compiler-core/src/ast.rs` — add typed structs
- `compiler-core/src/ast/typed.rs` — add Handle variant
- `compiler-core/src/ast/visit.rs` — add Handle arm
- `compiler-core/src/type_/error.rs` — add new errors
- `compiler-core/src/type_/expression.rs` — implement infer_handle
- `compiler-core/src/erlang.rs` — stub Handle arm
- `compiler-core/src/javascript/expression.rs` — stub Handle arm
- `compiler-core/src/inline.rs` — add Handle arm
- `compiler-core/src/analyse.rs` — add Handle arm if any exhaustive match

## Current Status

Complete. All exhaustive matches updated, infer_handle implemented, 4 new tests added and passing (3785 total).
