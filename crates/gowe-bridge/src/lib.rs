use std::fmt;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use gowe::model::SchemaField;
use gowe::{
    create_session_encoder, decode, encode, encode_batch, encode_with_schema, GoweError, Schema,
    SessionEncoder, SessionOptions, UnknownReferencePolicy, Value,
};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, BridgeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeError {
    message: String,
}

impl BridgeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BridgeError {}

impl From<serde_json::Error> for BridgeError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("invalid json payload: {value}"))
    }
}

impl From<GoweError> for BridgeError {
    fn from(value: GoweError) -> Self {
        Self::new(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "v")]
enum TransportValue {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "i64")]
    I64(String),
    #[serde(rename = "u64")]
    U64(String),
    #[serde(rename = "f64")]
    F64(f64),
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "binary")]
    Binary(String),
    #[serde(rename = "array")]
    Array(Vec<TransportValue>),
    #[serde(rename = "map")]
    Map(Vec<(String, TransportValue)>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum U64Like {
    Number(u64),
    String(String),
}

impl U64Like {
    fn parse(self, field_name: &'static str) -> Result<u64> {
        match self {
            Self::Number(value) => Ok(value),
            Self::String(raw) => raw
                .parse::<u64>()
                .map_err(|_| BridgeError::new(format!("invalid {field_name}: expected u64"))),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum I64Like {
    Number(i64),
    String(String),
}

impl I64Like {
    fn parse(self, field_name: &'static str) -> Result<i64> {
        match self {
            Self::Number(value) => Ok(value),
            Self::String(raw) => raw
                .parse::<i64>()
                .map_err(|_| BridgeError::new(format!("invalid {field_name}: expected i64"))),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransportSchema {
    schema_id: U64Like,
    name: String,
    fields: Vec<TransportSchemaField>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransportSchemaField {
    number: U64Like,
    name: String,
    logical_type: String,
    required: bool,
    #[serde(default)]
    default_value: Option<TransportValue>,
    #[serde(default)]
    min: Option<I64Like>,
    #[serde(default)]
    max: Option<I64Like>,
    #[serde(default)]
    enum_values: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct TransportSessionOptions {
    #[serde(default)]
    max_base_snapshots: Option<usize>,
    #[serde(default)]
    enable_state_patch: Option<bool>,
    #[serde(default)]
    enable_template_batch: Option<bool>,
    #[serde(default)]
    enable_trained_dictionary: Option<bool>,
    #[serde(default)]
    unknown_reference_policy: Option<String>,
}

pub fn encode_transport_json(value_json: &str) -> Result<Vec<u8>> {
    let transport: TransportValue = serde_json::from_str(value_json)?;
    let value = transport_to_value(transport)?;
    encode(&value).map_err(Into::into)
}

pub fn decode_to_transport_json(bytes: &[u8]) -> Result<String> {
    let value = decode(bytes)?;
    let transport = value_to_transport(value);
    serde_json::to_string(&transport).map_err(Into::into)
}

pub fn encode_with_schema_transport_json(schema_json: &str, value_json: &str) -> Result<Vec<u8>> {
    let schema = parse_schema_json(schema_json)?;
    let transport: TransportValue = serde_json::from_str(value_json)?;
    let value = transport_to_value(transport)?;
    encode_with_schema(&schema, &value).map_err(Into::into)
}

pub fn encode_batch_transport_json(values_json: &str) -> Result<Vec<u8>> {
    let values = parse_transport_values_json(values_json)?;
    encode_batch(&values).map_err(Into::into)
}

pub struct BridgeSessionEncoder {
    inner: SessionEncoder,
}

impl BridgeSessionEncoder {
    pub fn new(options_json: Option<&str>) -> Result<Self> {
        let options = parse_session_options_json(options_json)?;
        Ok(Self {
            inner: create_session_encoder(options),
        })
    }

    pub fn encode_transport_json(&mut self, value_json: &str) -> Result<Vec<u8>> {
        let transport: TransportValue = serde_json::from_str(value_json)?;
        let value = transport_to_value(transport)?;
        self.inner.encode(&value).map_err(Into::into)
    }

    pub fn encode_with_schema_transport_json(
        &mut self,
        schema_json: &str,
        value_json: &str,
    ) -> Result<Vec<u8>> {
        let schema = parse_schema_json(schema_json)?;
        let transport: TransportValue = serde_json::from_str(value_json)?;
        let value = transport_to_value(transport)?;
        self.inner
            .encode_with_schema(&schema, &value)
            .map_err(Into::into)
    }

    pub fn encode_batch_transport_json(&mut self, values_json: &str) -> Result<Vec<u8>> {
        let values = parse_transport_values_json(values_json)?;
        self.inner.encode_batch(&values).map_err(Into::into)
    }

    pub fn encode_patch_transport_json(&mut self, value_json: &str) -> Result<Vec<u8>> {
        let transport: TransportValue = serde_json::from_str(value_json)?;
        let value = transport_to_value(transport)?;
        self.inner.encode_patch(&value).map_err(Into::into)
    }

    pub fn encode_micro_batch_transport_json(&mut self, values_json: &str) -> Result<Vec<u8>> {
        let values = parse_transport_values_json(values_json)?;
        self.inner.encode_micro_batch(&values).map_err(Into::into)
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

fn parse_schema_json(schema_json: &str) -> Result<Schema> {
    let schema: TransportSchema = serde_json::from_str(schema_json)?;
    transport_schema_to_schema(schema)
}

fn parse_session_options_json(options_json: Option<&str>) -> Result<SessionOptions> {
    let Some(raw) = options_json else {
        return Ok(SessionOptions::default());
    };
    let options: TransportSessionOptions = serde_json::from_str(raw)?;
    transport_session_options_to_options(options)
}

fn parse_transport_values_json(values_json: &str) -> Result<Vec<Value>> {
    let transports: Vec<TransportValue> = serde_json::from_str(values_json)?;
    transports.into_iter().map(transport_to_value).collect()
}

fn transport_schema_to_schema(schema: TransportSchema) -> Result<Schema> {
    let schema_id = schema.schema_id.parse("schemaId")?;
    let fields = schema
        .fields
        .into_iter()
        .map(|field| {
            Ok(SchemaField {
                number: field.number.parse("field.number")?,
                name: field.name,
                logical_type: field.logical_type,
                required: field.required,
                default_value: field.default_value.map(transport_to_value).transpose()?,
                min: field.min.map(|v| v.parse("field.min")).transpose()?,
                max: field.max.map(|v| v.parse("field.max")).transpose()?,
                enum_values: field.enum_values,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Schema {
        schema_id,
        name: schema.name,
        fields,
    })
}

fn transport_session_options_to_options(
    options: TransportSessionOptions,
) -> Result<SessionOptions> {
    let mut parsed = SessionOptions::default();
    if let Some(value) = options.max_base_snapshots {
        parsed.max_base_snapshots = value;
    }
    if let Some(value) = options.enable_state_patch {
        parsed.enable_state_patch = value;
    }
    if let Some(value) = options.enable_template_batch {
        parsed.enable_template_batch = value;
    }
    if let Some(value) = options.enable_trained_dictionary {
        parsed.enable_trained_dictionary = value;
    }
    if let Some(policy) = options.unknown_reference_policy {
        parsed.unknown_reference_policy = parse_unknown_reference_policy(&policy)?;
    }
    Ok(parsed)
}

fn parse_unknown_reference_policy(value: &str) -> Result<UnknownReferencePolicy> {
    match value {
        "failFast" | "fail_fast" | "FailFast" => Ok(UnknownReferencePolicy::FailFast),
        "statelessRetry" | "stateless_retry" | "StatelessRetry" => {
            Ok(UnknownReferencePolicy::StatelessRetry)
        }
        _ => Err(BridgeError::new(
            "unknownReferencePolicy must be failFast or statelessRetry",
        )),
    }
}

fn transport_to_value(value: TransportValue) -> Result<Value> {
    match value {
        TransportValue::Null => Ok(Value::Null),
        TransportValue::Bool(v) => Ok(Value::Bool(v)),
        TransportValue::I64(raw) => raw
            .parse::<i64>()
            .map(Value::I64)
            .map_err(|_| BridgeError::new("invalid i64 value")),
        TransportValue::U64(raw) => raw
            .parse::<u64>()
            .map(Value::U64)
            .map_err(|_| BridgeError::new("invalid u64 value")),
        TransportValue::F64(v) => Ok(Value::F64(v)),
        TransportValue::String(v) => Ok(Value::String(v)),
        TransportValue::Binary(raw) => BASE64
            .decode(raw)
            .map(Value::Binary)
            .map_err(|_| BridgeError::new("invalid base64 binary payload")),
        TransportValue::Array(values) => values
            .into_iter()
            .map(transport_to_value)
            .collect::<Result<Vec<_>>>()
            .map(Value::Array),
        TransportValue::Map(entries) => entries
            .into_iter()
            .map(|(k, v)| transport_to_value(v).map(|vv| (k, vv)))
            .collect::<Result<Vec<_>>>()
            .map(Value::Map),
    }
}

fn value_to_transport(value: Value) -> TransportValue {
    match value {
        Value::Null => TransportValue::Null,
        Value::Bool(v) => TransportValue::Bool(v),
        Value::I64(v) => TransportValue::I64(v.to_string()),
        Value::U64(v) => TransportValue::U64(v.to_string()),
        Value::F64(v) => TransportValue::F64(v),
        Value::String(v) => TransportValue::String(v),
        Value::Binary(v) => TransportValue::Binary(BASE64.encode(v)),
        Value::Array(values) => {
            TransportValue::Array(values.into_iter().map(value_to_transport).collect())
        }
        Value::Map(entries) => TransportValue::Map(
            entries
                .into_iter()
                .map(|(k, v)| (k, value_to_transport(v)))
                .collect(),
        ),
    }
}
