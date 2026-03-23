export type GoweValue =
  | null
  | boolean
  | number
  | bigint
  | string
  | Uint8Array
  | GoweValue[]
  | { [key: string]: GoweValue };

export interface SchemaField {
  number: number | bigint;
  name: string;
  logicalType: string;
  required: boolean;
  defaultValue?: GoweValue;
  min?: number | bigint;
  max?: number | bigint;
  enumValues?: string[];
}

export interface Schema {
  schemaId: number | bigint;
  name: string;
  fields: SchemaField[];
}

export type UnknownReferencePolicy = "failFast" | "statelessRetry";

export interface SessionOptions {
  maxBaseSnapshots?: number;
  enableStatePatch?: boolean;
  enableTemplateBatch?: boolean;
  enableTrainedDictionary?: boolean;
  unknownReferencePolicy?: UnknownReferencePolicy;
}

export interface InitOptions {
  prefer?: "napi" | "wasm";
  wasmInput?: unknown;
}
