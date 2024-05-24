use std::str::FromStr;

use anyhow::Result;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use tokio_stream::StreamExt;

const RAYDIUM_AMM_PROGRAM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RAYDIUM_LIQUIDITY_POOL: &str = "GH8Ers4yzKR3UKDvgVu8cqJfGzU4cU62mTeg9bcJ7ug6";
const INITIALIZE_2: &str = "initialize2";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let rpc_ws_url = dotenv::var("RPC_WS_URL").expect("'RPC_WS_URL' must be set");

    let ws_client = PubsubClient::new(rpc_ws_url.as_str()).await?;
    let (mut stream, _) = ws_client
        .account_subscribe(
            &Pubkey::from_str(RAYDIUM_LIQUIDITY_POOL).unwrap(),
            Some(RpcAccountInfoConfig::default()),
        )
        .await?;

    tracing::info!("successfully subscribed to rpc logs.");

    while let Some(response) = stream.next().await {
        // dbg!(response.clone());

        // // Filter out responses containing errors.
        // if response.value.err.is_some() {
        //     // tracing::error!("error: {:?}", response.value.err);
        //     continue;
        // }

        dbg!(response.clone());
        // process_rpc_logs_response(response).await;
    }

    Ok(())
}

// fn filter_err_response(response: Response<RpcLogsResponse>) {
//     if response.value.err.is_some() {
//         tracing::error!("error: {:?}", response.value.err);
//         return;
//     }
// }

// async fn process_rpc_logs_response(response: Response<RpcLogsResponse>) {
//     let rpc_logs_response = response.value;
//     // dbg!(rpc_logs_response.logs.len());
//     for log in rpc_logs_response.logs {
//         if !log.contains(INITIALIZE_2) {
//             continue;
//         }
//         tracing::info!("log: {}", log);
//         let signature_str = &rpc_logs_response.signature;
//         tracing::info!("signature: {}", signature_str);
//     }
// }
