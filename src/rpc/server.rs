use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use crate::core::blockchain::{Blockchain, Block};
use crate::accounts::transactions::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

pub struct RpcState {
    pub blockchain: Mutex<Blockchain>,
}

#[derive(Serialize)]
pub struct ChainResponse {
    pub chain: Vec<Block>,
    pub length: usize,
}

async fn add_transaction(state: web::Data<RpcState>, tx: web::Json<TransactionRequest>) -> impl Responder {
    let transaction = Transaction::new(tx.from.clone(), tx.to.clone(), tx.amount, tx.signature.clone());
    // (For brevity, the transaction is not added to a mempool here.)
    HttpResponse::Ok().json(transaction)
}

async fn get_chain(state: web::Data<RpcState>) -> impl Responder {
    let blockchain = state.blockchain.lock().unwrap();
    let response = ChainResponse {
        chain: blockchain.chain.clone(),
        length: blockchain.chain.len(),
    };
    HttpResponse::Ok().json(response)
}

pub async fn run_rpc_server(port: u16, blockchain: Blockchain) -> std::io::Result<()> {
    let state = web::Data::new(RpcState { blockchain: Mutex::new(blockchain) });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/chain", web::get().to(get_chain))
            .route("/transaction", web::post().to(add_transaction))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
