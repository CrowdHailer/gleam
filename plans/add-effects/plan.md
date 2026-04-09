# **Proposal: Deep Stateful Algebraic Effects in Gleam**

## **1\. Executive Summary**

This proposal outlines a plan to introduce deep, stateful algebraic effects into the Gleam programming language. This feature will allow developers to isolate side effects from pure computation, significantly improving testability and modularity, while maintaining Erlang/BEAM compatibility and smooth JavaScript interoperability.

### **Core Design Principles**

* **No New Keywords for Resumption:** The continuation function is bound to a standard variable identifier (e.g., resume or cont) within pattern-matching arms, avoiding the need to reserve a resume keyword.  
* **Stateful Handlers:** Handlers can accept an initial state parameter that is threaded through the computation and continuation.  
* **Deep Handlers:** Handlers automatically intercept all subsequent effects of the same type within the computation until completion.  
* **Type-Safe Effect Rows:** The type system tracks effects via row polymorphism, ensuring all effects are handled or explicitly bubbled up.

### **Language Syntax**

1. **Definition:** Effects are defined via a new effect block, similar to custom types.  
2. **Performance:** Calling an effect acts like a normal function call but adds the effect to the function's type signature (e.g., fn() \-\> Int \+ Store(Int)).  
3. **Handling:** A new handle computation() with initial\_state { ... } block evaluates the computation, pattern matches on yielded effects, and handles the final return value.

### **Backend Strategy (JavaScript Focus)**

To avoid rewriting the JavaScript compiler into full Continuation-Passing Style (CPS), effectful functions will be compiled into JavaScript **Generators** (function\*).

* Performing an effect yields a request object.  
* A handler is a runner loop that iterates the generator.  
* Unhandled effects are recursively yielded up the stack (bubbling).  
* The resume variable in Gleam becomes a generated closure in JS that continues the loop with the new state and resolved value.

## ---

**2\. Implementation Plan**

The implementation should be broken down into four major phases. This plan abstracts away specific file names, focusing instead on the logical modules within the Gleam compiler architecture.

### **Phase 1: Syntax, Lexing, and Parsing**

**Target Subsystems:** Lexer, Parser, and AST Definitions.

* **Step 1.1: Lexical Analysis.** Introduce new tokens for the language: Effect, Handle, and With. Ensure Resume is **not** tokenized as a keyword.  
* **Step 1.2: AST Expansion.** \* Add an AST node for defining an effect (similar to custom type declarations).  
  * Add an AST node for the handle expression. This node must capture the target expression, the initial state expression, the pattern-matching clauses for effects, and a mandatory Return clause for the final value.  
* **Step 1.3: Parser Updates.** Update the parser grammar to recognize and construct the new AST nodes for both top-level effect definitions and inline handle expressions.

### **Phase 2: Type System and Inference**

**Target Subsystems:** Type Environment, Type Structures, and Inference Engine.

* **Step 2.1: Type Structures.** Extend function types to include an **Effect Row**. A function type must now represent its arguments, return type, and a set/row of effects it performs.  
* **Step 2.2: Row Polymorphism.** Implement unification logic for effect rows. When function A calls effectful function B, B's effects must unify with and be added to A's effect row.  
* **Step 2.3: Type Inference for Effects.** \* Infer the effect signature of an effect definition.  
  * When an effect is performed, attach that effect type to the enclosing function's row.  
* **Step 2.4: Type Inference for Handlers.** \* Type-check the handle expression body.  
  * Verify that the handler clauses cover the effects they claim to handle.  
  * **Crucial:** Infer the type of the user-named resume binding. It must type-check as a function fn(EffectResolutionType, NextStateType) \-\> HandlerReturnType.  
  * Subtract the handled effects from the expression's overall effect row to determine the final type of the handle block.

### **Phase 3: JavaScript Code Generation**

**Target Subsystems:** JavaScript Backend AST Translator and Code Printer.

* **Step 3.1: Generator Decoration.** Modify the JS code generator to emit function\* instead of function for any Gleam function that carries a non-empty effect row.  
* **Step 3.2: Effect Invocation.** Translate calls to effects into yield statements (e.g., yield { type: 'Store.Get' }). Update calls to other effectful functions to use yield\*.  
* **Step 3.3: Handler Loop Generation.** Translate the handle AST node into a stateful runner loop.  
  * The loop must call .next() on the target generator.  
  * It must intercept recognized yielded objects, apply the user's clause logic, and generate the resume closure.  
  * The resume closure must recursively call the loop with the updated state and the resolution value.  
* **Step 3.4: Effect Bubbling.** Ensure the runner loop includes a fallback branch: if a yielded effect does not match the handler's clauses, it must execute yield unhandled\_effect to pass it to the next handler up the stack.

### **Phase 4: Erlang Code Generation**

**Target Subsystems:** Erlang Backend AST Translator.

* *Note: True algebraic effects require delimited continuations, which BEAM lacks. The architectural decision here dictates the implementation.*  
* **Step 4.1: Choose Strategy.** Determine if the Erlang backend will use Erlang Processes (limits to single-shot continuations but generates clean, idiomatic BEAM code) or a full Continuation-Passing Style (CPS) rewrite (supports multi-shot but highly complex).  
* **Step 4.2: Implementation (Process Strategy).** \* Compile the handle block into a parent process.  
  * Compile the computation to spawn as a linked child process.  
  * Compile effect calls into send (to parent) and receive (wait for resolution).  
  * Compile the resume closure in the parent to send the resolution message back to the child process.  
* **Step 4.3: Implementation (CPS Strategy).** If CPS is chosen, modify all Erlang function generation to accept an implicit continuation parameter and rewrite the AST to thread this continuation through all effectful calls.