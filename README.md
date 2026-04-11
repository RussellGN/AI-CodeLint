# AI CodeLint

Boost code quality as you type.

AI CodeLint is an AI-first linter focused on finding real runtime and behavioral logic bugs that can survive compilation and traditional linting.

## Development Status

AI CodeLint is in heavy development and is not production-ready yet.

- Features and APIs can change quickly.
- Breaking changes are expected while core behavior is refined.
- Current releases should be treated as pre-release testing builds.

## Why It Is Different From Conventional Linters

Conventional linters are great at syntax, formatting, type-level issues, and common rule-based smells. AI CodeLint targets a different layer: subtle logic and runtime behavior bugs that are often valid code and can pass normal lint/type checks.

Examples include stale async state, shallow-copy mutation side effects, incorrect fallback operators, and review-evasive logic mistakes.

AI CodeLint is designed to complement existing linters, not replace them.

## Project Structure and Interfaces

AI CodeLint primarily consists of one standalone binary: `ai-codelint`.

That binary currently exposes two interfaces:

- CLI interface: run lint checks from the terminal, like conventional linters.
- LSP server interface: run as a language server so IDE extensions can request diagnostics.

Right now, the only IDE client in progress is the VS Code extension in this repository. More editor integrations are planned later.

## Availability

- Main binary: available now for pre-release testing.
- VS Code extension: in active development and not yet published.

Links:

- Latest release: [github.com/ai-codelint/releases/latest](https://github.com/ai-codelint/releases/latest)
- Project website (still in development): [russellgn.github.io/ai-codelint](https://russellgn.github.io/ai-codelint)

## OpenRouter-First Model Strategy

AI CodeLint intentionally uses OpenRouter so model selection is easy to switch over time without hard-coupling to a single model provider.

- Authentication uses the `OPENROUTER_API_KEY` environment variable.
- You can change models through configuration or runtime flags.

Model quality note:

- Best results so far come from frontier models.
- Lesser models can produce highly variable lint quality and consistency.
- Recommended model: `anthropic/claude-sonnet-4.6`.

## Lint Samples Included

The repository includes intentionally buggy TypeScript samples under `lint samples/typescript` from multiple sources (ChatGPT, Claude, and GPT-Codex).

These samples cover:

- syntax/semantics mistakes,
- easy rule-based linter catches,
- subtle runtime logic bugs,
- edge-case behavior,
- and review-evasive patterns.

They are useful for evaluation, regression testing, and comparing model performance.

## Other Noteworthy Details

- The binary checks recommended version status at startup.
- Current IDE integration focus is TypeScript in VS Code.
- The project is rapidly iterating toward a more complete release workflow and broader editor support.
