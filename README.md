# AI CodeLint

Boost code quality as you type.

AI CodeLint uses LLMs to identify the logic bugs that survive compilation and traditional linting.

## Install (Recommended)

macOS/Linux:

```sh
curl -fsSL https://raw.githubusercontent.com/RussellGN/AI-CodeLint/main/scripts/install.sh | sh
```

Windows (PowerShell):

```powershell
irm https://raw.githubusercontent.com/RussellGN/AI-CodeLint/main/scripts/install.ps1 | iex
```

After install:

```sh
ai-codelint --version
```

## Development Status

AI CodeLint is in heavy development and is not production-ready yet.

- Features and APIs can change quickly.
- Breaking changes are expected while core behavior is refined.
- Current releases should be treated as pre-release testing builds.

## Interface

AI CodeLint consists of one standalone binary: `ai-codelint`.

That binary currently exposes two modes of operation via the command line:

- CLI mode: run lint checks from the terminal, like conventional linters.
- LSP server mode: run as a language server so IDE clients can request diagnostics.

Right now, the only IDE client available is the VS Code extension in this repository (unpublished). More editor integrations are planned later.

## Inference

AI CodeLint intentionally uses OpenRouter so inference-provider and model selection is quick and easy change and experiment with.

- **You will need an [OpenRouter api key](https://openrouter.ai/keys) to use AI CodeLint**.
- At the time of writing, signing up is free, with free models available for use with limits. 
- Run `ai-codelint --configure` to set up api key and other optional runtime defaults.

Model quality note:

- Best results so far come from frontier models.
- Lesser models can produce highly variable lint quality and consistency.
- Recommended model: `anthropic/claude-sonnet-4.6`.

## Examples

The repository includes intentionally buggy TypeScript samples under `lint samples/typescript`.

These samples cover:

- syntax/semantics mistakes (should be ignored by linter),
- easy rule-based linter catches,
- subtle logic bugs,
- edge-case behavior and review-evasive patterns.

They are useful for evaluation, and comparing model performance.

Resources:

- Official Docs (still in development): [russellgn.github.io/AI-CodeLint](https://russellgn.github.io/AI-CodeLint)

