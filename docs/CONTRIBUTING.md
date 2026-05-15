# Contributing

Thank you for improving the Recurram JavaScript/TypeScript bindings.

## Scope

This repository contains:

- TypeScript API (`src/`)
- N-API native addon (`crates/recurram-napi`, `native/`)
- WebAssembly build (`crates/recurram-wasm`, `wasm/`)
- Node and integration tests (`tests/`)

Keep changes aligned with the normative spec in [recurram/recurram](https://github.com/recurram/recurram).

## Development

Requirements:

- Node.js 24+
- Rust stable
- `wasm-pack` for WASM builds

```bash
pnpm install
pnpm build
pnpm test
pnpm format:check
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/).

Use this format:

```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Common types include `feat`, `fix`, `docs`, `refactor`, `test`, `build`, `ci`, and `chore`.

Examples:

- `feat: add encodePatch transport helper`
- `fix(napi): handle empty batch payloads`

After `pnpm install`, Husky runs Commitlint on each local commit. Pull requests are also checked in CI so every commit in the branch follows the same rules.

## Pull Requests

Use the pull request template and fill in every required section. PR bodies are validated in CI.

## Contribution Checklist

- Tests added or updated for behavior changes
- `pnpm test` and `pnpm format:check` pass locally
- Documentation updated when the public API changes
- Commit messages follow Conventional Commits

By contributing to this repository, you agree that your contribution may be distributed under the MIT license used by the project.
