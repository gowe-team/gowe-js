import type { RuntimeBackend, RuntimeSessionEncoder } from "./types.js";

interface WasmSessionEncoder {
  encodeTransportJson(valueJson: string): Uint8Array;
  encodeWithSchemaTransportJson(
    schemaJson: string,
    valueJson: string,
  ): Uint8Array;
  encodeBatchTransportJson(valuesJson: string): Uint8Array;
  encodePatchTransportJson(valueJson: string): Uint8Array;
  encodeMicroBatchTransportJson(valuesJson: string): Uint8Array;
  reset(): void;
}

interface WasmModule {
  default: (input?: unknown) => Promise<unknown>;
  encodeTransportJson(valueJson: string): Uint8Array;
  decodeToTransportJson(bytes: Uint8Array): string;
  encodeWithSchemaTransportJson(
    schemaJson: string,
    valueJson: string,
  ): Uint8Array;
  encodeBatchTransportJson(valuesJson: string): Uint8Array;
  createSessionEncoder(optionsJson?: string): WasmSessionEncoder;
}

export async function loadWasmBackend(
  wasmInput?: unknown,
): Promise<RuntimeBackend> {
  const moduleUrl = new URL("../../wasm/pkg/gowe_wasm.js", import.meta.url);
  const wasm = (await import(moduleUrl.href)) as WasmModule;
  await wasm.default(wasmInput);
  return {
    kind: "wasm",
    encodeTransportJson: (valueJson) => wasm.encodeTransportJson(valueJson),
    decodeToTransportJson: (bytes) => wasm.decodeToTransportJson(bytes),
    encodeWithSchemaTransportJson: (schemaJson, valueJson) =>
      wasm.encodeWithSchemaTransportJson(schemaJson, valueJson),
    encodeBatchTransportJson: (valuesJson) =>
      wasm.encodeBatchTransportJson(valuesJson),
    createSessionEncoder: (optionsJson) => {
      const inner = wasm.createSessionEncoder(optionsJson);
      return wrapSessionEncoder(inner);
    },
  };
}

function wrapSessionEncoder(inner: WasmSessionEncoder): RuntimeSessionEncoder {
  return {
    encodeTransportJson: (valueJson) => inner.encodeTransportJson(valueJson),
    encodeWithSchemaTransportJson: (schemaJson, valueJson) =>
      inner.encodeWithSchemaTransportJson(schemaJson, valueJson),
    encodeBatchTransportJson: (valuesJson) =>
      inner.encodeBatchTransportJson(valuesJson),
    encodePatchTransportJson: (valueJson) =>
      inner.encodePatchTransportJson(valueJson),
    encodeMicroBatchTransportJson: (valuesJson) =>
      inner.encodeMicroBatchTransportJson(valuesJson),
    reset: () => inner.reset(),
  };
}
