# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- GitHub issue templates (feature request and bug report) and pull request template.
- Direct v2-to-JS NAPI decoder (`try_decode_v2_native`) that builds JS objects straight from v2 wire bytes without an intermediate Rust `Value` tree, improving `decode` throughput by ~72%.
- `js_to_recurram_value` NAPI helper using raw NAPI API for JS→Rust value conversion with full `BigInt` support via `get_bigint_u64_raw` / `get_bigint_i64_raw`.
- `encodeBatchNativeRaw` NAPI function that converts a JS array directly to `Vec<Value>` using the raw NAPI traversal API, avoiding `serde_json::Value` and the associated BigInt panic.
- `getrandom` dependency with `wasm_js` feature in `recurram-wasm` to satisfy the `wasm32-unknown-unknown` target requirement introduced by `recurram v2.0.0`.

### Changed

- `decode()` in the JS layer now uses the `decodeNative` fast path on the N-API backend (restored from the v2 migration).
- Integer encoding for JS `number` values in `encodeFast` is now fully inlined: `BigInt()` conversion is avoided for integers up to 2³² so encoding stays in the `number` type space for all common data.
- `ByteWriter.writeVaruint` now takes a direct fast path for values < 128 and replaces `% 0x80` with `& 0x7f` in the loop body.
- `getCachedUtf8` no longer skips the cache for strings longer than 64 bytes; the existing 4096-entry eviction bound already limits memory use.
- `ByteWriter` is now pooled at module level with a `reset()` + `finish()` (copy) API, eliminating per-call allocation and repeated buffer growth.
- `EncodeState` Maps (`keyIds`, `stringIds`, `shapeIds`) are now pooled at module level and cleared per call instead of being recreated on every `encodeFast` invocation.
- N-API backend `decode_native_napi` now skips the compact-protocol attempt for v2 bytes (first byte > `0x02`), avoiding wasted parsing work.
- `recurram-bridge` and `recurram-napi` Cargo dependencies now reference the local `recurram-rust` crate via a workspace-relative path instead of the published `recurram = "0.1.0"`, ensuring the v2 codec is used throughout.

### Fixed

- CI workflow (`ci.yml`): added a `git clone` step to check out `recurram-rust` alongside `recurram-js` before the Rust build, fixing the `failed to read recurram-rust/Cargo.toml` error that caused all CI jobs to fail.
- `encodeBatchNativeRaw` no longer panics when the JS array contains `BigInt` values; the function now uses `JsUnknown` with raw NAPI traversal instead of `serde_json::Value`.

## [2.0.0] - 2026-05-01

### Added

- Recurram v2 wire tags with fixint/fixstr/fixarray/fixmap families in the fast TypeScript codec.
- Per-message key and string interning (`key_ref`, `str_ref`) for smaller one-shot JSON payloads.
- In-array same-shape object encoding via shape definition reuse.

### Changed

- `encode()` in the JS fast path now emits v2 wire bytes by default.
- Project release version bumped to `2.0.0` for the v2 clean break.

## [0.1.0] - 2026-03-25

Initial public release of the JavaScript and TypeScript bindings for Recurram.

### Added

- Node.js N-API and browser WASM backends behind a shared `init()` runtime selection API.
- High-level encode and decode APIs for `RecurramValue`, schema-aware encoding, batch encoding, and session-based patch and micro-batch workflows.
- Transport JSON, compact JSON, and direct object fast paths for lower JS-side overhead in hot paths.
- TypeScript type exports for runtime options, schemas, session options, and transport-compatible values including `bigint` and `Uint8Array`.
- Rust bridge crates, N-API packaging, WASM packaging, Node test coverage, CI, npm publish automation, and release tag verification.

### Changed

- Renamed npm package from `gowe` to `recurram`
- Raised the Node.js runtime baseline to `24+` across local tooling, documentation, and publish workflows.
- Expanded the public API with fast-mode helpers for compact encoding, raw transport JSON encoding, direct encoding, and additional session encoder variants.
- Optimized bridge, N-API, WASM, and TypeScript runtime paths to reduce overhead for encode, decode, batch, session, and compact transport operations.

### Fixed

- Corrected the Rust crate path used by the workspace so native builds resolve the bridge crate correctly.

[unreleased]: https://github.com/recurram/recurram-js/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/recurram/recurram-js/releases/tag/v0.1.0
