use gowe_bridge::{
    decode_to_transport_json, encode_batch_transport_json, encode_transport_json,
    encode_with_schema_transport_json, BridgeError, BridgeSessionEncoder,
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

fn to_napi_error(error: BridgeError) -> napi::Error {
    napi::Error::from_reason(error.to_string())
}

#[napi(js_name = "encodeTransportJson")]
pub fn encode_transport_json_napi(value_json: String) -> napi::Result<Buffer> {
    encode_transport_json(&value_json)
        .map(Buffer::from)
        .map_err(to_napi_error)
}

#[napi(js_name = "decodeToTransportJson")]
pub fn decode_to_transport_json_napi(bytes: Buffer) -> napi::Result<String> {
    decode_to_transport_json(bytes.as_ref()).map_err(to_napi_error)
}

#[napi(js_name = "encodeWithSchemaTransportJson")]
pub fn encode_with_schema_transport_json_napi(
    schema_json: String,
    value_json: String,
) -> napi::Result<Buffer> {
    encode_with_schema_transport_json(&schema_json, &value_json)
        .map(Buffer::from)
        .map_err(to_napi_error)
}

#[napi(js_name = "encodeBatchTransportJson")]
pub fn encode_batch_transport_json_napi(values_json: String) -> napi::Result<Buffer> {
    encode_batch_transport_json(&values_json)
        .map(Buffer::from)
        .map_err(to_napi_error)
}

#[napi]
pub struct SessionEncoder {
    inner: BridgeSessionEncoder,
}

#[napi]
impl SessionEncoder {
    #[napi(constructor)]
    pub fn new(options_json: Option<String>) -> napi::Result<Self> {
        let inner = BridgeSessionEncoder::new(options_json.as_deref()).map_err(to_napi_error)?;
        Ok(Self { inner })
    }

    #[napi(js_name = "encodeTransportJson")]
    pub fn encode_transport_json(&mut self, value_json: String) -> napi::Result<Buffer> {
        self.inner
            .encode_transport_json(&value_json)
            .map(Buffer::from)
            .map_err(to_napi_error)
    }

    #[napi(js_name = "encodeWithSchemaTransportJson")]
    pub fn encode_with_schema_transport_json(
        &mut self,
        schema_json: String,
        value_json: String,
    ) -> napi::Result<Buffer> {
        self.inner
            .encode_with_schema_transport_json(&schema_json, &value_json)
            .map(Buffer::from)
            .map_err(to_napi_error)
    }

    #[napi(js_name = "encodeBatchTransportJson")]
    pub fn encode_batch_transport_json(&mut self, values_json: String) -> napi::Result<Buffer> {
        self.inner
            .encode_batch_transport_json(&values_json)
            .map(Buffer::from)
            .map_err(to_napi_error)
    }

    #[napi(js_name = "encodePatchTransportJson")]
    pub fn encode_patch_transport_json(&mut self, value_json: String) -> napi::Result<Buffer> {
        self.inner
            .encode_patch_transport_json(&value_json)
            .map(Buffer::from)
            .map_err(to_napi_error)
    }

    #[napi(js_name = "encodeMicroBatchTransportJson")]
    pub fn encode_micro_batch_transport_json(
        &mut self,
        values_json: String,
    ) -> napi::Result<Buffer> {
        self.inner
            .encode_micro_batch_transport_json(&values_json)
            .map(Buffer::from)
            .map_err(to_napi_error)
    }

    #[napi]
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

#[napi(js_name = "createSessionEncoder")]
pub fn create_session_encoder(options_json: Option<String>) -> napi::Result<SessionEncoder> {
    SessionEncoder::new(options_json)
}
