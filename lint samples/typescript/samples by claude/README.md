# TypeScript Buggy Code Samples

Bug annotations extracted from the source files. Each section corresponds to its `.ts` file and documents what is wrong, why it is wrong, and what correct behaviour looks like.

---

## `syntax_semantics_1.ts`

**Category:** Language syntax / semantics (compiler-level error)

**Bug:** Interface declares `getLabel(): string` but the implementing class returns `string | null`. TypeScript will reject this — the return type is not assignable to the interface's declared type, yet the intent looks superficially correct because `null` is "close enough" to `string` at a glance.

**Error site:** `getLabel()` method on `Product`

- Return type `string | null` is not assignable to `string`.

---

## `syntax_semantics_2.ts`

**Category:** Language syntax / semantics (compiler-level error)

**Bug:** The function asserts `value as T` without any real constraint that `T` extends a type that makes the cast valid. The generic is unconstrained, so TypeScript cannot verify the cast is safe — the `as unknown as T` double-cast silences the compiler entirely, making this a semantic hole that defeats the type system silently.

**Error site:** `return JSON.parse(value) as unknown as T`

- The double-cast (`as unknown as T`) bypasses all type checking.
- Callers believe they're getting a `T`, but they're really just getting whatever the raw string parse produces — no runtime validation at all.

**Inline consequence notes:**

- Caller trusts the return type completely and uses it without guards.
- `user.id.toFixed(2)` → Runtime: `"not-a-number".toFixed` is not a function.
- `user.isAdmin === true` → Runtime: `undefined === true` → `false` silently.

---

## `linter_catchable_logic_1.ts`

**Category:** Simple errors catchable by traditional linters (ESLint)

**Bugs (all ESLint-catchable):**

1. `==` used instead of `===` (`eqeqeq` rule) — `"0" == false` is `true` in JS.
2. Variable `multiplier` is declared but never used (`no-unused-vars` rule).
3. Direct NaN comparison with `==` always returns `false` (`use-isnan` rule).

**Error site 1:** `if (score == NaN)`

- `NaN == NaN` is always `false`; should use `Number.isNaN(score)`.

**Error site 2:** `if (score == false)`

- `==` coerces types before comparing. When `score` is `0`, `score == false` is `true`, misclassifying `0` as a fail.

**Inline consequence notes:**

- `processScore("0")` — should print `"fail"` for the right reason; passes by accident.
- `processScore("NaN")` — will NOT print `"invalid input"` because `NaN == NaN` is `false`.
- `processScore("75")` — `"pass"`, works fine.

---

## `linter_catchable_logic_2.ts`

**Category:** Simple errors catchable by traditional linters (ESLint)

**Bugs (all ESLint-catchable):**

1. Missing `break` in switch case `"warn"` — falls through to the `"error"` branch (`no-fallthrough` rule).
2. `retries > 0 || retries <= 0` is always `true` — dead `else` branch (`no-constant-condition` / similar rule).

**Error site 1:** `case "warn":` block — no `break` before `case "error":`.

- Every warning will also trigger the error handler.

**Error site 2:** `if (retries > 0 || retries <= 0)`

- Condition is a tautology — the `else` branch is unreachable dead code.
- `console.log("No retry logic needed.")` — never reached.

**Wrong output for `handleLog("warn", 3)`:**

```
[WARN] Something looks off.
[ERROR] Critical failure.       ← should not appear for a warning
Paging on-call engineer...      ← should not appear for a warning
```

---

## `subtle_logic_1.ts`

**Category:** Advanced / subtle errors that cannot be caught by traditional linters

**Bug:** `applyDiscount` receives a `cart` object and mutates it directly instead of working on a copy. The caller's original object is silently modified. TypeScript types are perfectly valid, no linter rule fires — the bug is purely about reference semantics being mistaken for value semantics.

**Error site:** `const discounted = { ...cart };`

- This is a shallow copy — `cart.items` is still the _same_ array reference.
- Mutating each `item.price` inside `forEach` mutates the caller's original objects too.

**Inline consequence notes:**

- `myCart.total` prints `300` (unchanged) — misleading; the top-level field was not mutated, but...
- `myCart.items[0].price` prints `90` — should still be `100`. The original cart was silently corrupted.
- `saleCart.total` prints `270` — the discounted cart is correct, but at the cost of destroying the source.

---

## `subtle_logic_2.ts`

**Category:** Advanced / subtle errors that cannot be caught by traditional linters

**Bug:** Classic off-by-one in binary search. The upper bound starts at `arr.length` instead of `arr.length - 1`, and the right-shrink uses `hi = mid` instead of `hi = mid - 1` when a higher value is found. This causes an infinite loop for certain inputs — `lo` never advances past the last two elements when the target is not present.

**Error site 1:** `let hi = arr.length;`

- Should be `arr.length - 1`.

**Error site 2:** `hi = mid;` in the `else` branch.

- Should be `hi = mid - 1`; can trap the loop when `lo === mid`.

**Inline consequence notes:**

- `binarySearch(sorted, 23)` → `5` — works by luck.
- `binarySearch(sorted, 91)` → `hi=9, lo=9, mid=9, arr[9]=91 === target` — finds it, but only because the target happens to sit at the exact boundary.
- `binarySearch(sorted, 100)` → `lo=9, hi=10 → mid=9 → arr[9]<100 → lo=10`; loop exits here, returns `-1`, but with `hi = arr.length` the last-element miss path is subtly wrong for other array sizes.

---

## `review_evasive_logic_1.ts`

**Category:** Very edgy / subtle errors that easily pass manual code review by advanced developers

**Bugs:**

**Bug 1 — Operator precedence / string concatenation trap:**
`"Actions: " + count + extra` — `+` is left-associative. The first `+` operand is a string, so all subsequent `+` operators are also string concatenations, not additions. With `count=5, extra=2` this produces `"Actions: 52"` instead of `"Actions: 7"`. The expression reads like arithmetic but behaves as string joining.

**Bug 2 — Bitmask check is not a strict gate:**
`(userPerms & PERM_WRITE) > 0` correctly tests whether the WRITE bit is set, but it does NOT verify that WRITE is the _only_ elevated permission. Auditors reading this assume it's a strict WRITE-only gate; it is not. A user with `READ | WRITE | EXECUTE` passes the same check as one with only `WRITE`.

**Inline consequence notes:**

- `PERM_READ` = `0b001` (1), `PERM_WRITE` = `0b010` (2), `PERM_EXECUTE` = `0b100` (4).
- `canWrite(perms)` → `true` — correct result, but for the wrong structural reason.
- `canWrite(PERM_READ | PERM_EXECUTE)` → `false` — correct coincidentally.
- `buildAuditLine("alice", "upload", 5, 2)` prints `"[AUDIT] alice | upload | Actions: 52"` — should be `"Actions: 7"`.

---

## `review_evasive_logic_2.ts`

**Category:** Very edgy / subtle errors that easily pass manual code review by advanced developers

**Bug:** A batch of async tasks is launched inside a `for` loop using `var` for the index. Because `var` is function-scoped (not block-scoped), every closure over `i` captures the _same_ variable. By the time any async callback fires, `i` has already been incremented to `tasks.length` by the loop completing.

The code reads naturally — it looks like each task gets its own index — and the `await`-free inner IIFE makes it even easier to miss. TypeScript types are completely clean. ESLint's `no-loop-func` rule would normally catch this, but here the closure is inside an immediately-called async IIFE, which many lint configurations fail to flag.

**Error site:** `for (var i = 0; i < tasks.length; i++)`

- `var` — not block-scoped. All IIFE closures share the same `i`.
- `await processItem(tasks[i], i)` — `i` is closed over, not captured by value.

**Wrong output for all three tasks:**

```
Processed task #3 → id=undefined
Processed task #3 → id=undefined
Processed task #3 → id=undefined
```

**Expected output:**

```
Processed task #0 → id=101
Processed task #1 → id=102
Processed task #2 → id=103
```

**Fix:** Replace `var i` with `let i` to give each loop iteration its own block-scoped binding.
