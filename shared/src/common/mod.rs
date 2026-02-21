// Shared foundational primitives used by protocol and domain modules.
pub mod math;
pub mod time;
pub mod units;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharedError {
	InvalidValue,
	Serialization,
	Deserialization,
}

pub type SharedResult<T> = core::result::Result<T, SharedError>;
