---
name: Lexer Tokens Progress
description: Progress tracking for task 1.1 - adding Effect, Handle, With tokens
type: project
---

# Task 1.1: Add Effect, Handle, With tokens to lexer

## Status: COMPLETE

## Plan
- Add `Effect`, `Handle`, `With` variants to `Token` enum in `compiler-core/src/parse/token.rs`
- Map `"effect"`, `"handle"`, `"with"` strings to tokens in `string_to_keyword()` in `compiler-core/src/parse/lexer.rs`
- `resume` is NOT a keyword (stays as a `Name` token per design)
- Add Display impl, guard_precedence arm, is_reserved_word arm for each new token

## Notes
- Keywords list is alphabetical in both files
- `Effect` fits between `Echo` and `Else`
- `Handle` fits between `Hash` and `If`  
- `With` fits after `Use` / before end
