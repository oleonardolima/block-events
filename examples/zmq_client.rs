use anyhow::{self, Ok};
use bitcoin::{consensus, Block};
use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::SUB).unwrap();

    socket.connect("tcp://127.0.0.1:28332").unwrap();
    socket.set_subscribe("rawblock".as_bytes()).unwrap();

    loop {
        log::debug!("[loop]");
        socket.pin

        if let [topic, serialized_block, seq] = &socket.recv_multipart(0).unwrap()[..] {
            log::debug!("[topic][{:?}", std::str::from_utf8(&topic));
            let block: Block = consensus::deserialize(&serialized_block).unwrap();
            log::debug!("[block][{:#?}]", block);
            let mut arr = [0u8; 4];
            for (place, element) in arr.iter_mut().zip(seq.iter()) {
                *place = *element;
            }
            log::debug!("[sequence][{:#?}]", u32::from_be_bytes(arr));
        }
    }

    // Ok(())
}
