# AI CodeLint VS Code Extension

Use LLMs to lint code.

This extension runs the `ai-codelint` language server in the background and surfaces AI-assisted logic diagnostics directly in VS Code.

## Project Status

AI CodeLint is in heavy development and not production-ready yet.

- Features and behavior can change quickly.
- Breaking changes are expected while core behavior is refined.

## What This Extension Does

- Starts `ai-codelint --mode server` as an LSP server.
- Analyzes open files and untitled buffers (`file` and `untitled` URI schemes).
- Reports likely logic bugs that can survive compilation and traditional linting.

## Prerequisites

1. Install the `ai-codelint` binary.
2. If already installed, make sure its at the latest version.
3. Acquire api key from [OpenRouter](https://openrouter.ai/keys).
4. Run `ai-codelint --configure` and complete all steps.

### Install The Binary (or update existing)

macOS/Linux:

```sh
curl -fsSL https://raw.githubusercontent.com/RussellGN/AI-CodeLint/main/scripts/install.sh | sh
```

Windows (PowerShell):

```powershell
irm https://raw.githubusercontent.com/RussellGN/AI-CodeLint/main/scripts/install.ps1 | iex
```

Then verify installation:

```sh
ai-codelint --version
```

## Configure Runtime Defaults

Run the CLI walkthrough once to set defaults such as model and max output tokens, and to get API key setup guidance:

```sh
ai-codelint --configure
```

Current evaluation recommends `anthropic/claude-sonnet-4.6` for best lint quality.

## For developers

### Run In Extension Development Host (Local Development)

From the repository root:

```sh
cd server && cargo build
cd ../clients/vs-code && pnpm i && pnpm compile
```

Then in VS Code, run the launch config named **Launch VS Code Client** (F5). The workspace launch configuration already sets:

- `SERVER_PATH=${workspaceRoot}/server/target/debug/ai-codelint`

### Package As VSIX And Install Locally (Unreleased Builds)

From `clients/vs-code`:

```sh
pnpm i
pnpm compile
pnpm package
```

Install the generated `.vsix` in VS Code using:

- Extensions: Install from VSIX...

## Settings

The extension currently contributes one setting:

- `ai-codelint.trace.server`
   - `off`: no trace logging
   - `messages`: error-only trace output (default)
   - `verbose`: full trace output

Use the **AI CodeLint trace** output channel in VS Code when troubleshooting.

## Troubleshooting

### "OPENROUTER_API_KEY environment variable is required"

- Acquire api key from [OpenRouter](https://openrouter.ai/keys) and set `OPENROUTER_API_KEY` globally for your OS/shell.
- Restart VS Code completely.

### "Version check failed"

- Update your local setup to the latest recommended release.
- Ensure extension and binary versions are up to date.

### Server Does Not Start (For developers)

- Verify `ai-codelint --version` works in a terminal.
- If using a custom path, verify `SERVER_PATH` points to an executable binary.
- Set `ai-codelint.trace.server` to `verbose` and check the **AI CodeLint trace** channel.

## Related Docs

- [Project README](../../README.md)
- [Project changelog](../../CHANGELOG.md)
- [TypeScript lint samples](../../lint%20samples/typescript)
