mod core;
mod crypto;
mod accounts;
mod p2p;
mod internal;
mod rpc;
mod miner;
mod pow;
mod event;
mod logger;
mod metrics;
mod trie;

use crate::core::blockchain::Blockchain;
use crate::rpc::server::run_rpc_server;
use crate::logger::log::init_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    println!("Starting PureChain Node...");

    // Initialize blockchain with a chosen difficulty (e.g., 4)
    let blockchain = Blockchain::new(4);

    // For Ethereum deployability, integrate an EVM runtime or bridge here.
    // (For example, using Substrate’s EVM pallet or a custom WASM runtime.)

    // Start the RPC server on port 8080
    run_rpc_server(8080, blockchain).await
}
