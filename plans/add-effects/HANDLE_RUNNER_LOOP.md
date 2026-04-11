---
task: 3.3
status: complete
---

# Task 3.3: Translate `HandleExpression` into a stateful runner loop

## Goal

Compile `handle computation() with initial_state { ... }` into a JavaScript stateful
runner-loop IIFE that calls `.next()` on the computation generator, dispatches on
yielded effect objects, builds `resume` closures, and threads state.

## Approach

### New `in_generator_context` field

Added `in_generator_context: bool` to `expression::Generator` (default `false`).  
Set to `true` in `javascript.rs` when compiling a module function that is a generator
(`function*`). Reset to `false` when entering anonymous functions (`fn_`) or handle
IIFEs.

The `yield*` delegation in `call_with_doc_arguments` is now gated on this flag:

```rust
if delegate && self.in_generator_context {
    self.wrap_return(docvec!["yield* ", call])
}
```

This prevents invalid `yield*` inside handle IIFEs (arrow functions, not generators).

### `handle_expression` method

New method in `expression::Generator`:

1. Saves and overrides outer state (`scope_position`, `function_position`,
   `current_scope_vars`, `statement_level`, `current_function`, `in_generator_context`).
2. Allocates fresh names (`gen`, `loop`) via `next_local_var` to avoid shadowing.
3. Compiles computation and initial_state in non-tail position.
4. Inserts `"state"` into `current_scope_vars` as the conventional current-state name
   available to clause bodies.
5. Delegates to `gen_handle_loop_body` to build the loop arrow-function body.
6. Assembles the IIFE and restores all outer state.

### `gen_handle_loop_body` method

Generates the body of `(state, $next) => { ... }`:

- `const $r = gen.next($next);` — advance the generator
- `if ($r.done) { const v = $r.value; return <return_body>; }` — return clause
- `const $effect = $r.value;` — get yielded effect (when effect_clauses is non-empty)
- For each effect clause:
  - `if ($effect.type === "Effect.Op") {`
  - `const arg_n = $effect.args[n];` — destructure operation arguments
  - `const resume = ($res, $ns) => loop($ns, $res);` — resume closure
  - `return <clause_body>;`

### State convention

`state` is the hardcoded parameter name of the loop arrow function.  Clause bodies
reference it as a free variable; the generated JS provides it via the loop parameter.
The type-checker already requires `state` to be in scope in the enclosing Gleam function
(it is the initial-state expression), so types check out.

## Files Changed

- `compiler-core/src/javascript/expression.rs`
  - Added `in_generator_context: bool` field to `Generator`
  - `call_with_doc_arguments`: gate `yield*` on `in_generator_context`
  - `fn_`: save/restore `in_generator_context` (always sets to `false`)
  - New `handle_expression` method
  - New `gen_handle_loop_body` method
- `compiler-core/src/javascript.rs`
  - Set `generator.in_generator_context = is_generator` before `function_body`
- `compiler-core/src/javascript/tests/functions.rs`
  - 3 new snapshot tests

## Tests

3 new tests, all passing:

- `handle_expression_return_mapping` — single-arg effect clause + return mapping
- `handle_expression_with_one_effect_clause` — no-arg effect + stateful resume
- `handle_expression_with_multiple_effect_clauses` — two effects, tuple return

3798 total tests pass.
