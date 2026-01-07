//! Contains the `[ZKsyncTxError]` type.
use alloy_evm::InvalidTxError;
use core::fmt::Display;
use revm::context_interface::{
    result::{EVMError, InvalidTransaction},
    transaction::TransactionError,
};

/// ZKsync OS transaction validation error.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ZKsyncTxError {
    /// Base transaction error.
    Base(InvalidTransaction),
}

impl TransactionError for ZKsyncTxError {}

impl Display for ZKsyncTxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Base(error) => error.fmt(f),
        }
    }
}

impl core::error::Error for ZKsyncTxError {}

impl From<InvalidTransaction> for ZKsyncTxError {
    fn from(value: InvalidTransaction) -> Self {
        Self::Base(value)
    }
}

impl<DBError> From<ZKsyncTxError> for EVMError<DBError, ZKsyncTxError> {
    fn from(value: ZKsyncTxError) -> Self {
        Self::Transaction(value)
    }
}

impl InvalidTxError for ZKsyncTxError {
    fn is_nonce_too_low(&self) -> bool {
        matches!(self, Self::Base(tx) if tx.is_nonce_too_low())
    }

    fn as_invalid_tx_err(&self) -> Option<&InvalidTransaction> {
        #[allow(unreachable_patterns)]
        match self {
            Self::Base(tx) => Some(tx),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_display_zk_errors() {
        assert_eq!(
            ZKsyncTxError::Base(InvalidTransaction::NonceTooHigh { tx: 2, state: 1 }).to_string(),
            "nonce 2 too high, expected 1"
        );
    }
}
