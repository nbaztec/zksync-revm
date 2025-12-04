//! Contains the `[ZkSpecId]` type and its implementation.
use core::str::FromStr;
use revm::{
    context::CfgEnv,
    primitives::hardfork::{SpecId, UnknownHardfork},
};

/// ZKsync OS spec id.
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[allow(non_camel_case_types)]
pub enum ZkSpecId {
    #[default]
    Atlas,
}

impl ZkSpecId {
    /// Converts the [`ZkSpecId`] into a [`SpecId`].
    pub const fn into_eth_spec(self) -> SpecId {
        match self {
            Self::Atlas => SpecId::CANCUN,
        }
    }

    /// Checks if the [`ZkSpecId`] is enabled in the other [`ZkSpecId`].
    pub const fn is_enabled_in(self, other: ZkSpecId) -> bool {
        other as u8 <= self as u8
    }
}

impl From<ZkSpecId> for SpecId {
    fn from(spec: ZkSpecId) -> Self {
        spec.into_eth_spec()
    }
}

impl FromStr for ZkSpecId {
    type Err = UnknownHardfork;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            name::ATLAS => Ok(ZkSpecId::Atlas),
            _ => Err(UnknownHardfork),
        }
    }
}

impl From<ZkSpecId> for &'static str {
    fn from(spec_id: ZkSpecId) -> Self {
        match spec_id {
            ZkSpecId::Atlas => name::ATLAS,
        }
    }
}

impl core::fmt::Display for ZkSpecId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", <&'static str>::from(*self))
    }
}

/// String identifiers for ZKsync OS hardforks
pub mod name {
    /// Initial spec name.
    pub const ATLAS: &str = "Atlas";
}

pub trait IntoZkSpecId {
    fn into_zk_spec_id(self) -> ZkSpecId;
}

impl IntoZkSpecId for SpecId {
    fn into_zk_spec_id(self) -> ZkSpecId {
        match self {
            SpecId::FRONTIER
            | SpecId::FRONTIER_THAWING
            | SpecId::HOMESTEAD
            | SpecId::DAO_FORK
            | SpecId::TANGERINE
            | SpecId::SPURIOUS_DRAGON
            | SpecId::BYZANTIUM
            | SpecId::CONSTANTINOPLE
            | SpecId::PETERSBURG
            | SpecId::ISTANBUL
            | SpecId::MUIR_GLACIER
            | SpecId::BERLIN
            | SpecId::LONDON
            | SpecId::ARROW_GLACIER
            | SpecId::GRAY_GLACIER
            | SpecId::MERGE
            | SpecId::SHANGHAI
            | SpecId::CANCUN
            | SpecId::PRAGUE
            | SpecId::OSAKA
            | SpecId::AMSTERDAM => ZkSpecId::Atlas,
        }
    }
}

pub trait ToZKsyncCfgEnv {
    fn to_zk_cfg_env(&self) -> CfgEnv<ZkSpecId>;
}

impl ToZKsyncCfgEnv for CfgEnv<SpecId> {
    fn to_zk_cfg_env(&self) -> CfgEnv<ZkSpecId> {
        let mut cfg = CfgEnv::<ZkSpecId>::new_with_spec(self.spec.into_zk_spec_id());

        cfg.chain_id = self.chain_id;
        cfg.tx_chain_id_check = self.tx_chain_id_check;
        cfg.limit_contract_code_size = self.limit_contract_code_size;
        cfg.limit_contract_initcode_size = self.limit_contract_initcode_size;
        cfg.disable_nonce_check = self.disable_nonce_check;
        cfg.max_blobs_per_tx = self.max_blobs_per_tx;
        cfg.blob_base_fee_update_fraction = self.blob_base_fee_update_fraction;
        cfg.tx_gas_limit_cap = self.tx_gas_limit_cap;
        cfg.disable_balance_check = self.disable_balance_check;

        cfg
    }
}
