---
task: 3.2a
status: complete
---

# Task 3.2a: Translate effect calls into `yield { type: 'EffectName.Operation', args: [...] }`

## Goal

When a Gleam function calls an effect operation (e.g. `Get()` or `Emit(msg)`), the JS backend
must emit a `yield` expression so the generator suspends and delivers the effect request to
the runner loop.

## Approach

Added `effect_name: Option<EcoString>` to `ValueConstructorVariant::ModuleFn`.  
Set to `Some(effect_name)` in `register_effect` (`analyse.rs`), `None` everywhere else.
The `#[serde(default)]` attribute ensures existing `.cache` files can still deserialize.

In `call_with_doc_arguments` (`javascript/expression.rs`), added a new match arm before the
catch-all that fires when the callee is a `Var` with `ModuleFn { effect_name: Some(ename) }`:

```js
yield { type: "EffectName.OpName", args: [arg1, arg2] }
```

`wrap_return` adds `return` at tail position, so tail-position effect calls become
`return yield { ... }`.

## Files Changed

- `compiler-core/src/type_.rs` — added `effect_name: Option<EcoString>` to `ModuleFn`
- `compiler-core/src/analyse.rs` — set `effect_name: Some(ename)` in `register_effect`, `None` elsewhere
- `compiler-core/src/metadata.rs` — pass `effect_name` through the metadata serializer
- `compiler-core/src/inline.rs` — add `effect_name: None` to `InlinableValueConstructor::Function`
- `compiler-core/src/javascript/expression.rs` — new match arm in `call_with_doc_arguments`
- Various test fixtures: `type_/tests.rs`, `metadata/tests.rs`

## Tests

3 new/updated snapshot tests:
- `effectful_function_is_generator` — `Get()` → `yield { type: "Store.Get", args: [] }`
- `private_effectful_function_is_generator` — `Emit(msg)` → `yield { type: "Log.Emit", args: [msg] }`
- `effect_operation_with_multiple_args` — `Request(url, port)` → `yield { type: "Http.Request", args: [url, port] }`

3793 tests pass total.
