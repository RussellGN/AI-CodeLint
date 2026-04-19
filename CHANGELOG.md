# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.8] - 2026-04-19

### fixed

- LSP server startup error reporting

## [0.0.7] - 2026-04-18

### Added

- Clearer guidance when selected models are from free tiers.

### Changed

- Improved CLI and lint output formatting.

### Fixed

- Fixed bug in which LLM output was being incorrectly parsed (almost 50% of the time).
- Fixed multiple inference/linter error paths to surface friendlier failures.
- Fixed install scripts' UX.
- Fixed config prompt wording and interaction clarity.

## [0.0.6] - 2026-04-17

### Added

- Installation scripts (`scripts/install.sh`, `scripts/install.ps1`).

### Changed

- Included source filename context in lint prompts.
- Made VS Code extension activation language-agnostic.

### Fixed

- Fixed recommended version metadata handling.
- Fixed extension error handling to avoid restarting the LSP server after startup failure.

## [0.0.5] - 2026-04-14

### Changed

- Refined linter preamble requirements so each reported issue has structured `BUG`, `WHY`, and `IMPACT` sections inside JSON output.

### Fixed

- Fixed LSP document-open caching to store the actual document text (instead of the URI), preventing invalid first-pass diagnostics.

## [0.0.4] - 2026-04-11

### Added

- Interactive configuration walkthrough (`--configure`), with bonus API key setup guidance.
- Basic TypeScript lint sample sets from LLM sources.
- Initial content, and deployment for project landing/docs.

### Changed

- Improved CLI UX with more standardized and readable terminal output.
- Runtime mode handling is now more explicit and missing arguments return a clear error path.
- Refined lint constraints for better model output consistency.
- Improved lint diagnostic display formatting.
- Expanded project README.

## [0.0.3] - 2026-04-08

### Summary

- pre-release of AI CodeLint binary
