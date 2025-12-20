use std::ops::Deref;

use bitcoin::{consensus::deserialize, Block};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let hash = "1a72d2a7d9c7f0894b8e83344ec5d73e0c58588a7bee95f8552feb39423bca9e";
    let url = format!("http://localhost:8999/api/v1/block/{}/raw", hash);
    // let url = format!("http://localhost:3000/block/{}/raw", hash);

    let res = reqwest::get(url).await?;
    log::debug!("{:#?}", res);

    // binary decoding
    let res_bytes = res.bytes().await?;
    log::debug!("{:#?}", res_bytes);
    let block: Block = deserialize(res_bytes.deref()).unwrap();
    log::debug!("{:#?}", block);
    // print!("{:#?}", block);

    // hex decoding
    // let res_text = res.text().await?;
    // log::debug!("{:#?}", res_text);
    // let block: Block = deserialize(&Vec::<u8>::from_hex(res_text.as_str()).unwrap()).unwrap();
    // // log::debug!("{:#?}", block);
    // print!("{:#?}", block);

    Ok(())
}
