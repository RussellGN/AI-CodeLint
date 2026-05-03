# Lint Samples

Use these to compare results with traditional linters

## 1. correct/

should produce no diagnostics from either a conventional
linter or AI-CodeLint.

| Filename             | Description                                   |
| -------------------- | --------------------------------------------- |
| c01_strlen.c         | Counts characters in a null-terminated string |
| c02_swap.c           | Swaps two integers via pointers               |
| j01_Fibonacci.java   | Iterative Fibonacci computation               |
| j02_IsPrime.java     | Primality check using trial division          |
| p01_binary_search.py | Binary search over a sorted array             |
| p02_flatten.py       | Recursively flattens a nested list            |
| r01_factorial.rs     | Computes factorial using an iterator product  |
| r02_is_palindrome.rs | Checks whether a string is a palindrome       |
| t01_debounce.ts      | Debounce wrapper for a function               |
| t02_chunk.ts         | Splits an array into fixed-size chunks        |

---

## 2. flagged_by_conventional_linters/

implementation and style issues that a standard linter would explicitly catch and flag.

| Filename                  | Issue                                            | Linter           |
| ------------------------- | ------------------------------------------------ | ---------------- |
| c01_unused_variable.c     | Variable declared but never used                 | GCC / Clang      |
| c02_no_return.c           | Non-void function missing return on some paths   | GCC / Clang      |
| c03_shadowed_variable.c   | Inner variable shadows outer variable            | GCC / Clang      |
| j01_empty_catch.java      | Empty catch block silently swallows exception    | PMD / IntelliJ   |
| j02_unused_import.java    | Import declared but never used                   | javac / IntelliJ |
| j03_string_equality.java  | String compared with `==` instead of `.equals()` | PMD / IntelliJ   |
| p01_mutable_default.py    | Mutable default argument                         | Pylint           |
| p02_none_comparison.py    | Comparison to None using `==` instead of `is`    | Pylint / Flake8  |
| p03_bare_except.py        | Bare `except` clause catches all exceptions      | Pylint / Flake8  |
| r01_unused_variable.rs    | Variable declared but never used                 | rustc            |
| r02_unnecessary_mut.rs    | `mut` binding that is never mutated              | Clippy           |
| r03_len_zero.rs           | `.len() == 0` instead of `.is_empty()`           | Clippy           |
| t01_implicit_any.ts       | Parameter implicitly typed as `any`              | ESLint / tsc     |
| t02_unused_variable.ts    | Declared variable is never used                  | ESLint           |
| t03_no_explicit_return.ts | Not all code paths return a value                | tsc              |

---

## 3. not_flagged_by_conventional_linters/

real logic bugs that are syntactically valid and produce no diagnostics from any conventional linter.

| Filename                       | Bug                                                                        |
| ------------------------------ | -------------------------------------------------------------------------- |
| c01_signed_unsigned.c          | Unsigned subtraction wraps around, making negative results appear positive |
| c02_operator_precedence.c      | Shift applies before OR due to precedence, producing wrong bit packing     |
| c03_logic_inversion.c          | `\|\|` should be `&&`, so invalid values pass the check                    |
| j01_wrong_modulo.java          | `n % 2 == 1` fails for negative odd numbers, returns false instead of true |
| j02_string_concat_in_loop.java | String concatenation in loop creates a new object each iteration           |
| j03_index_check.java           | Loop starts at index 1, silently skips the first element                   |

## 4. edge_case/

logic bugs that are difficult to spot even on careful review

| Filename                      | Bug                                                                                    |
| ----------------------------- | -------------------------------------------------------------------------------------- |
| c01_buffer_off_by_one.c       | Null terminator never copied, destination string unterminated                          |
| c02_int_overflow.c            | `lo + hi` overflows for large values before halving                                    |
| c03_strtol_unchecked.c        | Return value gives no indication of parse failure or overflow                          |
| j01_integer_overflow.java     | Accumulator is `int` but return type is `long`, overflow happens before widening       |
| j02_equals_contract.java      | Overloads `equals` instead of overriding it, object equality still used by collections |
| j03_thread_visibility.java    | `running` not declared `volatile`, change may never be visible to the running thread   |
| p01_float_equality.py         | Float subtraction is not exact, `a - b == 0` can fail for equal values                 |
| p02_recursive_default.py      | Mutable default shared across calls, children list grows unexpectedly                  |
| p03_iterator_exhaustion.py    | Generator exhausted after `sum()`, `list(gen)` always returns empty                    |
| r01_cast_truncation.rs        | Silent truncation on cast, high bits dropped with no warning in release mode           |
| r02_float_loop.rs             | Float accumulation never lands exactly on `1.0`, loop runs forever                     |
| r03_unwrap_in_loop.rs         | Panics silently on any empty line, no graceful handling                                |
| t01_wrong_default.ts          | `\|\| 0` masks legitimate zero values, should use `?? 0`                               |
| t02_object_spread_mutation.ts | `Object.assign` mutates the base object rather than creating a new one                 |
| t03_precision_loss.ts         | Integer exceeds `Number.MAX_SAFE_INTEGER`, parsed value is silently incorrect          |
