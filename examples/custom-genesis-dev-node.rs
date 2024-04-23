//! This example shows how to run a custom dev node programmatically and submit a transaction
//! through rpc.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use futures_util::StreamExt;
use reth::{
    args::DevArgs, builder::{NodeBuilder, NodeHandle}, providers::CanonStateSubscriptions, rpc::eth::EthTransactions, tasks::TaskManager
};
use reth_node_core::{args::RpcServerArgs, node_config::NodeConfig};
use reth_node_ethereum::EthereumNode;
use reth_primitives::{b256, hex, ChainSpec, Genesis};
use reth_tracing::{RethTracer, Tracer};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _guard = RethTracer::new().init()?;
    
    let tasks = TaskManager::current();

    // create node config
    let node_config = NodeConfig::test()
        .with_dev(DevArgs {
            dev: true, 
            block_max_transactions: None,
            block_time: Some(Duration::new(3, 0))
        })
        .with_rpc(RpcServerArgs::default().with_http())
        .with_chain(custom_chain());

    let handle = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .node(EthereumNode::default())
        .launch()
        .await
        .unwrap();

    println!("Node started");

    handle.node_exit_future.await
}

fn custom_chain() -> Arc<ChainSpec> {
    let custom_genesis = r#"
{
    "nonce": "0x42",
    "timestamp": "0x0",
    "extraData": "0x5343",
    "gasLimit": "0x1388",
    "difficulty": "0x400000000",
    "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "coinbase": "0x0000000000000000000000000000000000000000",
    "alloc": {
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
            "balance": "0x4a47e3c12448f4ad000000"
        }
    },
    "number": "0x0",
    "gasUsed": "0x0",
    "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "config": {
        "ethash": {},
        "chainId": 2600,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0,
        "byzantiumBlock": 0,
        "constantinopleBlock": 0,
        "petersburgBlock": 0,
        "istanbulBlock": 0,
        "berlinBlock": 0,
        "londonBlock": 0,
        "terminalTotalDifficulty": 0,
        "terminalTotalDifficultyPassed": true,
        "shanghaiTime": 0
    }
}
"#;
    let genesis: Genesis = serde_json::from_str(custom_genesis).unwrap();
    Arc::new(genesis.into())
}