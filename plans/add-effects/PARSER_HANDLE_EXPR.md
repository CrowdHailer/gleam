---
name: Parser Handle Expression Progress
description: Progress for task 1.3b - parsing handle...with expressions into HandleExpression nodes
type: project
---

# Task 1.3b: Parse `handle ... with ...` into HandleExpression nodes

## Status: COMPLETE

## What was already done (previous session)

`parse_handle_expression()` was fully implemented in `compiler-core/src/parse.rs` at line ~2704. It:
- Parses `handle <computation> with <initial_state> { ... }`
- Matches `with` as a plain name token (not a reserved keyword)
- Parses `EffectName.OperationName(arg1, ..., resume) -> body` clauses
- Parses mandatory `Return(value) -> body` clause
- Returns `UntypedExpr::Handle(HandleExpression { ... })`
- Dispatched from expression parser at line ~888 on `Token::Handle`

The `UntypedExpr::Handle` variant was already added (task 1.2b).

## What was done in this session

Added 5 snapshot parse tests in `compiler-core/src/parse/tests.rs`:
- `handle_expression_return_only` — bare handle with only a Return clause
- `handle_expression_with_one_effect_clause` — one effect clause + Return
- `handle_expression_with_multiple_effect_clauses` — two effect clauses + Return
- `handle_expression_missing_with` — error: `{` where `with` expected
- `handle_expression_missing_return_clause` — error: no Return clause

Also added `UntypedExpr::Handle(_)` to two exhaustive match arms in the type checker
to allow compilation (type_/expression.rs:4550, type_/pipe.rs:209) — both route to
existing fallback paths; actual type-checking deferred to Phase 2.

All 3769 tests pass.
