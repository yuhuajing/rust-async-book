use async_std::task;
use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use futures::{ try_join, AsyncWriteExt};
use futures::future::join_all;
use std::string::String;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const WETH_ADDRESS: &str = "0xff2B4721F997c242fF406a626f17df083Bd2C568";
const WSS_URL: &str = "wss://eth.getblock.io/ab0b1aa0-b490-4dc0-9bda-817c897a4580/mainnet";

async fn get_ws_client() -> Provider<Ws> {
    Provider::<Ws>::connect(WSS_URL).await.unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Arc::new(get_ws_client().await);


    //getlatestblonumber(client.clone()).await?;
    //getbalance(client.clone()).await?;

    let latest_blocknumber = getlatestblonumber(client.clone());
    let addr_balance= getbalance(client.clone());
    try_join!(latest_blocknumber, addr_balance); //  顺序执行，等待12s后才会执行
    Ok(())
}

async fn getbalance(client: Arc<Provider<Ws>>) -> Result<()> {
    let from_addr: &str = "0xc175006ED9Ee10210f466a043a300789a83C7420";
    //none is the latest blocknumber

    let balance = client.get_balance(from_addr, None).await?;
    println!("{balance}");
    Ok(())
}

async fn getlatestblonumber(client: Arc<Provider<Ws>>) -> Result<()> {
    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    thread::sleep(Duration::from_secs(12));
    println!("last_block: {last_block}");
    Ok(())
}
