//! Configuration types for EVM environment.

use core::fmt::Debug;

use alloy_evm::EvmEnv;
use alloy_primitives::U256;
use revm::{
    context::{BlockEnv, CfgEnv},
    primitives::hardfork::SpecId,
};

use crate::ZkSpecId;

/// Container type that holds both the configuration and block environment for EVM execution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ZKsyncEnv<Spec = ZkSpecId, BlockEnv = revm::context::BlockEnv> {
    pub inner: EvmEnv<Spec, BlockEnv>,
}

impl<Spec, BlockEnv> ZKsyncEnv<Spec, BlockEnv> {
    /// Create a new `EvmEnv` from its components.
    ///
    /// # Arguments
    ///
    /// * `cfg_env_with_handler_cfg` - The configuration environment with handler settings
    /// * `block` - The block environment containing block-specific data
    pub const fn new(inner: EvmEnv<Spec, BlockEnv>) -> Self {
        Self { inner }
    }
}

impl<Spec, BlockEnv: BlockEnvironment> ZKsyncEnv<Spec, BlockEnv> {
    /// Sets an extension on the environment.
    pub fn map_block_env<NewBlockEnv>(
        self,
        f: impl FnOnce(BlockEnv) -> NewBlockEnv,
    ) -> ZKsyncEnv<Spec, NewBlockEnv> {
        let EvmEnv { cfg_env, block_env } = self.inner;
        ZKsyncEnv {
            inner: EvmEnv {
                cfg_env,
                block_env: f(block_env),
            },
        }
    }

    /// Returns a reference to the block environment.
    pub const fn block_env(&self) -> &BlockEnv {
        &self.inner.block_env
    }

    /// Returns a reference to the configuration environment.
    pub const fn cfg_env(&self) -> &CfgEnv<Spec> {
        &self.inner.cfg_env
    }

    /// Returns the chain ID of the environment.
    pub const fn chainid(&self) -> u64 {
        self.inner.cfg_env.chain_id
    }

    /// Returns the spec id of the chain
    pub const fn spec_id(&self) -> &Spec {
        &self.inner.cfg_env.spec
    }

    /// Overrides the configured block number
    pub fn with_block_number(mut self, number: U256) -> Self {
        self.inner.block_env.inner_mut().number = number;
        self
    }

    /// Convenience function that overrides the configured block number with the given
    /// `Some(number)`.
    ///
    /// This is intended for block overrides.
    pub fn with_block_number_opt(mut self, number: Option<U256>) -> Self {
        if let Some(number) = number {
            self.inner.block_env.inner_mut().number = number;
        }
        self
    }

    /// Sets the block number if provided.
    pub fn set_block_number_opt(&mut self, number: Option<U256>) -> &mut Self {
        if let Some(number) = number {
            self.inner.block_env.inner_mut().number = number;
        }
        self
    }

    /// Overrides the configured block timestamp.
    pub fn with_timestamp(mut self, timestamp: U256) -> Self {
        self.inner.block_env.inner_mut().timestamp = timestamp;
        self
    }

    /// Convenience function that overrides the configured block timestamp with the given
    /// `Some(timestamp)`.
    ///
    /// This is intended for block overrides.
    pub fn with_timestamp_opt(mut self, timestamp: Option<U256>) -> Self {
        if let Some(timestamp) = timestamp {
            self.inner.block_env.inner_mut().timestamp = timestamp;
        }
        self
    }

    /// Sets the block timestamp if provided.
    pub fn set_timestamp_opt(&mut self, timestamp: Option<U256>) -> &mut Self {
        if let Some(timestamp) = timestamp {
            self.inner.block_env.inner_mut().timestamp = timestamp;
        }
        self
    }

    /// Overrides the configured block base fee.
    pub fn with_base_fee(mut self, base_fee: u64) -> Self {
        self.inner.block_env.inner_mut().basefee = base_fee;
        self
    }

    /// Convenience function that overrides the configured block base fee with the given
    /// `Some(base_fee)`.
    ///
    /// This is intended for block overrides.
    pub fn with_base_fee_opt(mut self, base_fee: Option<u64>) -> Self {
        if let Some(base_fee) = base_fee {
            self.inner.block_env.inner_mut().basefee = base_fee;
        }
        self
    }

    /// Sets the block base fee if provided.
    pub fn set_base_fee_opt(&mut self, base_fee: Option<u64>) -> &mut Self {
        if let Some(base_fee) = base_fee {
            self.inner.block_env.inner_mut().basefee = base_fee;
        }
        self
    }
}

impl<Spec, BlockEnv> From<(CfgEnv<Spec>, BlockEnv)> for ZKsyncEnv<Spec, BlockEnv> {
    fn from((cfg_env, block_env): (CfgEnv<Spec>, BlockEnv)) -> Self {
        Self {
            inner: EvmEnv { cfg_env, block_env },
        }
    }
}

/// Trait for types that can be used as a block environment.
///
/// Assumes that the type wraps an inner [`revm::context::BlockEnv`].
pub trait BlockEnvironment: revm::context::Block + Clone + Debug + Send + Sync + 'static {
    /// Returns a mutable reference to the inner [`revm::context::BlockEnv`].
    fn inner_mut(&mut self) -> &mut revm::context::BlockEnv;
}

impl BlockEnvironment for BlockEnv {
    fn inner_mut(&mut self) -> &mut revm::context::BlockEnv {
        self
    }
}
