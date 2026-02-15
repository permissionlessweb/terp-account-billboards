pub mod suite;
pub use suite::BtsgAccountSuite;
pub mod networks;

// re-export contract cw-orch functions & entrypoint types
pub use terp721_account::msg::{
    AsyncTerp721AccountsQueryMsgFns, ExecuteMsg as Bs721AccountExecuteMsgTypes,
    ExecuteMsgFns as BtsgAccountExecuteFns, Terp721AccountsQueryMsg, Terp721AccountsQueryMsgFns,
};
pub use terp_account::manifold::{
    AsyncQueryMsgFns as TerpAccountMinterAsyncQueryMsgFns,
    ExecuteMsg as Bs721AccountMinterExecuteMsgTypes, ExecuteMsgFns as BtsgAccountMinterExecuteFns,
    QueryMsg as BtsgAccountMinterQueryMsgTypes, QueryMsgFns as BtsgAccountMinterQueryMsgFns,
};
// pub use cw721_base::msg::{
//     AsyncQueryMsgFns as TerpAccountMinterAsyncQueryMsgFn, ExecuteMsgFns as Btsg721BaseExecuteFns,
//     QueryMsg as BtsgAccountQueryMsgTypes, QueryMsgFns as TerpAccountQueryMsgFns,
// };
pub use terp_account::manifold::{
    AsyncQueryMsgFns as BtsgAccountMarketAsyncQueryMsgFns,
    ExecuteMsg as Bs721AccountMarketExecuteMsgTypes, ExecuteMsgFns as BtsgAccountMarketExecuteFns,
    QueryMsg as BtsgAccountMarketQueryMsgTypes, QueryMsgFns as BtsgAccountMarketQueryFns,
};

// pub use account_registry_middleware::{
//     AsyncQueryMsgFns as AccountRegistryAsyncQueryMsgFns,
//     ExecuteMsg as AccountRegistryExecuteMsgTypes, ExecuteMsgFns as AccountRegistryExecuteFns,
//     QueryMsg as AccountRegistryQueryMsgTypes, QueryMsgFns as AccountRegistryQueryFns,
// };

pub use ownership_verifier::{
    ExecuteMsg as TestOwnershipExecuteMsg, ExecuteMsgFns as TestOwnershipExecuteMsgFns,
    InstantiateMsg as TestOwnershipInitMsg, QueryMsg as TestOwnershipQueryMsg,
    QueryMsgFns as TestOwnershipQueryMsgFns,
};

#[cfg(test)]
pub mod test;
