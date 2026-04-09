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

## Status: TODO
