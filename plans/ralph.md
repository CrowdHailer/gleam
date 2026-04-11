Execute these steps.

1. From the unfinished tasks in ,`plans/add-effects/plan.md` choose a logical one to do next.
2. If a progress file is specified in the task, study it. Otherwise create one in `plans/add-effects/progress.md` and reference in the plan file task.
3. Any pending git changes are from a previous attempt. your choice to finish or reset.
4. Do ONLY that task, and related automated tests.
5. Verify (by running tests).
6. If complete: update PLAN to mark complete, commit.
7. If not complete: update progress file and bail.
8. If successfully completed a step commit and push to the `add-effects` branch

## Task Checklist

Tasks are ordered by dependency. Each is self-contained and testable in isolation.

### Phase 1: Syntax, Lexing, and Parsing

- [x] **1.1** Add `Effect`, `Handle`, and `With` tokens to the lexer (`compiler-core/src/parse/lexer.rs`) — progress: `plans/add-effects/LEXER_TOKENS.md`
- [x] **1.2a** Add `EffectDefinition` AST node (name, type params, operations with arg/return types) to `compiler-core/src/ast.rs` — progress: `plans/add-effects/AST_NODES.md`
- [x] **1.2b** Add `HandleExpression` AST node (computation, initial state, effect clauses, return clause) to `compiler-core/src/ast.rs`
- [x] **1.3a** Update top-level parser to parse `effect` blocks into `EffectDefinition` nodes — progress: `plans/add-effects/PARSER_EFFECT_DEF.md`
- [x] **1.3b** Update expression parser to parse `handle ... with ...` into `HandleExpression` nodes — progress: `plans/add-effects/PARSER_HANDLE_EXPR.md`

### Phase 2: Type System and Inference

- [x] **2.1** Extend `Type::Fn` to carry an effect row field representing the set of performed effects — progress: `plans/add-effects/TYPE_FN_EFFECTS.md`
- [x] **2.2** Implement effect row unification: propagate effects from callee to caller during type inference — progress: `plans/add-effects/EFFECT_ROW_UNIFY.md`
- [x] **2.3a** Infer effect signature from an `EffectDefinition`; register it in the type environment — progress: `plans/add-effects/EFFECT_REGISTER.md`
- [x] **2.3b** When an effect is performed inside a function, add that effect to the enclosing function's row — progress: `plans/add-effects/EFFECT_PROPAGATE.md`
- [x] **2.4a** Type-check `HandleExpression` body; verify handler clauses match declared effects — progress: `plans/add-effects/HANDLE_TYPECHECK.md`
- [x] **2.4b** Infer the type of the `resume` binding in each handler clause as `fn(ResolutionType, StateType) -> ReturnType` — progress: `plans/add-effects/HANDLE_TYPECHECK.md`
- [ ] **2.4c** Subtract handled effects from the expression's row to produce the final `HandleExpression` type

### Phase 3: JavaScript Code Generation

- [ ] **3.1** Emit `function*` instead of `function` for any Gleam function with a non-empty effect row in `compiler-core/src/javascript.rs`
- [ ] **3.2a** Translate effect calls into `yield { type: 'EffectName.Operation', ... }` statements
- [ ] **3.2b** Translate calls to other effectful functions to use `yield*` delegation
- [ ] **3.3** Translate `HandleExpression` into a stateful runner loop: call `.next()`, match yielded objects, build `resume` closures, thread state
- [ ] **3.4** Add fallback branch in runner loop to `yield` unrecognised effects upward (effect bubbling)

### Phase 4: Erlang Code Generation

- [ ] **4.1** Decide and document strategy: BEAM processes vs. CPS (update `plans/add-effects/plan.md` with decision)
- [ ] **4.2a** *(Process strategy)* Compile `handle` block into a parent process; spawn computation as linked child
- [ ] **4.2b** *(Process strategy)* Compile effect calls into `send`/`receive` pairs; `resume` closure sends resolution back to child
- [ ] **4.3** *(CPS strategy, if chosen)* Add implicit continuation parameter to all effectful Erlang functions and rewrite AST accordingly

## Progress files

Choose unique short descriptive names 2-4 words formatted `LIKE_THIS.md`. If you need to avoid collision, numbers are fine.

The audience will be a coding agent that needs to continue your task with little help, or understand the history of the execution. Err towards terse mention of past steps (unless something went wrong), more detail on current state.

## Tech Guidelines

Makefile has test, lint (clippy) and check (doing both).

For tests prefer data-driven from cases in in `rust-interpreter/testdata/*.json` listing input/output for examples (or reuse `spec` folder from root project without modifying).