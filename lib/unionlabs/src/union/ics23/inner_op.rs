use macros::model;

use crate::{cosmos::ics23::hash_op::HashOp, ensure};

#[model(
    proto(raw(protos::cosmos::ics23::v1::InnerOp), into, from),
    ethabi(raw(InnerOpEthAbi), into, from)
)]
pub struct InnerOp {
    #[serde(with = "::serde_utils::hex_string")]
    #[debug(wrap = ::serde_utils::fmt::DebugAsHex)]
    pub prefix: Vec<u8>,
    #[serde(with = "::serde_utils::hex_string")]
    #[debug(wrap = ::serde_utils::fmt::DebugAsHex)]
    pub suffix: Vec<u8>,
}

#[cfg(feature = "ethabi")]
#[doc(hidden)]
#[derive(Debug, PartialEq, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
pub struct InnerOpEthAbi {
    pub prefix: ethers::types::Bytes,
    pub suffix: ethers::types::Bytes,
}

const EXPECTED_HASH_OP: HashOp = HashOp::Sha256;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TryFromInnerOpError {
    #[error("unable to decode cosmos::ics23::InnerOp")]
    Cosmos(#[from] crate::cosmos::ics23::inner_op::TryFromInnerOpError),
    #[error("hash must be {}, found {0}", EXPECTED_HASH_OP)]
    InvalidHash(HashOp),
}

impl TryFrom<protos::cosmos::ics23::v1::InnerOp> for InnerOp {
    type Error = TryFromInnerOpError;

    fn try_from(value: protos::cosmos::ics23::v1::InnerOp) -> Result<Self, TryFromInnerOpError> {
        let value = crate::cosmos::ics23::inner_op::InnerOp::try_from(value)?;

        ensure(
            value.hash == EXPECTED_HASH_OP,
            TryFromInnerOpError::InvalidHash(value.hash),
        )?;

        Ok(Self {
            prefix: value.prefix,
            suffix: value.suffix,
        })
    }
}

impl From<InnerOp> for protos::cosmos::ics23::v1::InnerOp {
    fn from(value: InnerOp) -> Self {
        crate::cosmos::ics23::inner_op::InnerOp {
            hash: EXPECTED_HASH_OP,
            prefix: value.prefix,
            suffix: value.suffix,
        }
        .into()
    }
}

#[cfg(feature = "ethabi")]
impl From<InnerOpEthAbi> for InnerOp {
    fn from(value: InnerOpEthAbi) -> Self {
        Self {
            prefix: value.prefix.to_vec(),
            suffix: value.suffix.to_vec(),
        }
    }
}

#[cfg(feature = "ethabi")]
impl From<InnerOp> for InnerOpEthAbi {
    fn from(value: InnerOp) -> Self {
        Self {
            prefix: value.prefix.into(),
            suffix: value.suffix.into(),
        }
    }
}
