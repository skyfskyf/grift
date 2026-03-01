# Grift Language Specification

**Version:** 1.5.0
**Status:** Draft
**Based on:** Kernel Programming Language (Shutt, R-1RK, 2010)

## 1. Overview

Grift is a Lisp dialect implementing a strict subset of the Kernel programming
language.  It is based on the **vau calculus**, where the fundamental
abstraction mechanism is the *operative* (fexpr) rather than the lambda.
Applicatives (ordinary functions) are derived by wrapping operatives.

Grift is designed for embedded and resource-constrained systems:

- **No standard library (`no_std`)** — only `core::` types.
- **No heap allocation (`no_alloc`)** — all values live in a fixed-size arena.
- **No unsafe code** — `#![forbid(unsafe_code)]` enforced crate-wide.

## 2. Lexical Syntax

### 2.1 Character Set

Source text is a sequence of ASCII bytes.  All tokens are drawn from the
printable ASCII range plus whitespace characters.

### 2.2 Whitespace and Comments

Whitespace characters (`\x20`, `\t`, `\n`, `\r`) separate tokens.
A semicolon (`;`) begins a line comment that extends to the next newline or
end of input.

### 2.3 Tokens

| Token class  | Examples                              |
|-------------|---------------------------------------|
| Open paren  | `(`                                   |
| Close paren | `)`                                   |
| Dot         | `.`  (only as pair separator)         |
| Quote       | `'`  (shorthand for `(quote …)`)      |
| String      | `"hello"`, `"line\n"`                 |
| Boolean     | `#t`, `#f`, `#true`, `#false`         |
| Special     | `#inert`, `#ignore`                   |
| Number      | `42`, `-7`, `+3`                      |
| Symbol      | `foo`, `define!`, `+`, `my-var?`      |

### 2.4 Strings

String literals are delimited by double quotes.  The following escape
sequences are recognised: `\n` (newline), `\t` (tab), `\r` (carriage return),
`\\` (backslash), `\"` (double quote).  Unrecognised escapes signal
`InvalidArgument`.

Strings are internally represented as linked lists of `CharPair` nodes in the
arena.

### 2.5 Numbers

Numbers are signed integers (Rust `isize`).  An optional leading `+` or `-`
is permitted.  Overflow during parsing or arithmetic signals
`ArithmeticOverflow`.

### 2.6 Symbols

A symbol is any run of non-delimiter, non-whitespace characters that does not
parse as a boolean, special value, or number.  Symbols are interned: each
unique name maps to exactly one arena slot.

## 3. Types

Grift has 12 value types.  Every value occupies exactly one arena slot.

| #  | Type          | Self-eval | Mutable | `eq?` basis |
|----|---------------|-----------|---------|-------------|
| 1  | Nil           | yes       | no      | value       |
| 2  | Boolean       | yes       | no      | value       |
| 3  | Number        | yes       | no      | value       |
| 4  | Symbol        | no        | no      | value       |
| 5  | Pair (Cons)   | no        | no      | identity    |
| 6  | String (CharPair) | yes   | no      | identity    |
| 7  | Operative     | yes       | no      | identity    |
| 8  | Applicative   | yes       | no      | identity    |
| 9  | Builtin       | yes       | no      | identity    |
| 10 | Environment   | yes       | yes     | identity    |
| 11 | Inert         | yes       | no      | value       |
| 12 | Ignore        | yes       | no      | value       |

**Value equality** means `eq?` compares content.  **Identity equality** means
`eq?` compares arena slot indices.

## 4. Evaluation

### 4.1 Self-Evaluating Forms

All types except Symbol and Pair evaluate to themselves.

### 4.2 Symbol Lookup

The evaluator searches the environment chain:

1. Search the current environment's own bindings (association list).
2. If a single parent exists, continue there.
3. If multiple parents exist, perform depth-first search with cycle detection.
4. If not found, signal `UnboundVariable`.

### 4.3 Combination

A pair `(combiner . operands)` is evaluated as follows:

1. Evaluate `combiner` to obtain a value.
2. If it is not a combiner type, signal `NotCallable`.
3. Dispatch:
   - **Operative / Builtin:** pass operands unevaluated and the dynamic
     environment.
   - **Applicative:** evaluate each operand left-to-right in the dynamic
     environment, then pass the result list to the underlying combiner.

### 4.4 Tail-Call Optimisation

The evaluator implements TCO via a trampoline loop.  The following forms
produce tail calls:

- `if` — the selected branch.
- `begin` — the last expression.
- `cond` — the selected clause body.
- `and` / `or` — the last expression.
- `vau` / `lambda` body — the last expression.
- `let` / named `let` — the body.

### 4.5 Truthiness

Grift uses **strict boolean semantics**.  Conditional forms (`if`, `and`,
`or`, `cond`, `not`) require their test expressions to produce `#t` or `#f`.
Non-boolean values in boolean context signal `TypeError`.

## 5. Formal Parameter Trees (Kernel §4.9.1)

Parameters are matched recursively:

- **Symbol** — binds to the corresponding argument.
- **`#ignore`** — discards the argument.
- **`()`** — requires the argument to be nil.
- **Pair** — requires a pair argument; recurse on car and cdr.

Constraints: no duplicate symbols; acyclic; `env-param` of `vau` must not
duplicate any formal.

## 6. Primitive Forms

### 6.1 Operatives (unevaluated operands)

| Form | Signature |
|------|-----------|
| `quote` | `(quote expr)` |
| `if` | `(if test then [else])` |
| `define!` | `(define! ptree expr)` or `(define! (fn name params…) body…)` |
| `set!` | `(set! env sym expr)` |
| `vau` | `(vau ptree env-param body…)` |
| `lambda` | `(lambda ptree body…)` |
| `begin` | `(begin expr…)` |
| `cond` | `(cond (test body…)…)` |
| `and` | `(and expr…)` |
| `or` | `(or expr…)` |
| `let` | `(let ((name val)…) body…)` or `(let name ((p v)…) body…)` |
| `current-environment` | `(current-environment)` |

### 6.2 Applicatives (evaluated arguments)

| Form | Signature |
|------|-----------|
| `+`, `-`, `*`, `/` | Arithmetic |
| `=`, `<`, `>`, `<=`, `>=` | Numeric comparison |
| `cons`, `car`, `cdr`, `list` | Pair operations |
| `null?`, `pair?`, `number?`, `symbol?`, `boolean?`, `inert?`, `ignore?`, `operative?`, `applicative?`, `environment?` | Type predicates (variadic) |
| `not` | Boolean negation |
| `eq?`, `equal?` | Equality |
| `eval` | `(eval expr [env])` |
| `wrap`, `unwrap` | Combiner wrapping |
| `make-environment`, `make-empty-environment` | Environment construction |
| `apply` | `(apply combiner args [env])` |
| `error` | `(error msg)` |
| `gc-collect` | `(gc-collect)` |

## 7. Error Conditions

| Error | Condition |
|-------|-----------|
| `OutOfMemory` | Arena full, cannot allocate. |
| `IndexOutOfBounds` | Arena index exceeds capacity. |
| `IndexNotAllocated` | Slot was freed or never allocated. |
| `InvalidArgument` | Bad argument to an operation. |
| `TraceError` | GC mark stack overflow or invalid roots. |
| `Cyclic` | Cycle detected in traversal. |
| `TypeError` | Wrong type for operation. |
| `ParseError { line, col }` | Malformed S-expression at given source location. |
| `ArithmeticOverflow` | Checked arithmetic overflowed. |
| `DivisionByZero` | Division or modulo by zero. |
| `UnboundVariable` | Symbol not found in environment chain. |
| `NotCallable` | Value is not a combiner. |
| `ImmutableEnvironment` | Attempted mutation of ground environment. |

## 8. Memory Model

All values reside in a single `Arena<Value, N>` with compile-time capacity `N`.
The arena uses:

- **Free-list allocation** — O(1) alloc and free.
- **Interior mutability** — `Cell<T>` for safe mutation without `&mut`.
- **Mark-and-sweep GC** — triggered automatically on OOM; can also be invoked
  explicitly via `(gc-collect)`.

Singleton slots (NIL, TRUE, FALSE, INERT, IGNORE, environments) are
pre-allocated and never collected.

## 9. References

- Shutt, J. N. (2010). *Revised⁻¹ Report on the Kernel Programming Language*.
- Shutt, J. N. (2007). *Fexprs as the basis of Lisp function application*.
