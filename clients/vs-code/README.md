# AI CodeLint VS Code Extension

Boost code quality as you type.

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
2. Set `OPENROUTER_API_KEY` as a global environment variable.
3. Keep your extension/binary version aligned with the recommended version in `status.json`.

### Install The Binary (Recommended)

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

The root README currently recommends `anthropic/claude-sonnet-4.6` for best lint quality.

## Install The VS Code Extension

Install from the Visual Studio Marketplace:

- [AI CodeLint (russell-gn.ai-codelint)](https://marketplace.visualstudio.com/items?itemName=russell-gn.ai-codelint)

Or install via CLI:

```sh
code --install-extension russell-gn.ai-codelint
```

### Option A: Run In Extension Development Host (Local Development)

From the repository root:

```sh
cd server && cargo build
cd ../clients/vs-code && pnpm i && pnpm compile
```

Then in VS Code, run the launch config named **Launch VS Code Client** (F5). The workspace launch configuration already sets:

- `SERVER_PATH=${workspaceRoot}/server/target/debug/ai-codelint`

### Option B: Package As VSIX And Install Locally (Unreleased Builds)

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

## Environment Variables

- `OPENROUTER_API_KEY` (required): API key used for lint inference.
- `SERVER_PATH` (optional): path to a custom `ai-codelint` binary. If not set, the extension uses `ai-codelint` from your PATH.

Important: after changing environment variables globally, fully restart VS Code so the extension host picks them up.

## Troubleshooting

### "OPENROUTER_API_KEY environment variable is required"

- Set `OPENROUTER_API_KEY` globally for your OS/shell.
- Restart VS Code completely.

### "Current version 'x' of ai-codelint is out of date"

- Update your local setup to the latest recommended release.
- Ensure extension and binary versions are up to date.

### Server Does Not Start

- Verify `ai-codelint --version` works in a terminal.
- If using a custom path, verify `SERVER_PATH` points to an executable binary.
- Set `ai-codelint.trace.server` to `verbose` and check the **AI CodeLint trace** channel.

## Development

From repository root:

```sh
cd server && cargo build && cd ../clients/vs-code && pnpm i && pnpm compile
```

There is also a workspace task named **install and compile** that runs the same workflow.

### Useful Scripts (`clients/vs-code`)

- `pnpm compile`: type-check and bundle extension output
- `pnpm watch`: watch mode for rapid extension iteration
- `pnpm lint`: run `oxlint`
- `pnpm fmt`: run `oxfmt --write`
- `pnpm package`: build a `.vsix` artifact

## Related Docs

- [Project README](../../README.md)
- [Project changelog](../../CHANGELOG.md)
- [TypeScript lint samples](../../lint%20samples/typescript)
