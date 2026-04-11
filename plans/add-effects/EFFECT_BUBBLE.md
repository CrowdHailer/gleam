---
task: 3.4
status: complete
---

# Task 3.4: Fallback branch in runner loop for effect bubbling

## Goal

When a `handle` expression only handles some effects, unhandled effects must
propagate (bubble) upward to the enclosing generator.

## Approach

The key insight: the loop arrow function cannot `yield`, so we need a different
structure when bubbling is required.

### Detection

`bubbling = outer_in_generator_context` — if the enclosing function is a
generator, there are remaining unhandled effects that must propagate.

### Non-bubbling structure (unchanged)

```js
(() => {
  const gen = computation();
  const loop = (state, $next) => { … };
  return loop(initial_state, undefined);
})()
```

### Bubbling structure (new)

```js
yield* (function*() {
  const gen = computation();
  function* loop(state, $next) {
    …
    if ($effect.type === "Effect.Op") { … }
    return yield $effect;          // ← fallback: bubble unrecognised effect
  }
  return yield* loop(initial_state, undefined);
})()
```

The outer `return` (from tail position) produces `return yield* (…)()`.

### Clause bodies in bubbling mode

When `bubbling` is true, `in_generator_context` is set to `true` while
compiling each clause body.  This ensures effectful calls inside clause bodies
(including calls to `resume`, which now returns a generator) get `yield*`
delegation.

## Files Changed

- `compiler-core/src/javascript/expression.rs`
  - `handle_expression`: detect `bubbling`, choose loop syntax and IIFE kind,
    return `yield* iife` when bubbling
  - `gen_handle_loop_body`: new `bubbling: bool` param; set
    `in_generator_context = true` for clause bodies; add `return yield $effect;`
    fallback
  - New `immediately_invoked_generator_expression_document` helper

## Tests

1 new test: `handle_expression_with_effect_bubbling` (partial handler — Log only,
Store.Get bubbles).  3799 total tests pass.
