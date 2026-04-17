---
task: 4.1
status: complete
---

# Task 4.1: Erlang Backend Strategy Decision

## Decision: Process Strategy

Use BEAM processes as coroutines to implement algebraic effects on Erlang.

## Rationale

- **Matches JS semantics**: JS backend uses generators (single-shot). Processes give equivalent single-shot continuation semantics on BEAM.
- **Idiomatic BEAM**: Leverages native Erlang concurrency primitives — no compiler infrastructure changes needed.
- **Implementation simplicity**: CPS would require a full rewrite of the Erlang backend, threading an implicit continuation through every function call site.
- **Sufficient for use case**: Single-shot limitation is acceptable; Gleam's design does not require multi-shot continuations.

## Rejected: CPS Strategy

CPS would support multi-shot continuations but requires:
- An implicit `Cont` parameter added to every effectful Erlang function signature
- Every call site in the Erlang backend rewritten to thread `Cont`
- Massive complexity for negligible benefit given the single-shot JS baseline

## Implementation Pattern (for tasks 4.2a/4.2b)

```erlang
%% Gleam: handle computation() with initial_state { Store.Get(key) -> ... }
%%
%% Compiles to:

-spec handle_expr_iife() -> ReturnType.
handle_expr_iife() ->
    Parent = self(),
    Child = spawn_link(fun() ->
        %% effectful computation runs here; effect calls send to Parent
        Res = computation(),
        Parent ! {gleam_effect_done, Res}
    end),
    handle_loop(Child, InitialState).

handle_loop(Child, State) ->
    receive
        {gleam_effect, 'Store.Get', [Key], ResumeTag} ->
            %% execute handler clause; resume is a fn(Value, NewState)
            Resume = fun(Value, NewState) ->
                Child ! {gleam_resume, ResumeTag, Value},
                handle_loop(Child, NewState)
            end,
            %% ... clause body using Key, Resume, State ...
            ;
        {gleam_effect_done, Value} ->
            %% return clause
            ...
    end.
```

Effect call site (`Store.Get(key)`) compiles to:
```erlang
Tag = make_ref(),
erlang:self() ! {gleam_effect, 'Store.Get', [Key], Tag},   %% actually send to parent
receive {gleam_resume, Tag, Value} -> Value end
```

The child must know who its parent is. Pass `Parent` as an extra implicit argument to the computation, or capture it via closure.

## Key Constraints

- **Single-shot only**: child blocks on `receive`; only one resume per effect.
- **No nested cross-process bubbling** without spawning additional processes.
- Bubbling (unhandled effects) propagates by forwarding the effect message to the grandparent.
