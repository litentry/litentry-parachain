use codec::Error as CodecError;
use sgx_types::error::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("{0}")]
	Codec(#[from] CodecError),
	#[error("Enclave Error: {0}")]
	Sgx(SgxStatus),
	#[error("Enclave Quote Error: {0}")]
	SgxQuote(Quote3Error),
	#[error("Error, other: {0}")]
	Other(Box<dyn std::error::Error + Sync + Send + 'static>),
}
