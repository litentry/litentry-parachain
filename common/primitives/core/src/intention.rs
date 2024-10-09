use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{H160};
use sp_runtime::{traits::ConstU32, BoundedVec};

pub const CALL_ETHEREUM_INPUT_LEN: u32 = 10 * 1024;

//todo: this should be versioned ?
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum Intention {
    #[codec(index = 0)]
    TransferEthereum(TransferEthereum),
    #[codec(index = 1)]
    CallEthereum(CallEthereum),
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct TransferEthereum {
    pub to: H160,
    pub value: [u8; 32],
}

pub type CallEthereumInputLen = ConstU32<CALL_ETHEREUM_INPUT_LEN>;

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct CallEthereum {
    pub address: H160,
    pub input: BoundedVec<u8, CallEthereumInputLen>,
}
