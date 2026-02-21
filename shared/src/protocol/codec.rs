// Thin serde/postcard helpers for serializing command and telemetry payloads.
use serde::{de::DeserializeOwned, Serialize};

use crate::common::SharedError;

pub fn encode_to_slice<'a, T>(value: &T, out: &'a mut [u8]) -> Result<&'a mut [u8], SharedError>
where
	T: Serialize,
{
	postcard::to_slice(value, out).map_err(|_| SharedError::Serialization)
}

pub fn decode_from_bytes<T>(bytes: &[u8]) -> Result<T, SharedError>
where
	T: DeserializeOwned,
{
	postcard::from_bytes(bytes).map_err(|_| SharedError::Deserialization)
}
