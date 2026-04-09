---
name: AST Nodes Progress
description: Progress for tasks 1.2a and 1.2b - adding EffectDefinition and HandleExpression AST nodes
type: project
---

# Task 1.2a: EffectDefinition AST node

## Status: COMPLETE

Added to `compiler-core/src/ast.rs` just before `UntypedDefinition`:
- `EffectOperation` struct: location, name, name_location, arguments (Vec<TypeAst>), return_type (TypeAst)
- `EffectDefinition` struct: location, end_position, name, name_location, publicity, parameters (Vec<SpannedString>), operations, documentation
- `EffectDefinition::full_location()` method

NOT yet added to `Definition` enum — that is part of task 1.3a (parser integration).

# Task 1.2b: HandleExpression AST node

## Status: COMPLETE

Added to `compiler-core/src/ast.rs` after `EffectDefinition`:
- `EffectClause` struct: location, effect_name, effect_name_location, operation_name,
  operation_name_location, arguments (Vec<SpannedString>), resume (SpannedString), body (UntypedExpr)
- `EffectReturnClause` struct: location, value (SpannedString), body (UntypedExpr)
- `HandleExpression` struct: location, computation (Box<UntypedExpr>), initial_state (Box<UntypedExpr>),
  effect_clauses (Vec<EffectClause>), return_clause (Box<EffectReturnClause>)

NOT yet a variant of `UntypedExpr` — that is part of task 1.3b (parser integration).
