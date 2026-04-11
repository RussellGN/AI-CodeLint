# TypeScript Bug Snippets Notes

This folder contains intentionally buggy TypeScript snippets.
The snippets are comment-free to avoid giving direct hints.

## Notes on snippets

- 01_lang_syntax_semantics_missing_brace.ts: missing closing brace for the for-loop block
- 02_lang_syntax_semantics_type_mismatch.ts: string provided where number is required
- 03_simple_linter_logic_assignment_in_condition.ts: assignment instead of comparison
- 04_simple_linter_logic_switch_fallthrough.ts: low falls through and becomes orange
- 05_advanced_logic_async_map_without_promise_all.ts: returns Promise<string>[] instead of resolved string[]
- 06_advanced_logic_stale_state_after_await.ts: concurrent calls can still write stale data
- 07_edgy_logic_or_instead_of_nullish_coalescing.ts: valid 0% discount is replaced by 10%
- 08_edgy_logic_mutable_default_object_aliasing.ts: mutates shared default object for future calls
