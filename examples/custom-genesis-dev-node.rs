//! This example shows how to run a custom dev node programmatically
//! through rpc.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use reth::{
    args::DevArgs, builder::NodeBuilder, rpc::builder::{RethRpcModule, RpcModuleSelection}, tasks::TaskManager
};
use reth_node_core::{args::RpcServerArgs, node_config::NodeConfig};
use reth_primitives::DEV;
use reth_tracing::{RethTracer, Tracer};
use std::{net::{IpAddr, Ipv4Addr}, time::Duration};
use alphanet_node::node::AlphaNetNode;
use once_cell::sync::Lazy;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _guard = RethTracer::new().init()?;
    let tasks = TaskManager::current();

    let chain_spec = Lazy::force(&DEV).clone();

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
            http_api: Some(RpcModuleSelection::Selection([
                RethRpcModule::Debug, 
                RethRpcModule::Eth, 
                RethRpcModule::Net, 
                RethRpcModule::Trace, 
                RethRpcModule::Web3
            ].to_vec())),
            ..RpcServerArgs::default().with_http().with_ws()
        })
        .with_chain(chain_spec);

    let handle = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .node(AlphaNetNode::default())
        .launch()
        .await
        .unwrap();

    println!("Node started");

    handle.node_exit_future.await
}