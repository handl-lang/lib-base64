//! The `base64` codec — implemented here, not in `lib-encoding`, so the
//! contract crate stays free of concrete impls.

use handl_lib_encoding::{Encoding, EncodingError, EncodingRegistry};
use serde_json::Value;

/// Base64 (standard alphabet, padded) codec.
#[derive(Debug, Default, Clone, Copy)]
pub struct Base64;

impl Encoding for Base64 {
    fn encode(&self, value: &Value) -> Result<Value, EncodingError> {
        use base64::{engine::Engine as _, prelude::BASE64_STANDARD};
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("base64.encode expects a string".to_string()))?;
        Ok(Value::String(BASE64_STANDARD.encode(s.as_bytes())))
    }

    fn decode(&self, value: &Value) -> Result<Value, EncodingError> {
        use base64::{engine::Engine as _, prelude::BASE64_STANDARD};
        let s = value
            .as_str()
            .ok_or_else(|| EncodingError::Message("base64.decode expects a string".to_string()))?;
        let decoded = BASE64_STANDARD
            .decode(s)
            .map_err(|e| EncodingError::Message(format!("base64 decode error: {e}")))?;
        let out = String::from_utf8(decoded).map_err(|e| {
            EncodingError::Message(format!("base64 decode is not valid utf-8: {e}"))
        })?;
        Ok(Value::String(out))
    }
}

/// Register this codec under `"base64"`.
pub fn register(registry: EncodingRegistry) -> EncodingRegistry {
    registry.with("base64", Base64)
}
