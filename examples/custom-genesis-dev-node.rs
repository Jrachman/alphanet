//! This example shows how to run a custom dev node programmatically
//! through rpc.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use reth::{
    args::DevArgs, builder::NodeBuilder, tasks::TaskManager
};
use reth_node_core::{args::RpcServerArgs, node_config::NodeConfig};
use reth_node_optimism::{args::RollupArgs, OptimismNode};
use reth_primitives::{ChainSpec, ForkCondition, ForkTimestamps, Genesis, GenesisAccount, Hardfork};
use reth_tracing::{RethTracer, Tracer};
use revm_primitives::{address, U256};
use std::{net::{IpAddr, Ipv4Addr}, sync::Arc, time::Duration};
use alphanet_node::node::AlphaNetNode;

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
        .with_rpc(RpcServerArgs {
            http_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ws_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            auth_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..RpcServerArgs::default().with_http().with_ws()
        })
        .with_chain(custom_chain());

    let handle = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .with_types(AlphaNetNode::default())
        .with_components(OptimismNode::components(RollupArgs::default()))
        .launch()
        .await
        .unwrap();

    println!("Node started");

    handle.node_exit_future.await
}

fn custom_chain() -> Arc<ChainSpec> {
    let custom_genesis = include_str!("../etc/alphanet-genesis.json");
    let mut genesis: Genesis = serde_json::from_str(custom_genesis).unwrap();
    genesis.alloc.insert(
        address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
        GenesisAccount {
            balance: U256::MAX,
            nonce: None,
            code: None,
            storage: None,
            private_key: None,
        },
    );
    let mut chainspec: ChainSpec = ChainSpec {
        fork_timestamps: ForkTimestamps::default()
            .shanghai(1699981200)
            .canyon(1699981200)
            .cancun(1708534800)
            .ecotone(1708534800),
        ..genesis.into()
    };
    chainspec.hardforks.insert(Hardfork::Bedrock, ForkCondition::Block(0));
    chainspec.hardforks.insert(Hardfork::Regolith, ForkCondition::Timestamp(0));
    chainspec.hardforks.insert(Hardfork::Shanghai, ForkCondition::Timestamp(1699981200));
    chainspec.hardforks.insert(Hardfork::Canyon, ForkCondition::Timestamp(1699981200));
    chainspec.hardforks.insert(Hardfork::Cancun, ForkCondition::Timestamp(1708534800));
    chainspec.hardforks.insert(Hardfork::Ecotone, ForkCondition::Timestamp(1708534800));

    Arc::new(chainspec)
}