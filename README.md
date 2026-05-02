# Recurram (JS)

JavaScript/TypeScript bindings for `recurram-rust` with two backends:

- Node.js: N-API (`recurram-napi`)
- Browser/JS runtime: WebAssembly (`recurram-wasm`)

Integers decode as `bigint` by default (i64/u64 safe handling).

This release line targets the Recurram v2 wire format.

## Requirements

- Node.js 24+
- Rust stable
- `wasm-pack` for WASM builds

## Build

```bash
pnpm install
pnpm build
```

Build steps:

1. Build N-API addon (`native/recurram_napi.node`)
2. Build WASM package (`wasm/pkg/*`)
3. Build TypeScript output (`dist/*`)

## Formatting

```bash
pnpm format
pnpm format:check
```

## Test

```bash
pnpm test
```

What it validates:

- Rust bridge tests (`test:rust`)
- Node API tests (`test:node`) covering `init`, `encode`, `decode`, schema, batch, and session APIs
- TypeScript API usage against built output

## Usage (Node)

```ts
import {
  encode,
  decode,
  createSessionEncoder,
  type RecurramValue,
} from "recurram";

const value: RecurramValue = {
  id: 1001n,
  name: "alice",
  active: true,
};

const bytes = encode(value);
const roundtrip = decode(bytes);

const session = createSessionEncoder();
const first = session.encode(value);
const patch = session.encodePatch({ ...value, name: "alicia" });
```

Node.js picks the N-API backend automatically on first use. The default APIs already use the fastest benchmarked path for each operation, so you should not need to choose between transport JSON, compact JSON, or direct object modes.

## Advanced APIs

If you need raw transport helpers, explicit schema encoding, or internal-format control, import the advanced entrypoint:

```ts
import {
  createSessionEncoder,
  encodeTransportJson,
  encodeWithSchema,
  toTransportJson,
} from "recurram/advanced";
```

This entrypoint contains:

- transport JSON helpers
- compact JSON helpers
- direct object helpers
- schema encoding helpers
- full raw session encoder methods

## Usage (Browser)

```ts
import { init, encode, decode } from "recurram";

await init({ prefer: "wasm" });

const bytes = encode({ id: 1n, role: "admin" });
const value = decode(bytes);
```

Browser/WASM still requires explicit async initialization. If you want to pass a custom WASM source, use `wasmInput`:

```ts
await init({ prefer: "wasm", wasmInput: "/assets/recurram_wasm_bg.wasm" });
```

## TypeScript types

Main exported types:

- `RecurramValue`
- `Schema`, `SchemaField`
- `SessionOptions`

`RecurramValue` includes `bigint` and `Uint8Array` support:

```ts
type RecurramValue =
  | null
  | boolean
  | number
  | bigint
  | string
  | Uint8Array
  | RecurramValue[]
  | { [key: string]: RecurramValue };
```

## Publish to npm

The package is configured for npm publish and ships build artifacts from `dist/`, `native/`, and `wasm/pkg/`.

Local dry run:

```bash
pnpm build
pnpm pack
```

GitHub Actions publish:

1. Add repository secret `NPM_TOKEN`.
2. Bump `version` in `package.json`.
3. Create and push matching tag `v<version>`.

Example:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow `.github/workflows/publish-npm.yml` verifies tag/version match and then runs `pnpm publish`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
