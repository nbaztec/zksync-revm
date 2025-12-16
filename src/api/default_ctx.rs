//! Contains trait [`DefaultZk`] used to create a default context.
use crate::{ZKsyncTx, ZkSpecId};
use revm::{
    Context, Database, Journal, MainContext,
    context::{BlockEnv, CfgEnv, TxEnv},
    database_interface::EmptyDB,
};

/// Type alias for the default context type of the ZKsyncEvm.
pub type ZkContext<DB, J = Journal<DB>, C = ()> =
    Context<BlockEnv, ZKsyncTx<TxEnv>, CfgEnv<ZkSpecId>, DB, J, C>;
// pub type ZkContext<DB, TX, CFG, DB> = Context<
//     BLOCK = BlockEnv,
//     TX = TxEnv,
//     CFG = CfgEnv,
//     DB= EmptyDB,
// >;

/// Trait that allows for a default context to be created.
pub trait DefaultZk {
    /// Create a default context.
    fn default() -> ZkContext<EmptyDB>;
}

impl DefaultZk for ZkContext<EmptyDB> {
    fn default() -> Self {
        Context::mainnet()
            .with_tx(ZKsyncTx::builder().build_fill())
            .with_cfg(CfgEnv::new_with_spec(ZkSpecId::Atlas))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::builder::ZkBuilder;
    use revm::{
        ExecuteEvm,
        inspector::{InspectEvm, NoOpInspector},
    };

    #[test]
    fn default_run_zk() {
        let ctx = Context::default();
        // convert to ZKsync OS context
        let mut evm = ctx.build_zk_with_inspector(NoOpInspector {});
        // execute
        let _ = evm.transact(ZKsyncTx::builder().build_fill());
        // inspect
        let _ = evm.inspect_one_tx(ZKsyncTx::builder().build_fill());
    }
}
