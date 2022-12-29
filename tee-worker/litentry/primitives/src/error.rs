#[cfg(all(not(feature = "std"), feature = "sgx"))]
use thiserror_sgx as thiserror;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
	#[error("network type and handle type don't match")]
	InvalidParams,
	#[error("wrong identity network type or handle type")]
	InvalidIdentity,
	#[error("invalid identity string in handle")]
	InvalidHandleString,
}
