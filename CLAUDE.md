# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **Gleam compiler** — a friendly, type-safe language that compiles to Erlang (BEAM VM) and JavaScript (Node/Deno/Bun). It's written in Rust.

## Build and Test Commands

```bash
make build                     # Build release binary (cargo build --release)
make install                   # Build and install gleam to PATH
make test                      # Run all tests (unit + integration)
make test-watch                # Run tests on file changes (requires watchexec)
make language-test             # Run language integration tests only
make language-test-watch       # Language tests with file watching
```

Running specific tests:
```bash
cargo test module_name         # Run tests matching a name pattern
cargo test --package compiler-core  # Test a specific crate
cd test/language && make erlang     # Erlang target integration tests only
cd test/language && make nodejs     # JavaScript/Node integration tests only
```

Linting:
```bash
cargo clippy                   # Run linter (strict rules enforced)
cargo deny check               # Check dependency licenses/advisories
```

Snapshot tests (used heavily for compiler output):
```bash
cargo insta review             # Interactively approve/reject snapshot changes
make insta-check-unused        # Check for unused snapshots
make insta-fix-unused          # Remove unused snapshots
```

Debug logging: `GLEAM_LOG=trace cargo test`

## Architecture

The codebase is a Rust workspace. The key crates:

### `compiler-core/` — Pure compiler engine (no IO)
All parsing, type-checking, analysis, and code generation lives here. Being IO-free enables clean testing and multiple front-ends.

Key modules:
- `parse.rs` — Source → untyped AST
- `type_.rs` — Type checking and inference
- `ast.rs` — AST type definitions
- `erlang.rs` / `javascript.rs` — Code generation per target
- `build.rs` — Build orchestration
- `format.rs` — Code formatter
- `exhaustiveness.rs` — Pattern match exhaustiveness
- `analyse.rs` — Reference/call analysis
- `metadata.rs` — Serialization of compiled module metadata (cached to `.cache/`)

### `compiler-cli/` — CLI + IO layer
Wraps `compiler-core` with file system operations, HTTP, and the actual `gleam` command implementations (`build`, `run`, `format`, `add`, `new`, `publish`, etc.).

### `language-server/` — LSP implementation
Large crate. Key files: `engine.rs` (core logic), `code_action.rs` (fixes/refactors), `completer.rs` (autocomplete), `router.rs` (message dispatch).

### `compiler-wasm/` — WebAssembly interface
Built with `wasm-pack`. Exposes compiler to JavaScript/browser environments.

### `gleam-bin/` — Binary entry point
Thin wrapper that calls `compiler-cli::main()`.

## Compilation Flow

```
Gleam source                .cache binaries
     ↓                            ↓
  Parser                Metadata deserializer
     ↓                            ↓
Untyped AST ←──── Dependency sorter ────→ Module metadata
                         ↓
                   Type checker
                         ↓
                     Typed AST
                    ↙         ↘
           Code generator   Metadata serializer
                ↓                   ↓
         Pretty printer       .cache binaries
                ↓
       Erlang or JavaScript source
```

## Testing Approach

- **Unit tests**: Inline in Rust modules via `#[test]`
- **Snapshot tests**: `cargo-insta` is used extensively for testing compiler output (errors, generated code, diagnostics). Snapshot files live in `src/snapshots/` subdirectories. When output intentionally changes, run `cargo insta review` to accept new snapshots.
- **Integration tests**: Real Gleam projects in `test/` directory, compiled and executed against actual runtimes (Erlang/OTP, Node.js, Deno, Bun)
- **Language tests**: `test/language/` contains cross-target feature tests

## Development Environment Requirements

- Rust (stable)
- Erlang 28+
- Elixir 1.18+
- Node.js 25+
- Deno 2.2+
- Bun 1.2+
- rebar3 3+

Optional: `watchexec` (for `*-watch` targets), `wasm-pack` (for WASM build), Docker (cross-compilation).

## Git / SSH

Pushing to GitHub requires the deploy key at `~/.ssh/gleam_deploy_key`. If push fails with a permission denied error, ensure `~/.ssh/config` contains:

```
Host github.com
  IdentityFile ~/.ssh/gleam_deploy_key
  IdentitiesOnly yes
```

Then `git push` will work normally.

## Key Docs

- `docs/compiler/README.md` — Detailed architecture and compilation flow
- `CONTRIBUTING.md` — Development setup and Rust tips
- `RELEASE.md` — Release process
