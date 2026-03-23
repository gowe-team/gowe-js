export type RuntimeKind = "napi" | "wasm";

export interface RuntimeSessionEncoder {
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

export interface RuntimeBackend {
  kind: RuntimeKind;
  encodeTransportJson(valueJson: string): Uint8Array;
  decodeToTransportJson(bytes: Uint8Array): string;
  encodeWithSchemaTransportJson(
    schemaJson: string,
    valueJson: string,
  ): Uint8Array;
  encodeBatchTransportJson(valuesJson: string): Uint8Array;
  createSessionEncoder(optionsJson?: string): RuntimeSessionEncoder;
}
