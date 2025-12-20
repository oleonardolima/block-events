use bitcoin::hashes::hex::FromHex;
use bitcoin::Block;
use bitcoin::{consensus::deserialize, BlockHeader, Transaction, Txid};
use block_events::api::BlockExtended;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let hash = "0000000000000033323d04ee44e38a64aa0ed912263445aa1b4f3e516302cf20";
    let base_url = "https://blockstream.info/testnet";

    let res = reqwest::get(format!("{}/api/block/{}", base_url, hash))
        .await?
        .text()
        .await?;

    let block_ext: BlockExtended = serde_json::from_str(res.as_str())?;
    log::debug!("[block_ext] {:#?}", block_ext);
    println!("[block_ext] {:#?}", block_ext);

    let block_header: BlockHeader = BlockHeader::from(block_ext);
    log::debug!("[block_header] {:#?}", block_header);
    println!("[block_header] {:#?}", block_header);

    let res = reqwest::get(format!("{}/api/block/{}/txids", base_url, hash))
        .await?
        .text()
        .await?;

    let tx_ids: Vec<Txid> = serde_json::from_str(res.as_str())?;
    log::debug!("tx_ids: {:#?}", tx_ids);
    println!("[tx_ids] {:#?}", tx_ids);

    let mut txs: Vec<Transaction> = Vec::new();
    for id in tx_ids {
        log::debug!("[tx_id] {:#?}", id);

        let res = reqwest::get(format!("{}/api/tx/{}/hex", base_url, id)).await?;
        let tx_text = res.text().await?;
        log::debug!("[tx_text] {:#?}", tx_text);
        println!("[tx_text] {:#?}", tx_text);

        let tx: Transaction = deserialize(&Vec::<u8>::from_hex(tx_text.as_str()).unwrap()).unwrap();
        log::debug!("[tx] {:#?}", tx);
        println!("[tx] {:#?}", tx);

        txs.push(Transaction::from(tx));
        break;
    }
    log::debug!("[txs] {:#?}", txs);
    println!("[txs] {:#?}", txs);

    let block = Block {
        header: block_header,
        txdata: txs,
    };

    log::debug!("[block] {:#?}", block);
    println!("[block] {:#?}", block);

    Ok(())
}
