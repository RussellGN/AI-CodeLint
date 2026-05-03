# Lint Samples

A curated dataset of code samples used to evaluate an AI-augmented linter. Samples are organised
by category reflecting the nature and detectability of any issues they contain. The dataset spans
Python, TypeScript, C, Rust, and Java.

---

## correct/

Clean, bug-free implementations that should produce no diagnostics from either a conventional linter or the AI-augmented linter.

| Filename             | Description                                   |
| -------------------- | --------------------------------------------- |
| p01_binary_search.py | Binary search over a sorted array             |
| p02_flatten.py       | Recursively flattens a nested list            |
| t01_debounce.ts      | Debounce wrapper for a function               |
| t02_chunk.ts         | Splits an array into fixed-size chunks        |
| c01_strlen.c         | Counts characters in a null-terminated string |
| c02_swap.c           | Swaps two integers via pointers               |
| r01_factorial.rs     | Computes factorial using an iterator product  |
| r02_is_palindrome.rs | Checks whether a string is a palindrome       |
| j01_Fibonacci.java   | Iterative Fibonacci computation               |
| j02_IsPrime.java     | Primality check using trial division          |

---

## easily_flagged/

Flawed snippets containing issues that a standard linter would explicitly catch and flag.

| Filename                  | Bug                                              | Linter           |
| ------------------------- | ------------------------------------------------ | ---------------- |
| p01_mutable_default.py    | Mutable default argument                         | Pylint           |
| p02_none_comparison.py    | Comparison to None using `==` instead of `is`    | Pylint / Flake8  |
| p03_bare_except.py        | Bare `except` clause catches all exceptions      | Pylint / Flake8  |
| t01_implicit_any.ts       | Parameter implicitly typed as `any`              | ESLint / tsc     |
| t02_unused_variable.ts    | Declared variable is never used                  | ESLint           |
| t03_no_explicit_return.ts | Not all code paths return a value                | tsc              |
| c01_unused_variable.c     | Variable declared but never used                 | GCC / Clang      |
| c02_no_return.c           | Non-void function missing return on some paths   | GCC / Clang      |
| c03_shadowed_variable.c   | Inner variable shadows outer variable            | GCC / Clang      |
| r01_unused_variable.rs    | Variable declared but never used                 | rustc            |
| r02_unnecessary_mut.rs    | `mut` binding that is never mutated              | Clippy           |
| r03_len_zero.rs           | `.len() == 0` instead of `.is_empty()`           | Clippy           |
| j01_empty_catch.java      | Empty catch block silently swallows exception    | PMD / IntelliJ   |
| j02_unused_import.java    | Import declared but never used                   | javac / IntelliJ |
| j03_string_equality.java  | String compared with `==` instead of `.equals()` | PMD / IntelliJ   |
