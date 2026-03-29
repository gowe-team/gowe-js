import { initBackend, requireBackend } from "./backend.js";
import {
  deserializeCompact,
  serializeCompact,
  serializeCompactBatch,
  serializeSessionOptions,
  serializeValue,
} from "./transport.js";
import type { InitOptions, RecurramValue, SessionOptions } from "./types.js";
import type { RuntimeKind, RuntimeSessionEncoder } from "./runtime/types.js";

export type {
  InitOptions,
  RecurramValue,
  Schema,
  SchemaField,
  SessionOptions,
  UnknownReferencePolicy,
} from "./types.js";

export async function init(options: InitOptions = {}): Promise<RuntimeKind> {
  return initBackend(options);
}

export function encode(value: RecurramValue): Uint8Array {
  return requireBackend().encodeCompactJson(serializeCompact(value));
}

export function decode(bytes: Uint8Array): RecurramValue {
  return deserializeCompact(requireBackend().decodeToCompactJson(bytes));
}

export function createSessionEncoder(
  options: SessionOptions = {},
): SessionEncoder {
  const raw = requireBackend().createSessionEncoder(
    serializeSessionOptions(options),
  );
  return new SessionEncoder(raw);
}

export class SessionEncoder {
  readonly #inner: RuntimeSessionEncoder;

  constructor(inner: RuntimeSessionEncoder) {
    this.#inner = inner;
  }

  encode(value: RecurramValue): Uint8Array {
    return this.#inner.encodeCompactJson(serializeCompact(value));
  }

  encodeBatch(values: RecurramValue[]): Uint8Array {
    return this.#inner.encodeBatchCompactJson(serializeCompactBatch(values));
  }

  encodePatch(value: RecurramValue): Uint8Array {
    return this.#inner.encodePatchTransportJson(serializeValue(value));
  }

  encodeMicroBatch(values: RecurramValue[]): Uint8Array {
    return this.#inner.encodeMicroBatchCompactJson(
      serializeCompactBatch(values),
    );
  }

  reset(): void {
    this.#inner.reset();
  }
}
