use bitcoin::Network;
use bitcoind::bitcoincore_rpc::RpcApi;
use block_events::{fetch_data_stream, get_default_websocket_address, BlockEvent};
use electrsd::bitcoind::BitcoinD;
use futures_util::{pin_mut, StreamExt};
use std::time::Duration;
use testcontainers::{clients, images, images::generic::GenericImage, RunnableImage};

const HOST_IP: &str = "127.0.0.1";

const MARIADB_NAME: &str = "mariadb";
const MARIADB_TAG: &str = "10.5.8";
const MARIADB_READY_CONDITION: &str = "mysqld: ready for connections.";

const MEMPOOL_BACKEND_NAME: &str = "mempool/backend";
const MEMPOOL_BACKEND_TAG: &str = "v2.4.0";
const MEMPOOL_BACKEND_READY_CONDITION: &str = "Mempool Server is running on port 8999";

// TODO: (@leonardo.lima) This should be derived instead, should we add it to bitcoind ?
const RPC_AUTH: &str = "mempool:3c417dbc7ccabb51d8e6fedc302288db$ed44e37a937e8706ea51bbc761df76e995fe92feff8751ce85feaea4c4ae80b1";

#[cfg(all(
    target_os = "macos",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn docker_host_address() -> &'static str {
    "host.docker.internal"
}

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_arch = "aarch64"))]
fn docker_host_address() -> &'static &str {
    "172.17.0.1"
}

pub struct MempoolTestClient {
    pub bitcoind: BitcoinD,
    pub mariadb_database: RunnableImage<GenericImage>,
    pub mempool_backend: RunnableImage<GenericImage>,
}

impl MempoolTestClient {
    fn start_bitcoind(bitcoind_exe: Option<String>) -> BitcoinD {
        let bitcoind_exe = bitcoind_exe.unwrap_or(bitcoind::downloaded_exe_path().ok().expect(
            "you should provide a bitcoind_exe parameter or specify a bitcoind version feature",
        ));

        log::debug!("launching bitcoind [bitcoind_exe {:?}]", bitcoind_exe);

        let mut conf = bitcoind::Conf::default();
        let rpc_auth = format!("-rpcauth={}", RPC_AUTH);
        conf.args.push(rpc_auth.as_str());

        let bitcoind = BitcoinD::with_conf(&bitcoind_exe, &conf).unwrap();

        log::debug!("successfully launched [bitcoind_exe {:?}]", bitcoind_exe);
        bitcoind
    }

    fn start_database(name: Option<&str>, tag: Option<&str>) -> RunnableImage<GenericImage> {
        let name = name.unwrap_or(MARIADB_NAME);
        let tag = tag.unwrap_or(MARIADB_TAG);

        log::debug!(
            "creating image and starting container [name {}] [tag {}]",
            name,
            tag
        );

        let image = images::generic::GenericImage::new(name, tag).with_wait_for(
            testcontainers::core::WaitFor::StdErrMessage {
                message: MARIADB_READY_CONDITION.to_string(),
            },
        );

        let image = RunnableImage::from(image)
            .with_env_var(("MYSQL_DATABASE", "mempool"))
            .with_env_var(("MYSQL_USER", "mempool"))
            .with_env_var(("MYSQL_PASSWORD", "mempool"))
            .with_env_var(("MYSQL_ROOT_PASSWORD", "mempool"))
            .with_mapped_port((3306, 3306));

        log::debug!(
            "successfully created and started container [name {}] [tag {}]",
            name,
            tag
        );
        image
    }

    fn start_backend(
        name: Option<&str>,
        tag: Option<&str>,
        core: &BitcoinD,
    ) -> RunnableImage<GenericImage> {
        let name = name.unwrap_or(MEMPOOL_BACKEND_NAME);
        let tag = tag.unwrap_or(MEMPOOL_BACKEND_TAG);

        log::debug!(
            "creating image and starting container [name {}] [tag {}]",
            name,
            tag
        );

        let image = images::generic::GenericImage::new(name, tag).with_wait_for(
            testcontainers::core::WaitFor::StdErrMessage {
                message: MEMPOOL_BACKEND_READY_CONDITION.to_string(),
            },
        );

        let bitcoind_port = core.params.rpc_socket.port().to_string();

        println!("{}", docker_host_address().to_string());

        let image = RunnableImage::from(image)
            .with_env_var(("MEMPOOL_BACKEND", "none"))
            .with_env_var(("MEMPOOL_NETWORK", "regtest"))
            .with_env_var(("DATABASE_HOST", docker_host_address().to_string()))
            .with_env_var(("CORE_RPC_HOST", docker_host_address().to_string()))
            .with_env_var(("CORE_RPC_PORT", bitcoind_port))
            .with_mapped_port((8999, 8999));

        log::debug!(
            "successfully created and started container [name {}] [tag {}]",
            name,
            tag
        );
        image
    }
}

impl Default for MempoolTestClient {
    fn default() -> Self {
        let bitcoind = Self::start_bitcoind(None);
        let mariadb = Self::start_database(None, None);
        let mempool = Self::start_backend(None, None, &bitcoind);

        MempoolTestClient {
            bitcoind: (bitcoind),
            mariadb_database: (mariadb),
            mempool_backend: (mempool),
        }
    }
}

#[test]
fn should_return_websocket_address() {
    let network = Network::Testnet;
    let address = get_default_websocket_address(&network);

    assert_eq!(address, "wss://mempool.space/testnet/api/v1/ws");
}

#[tokio::test]
async fn should_produce_stream_of_block_events() {
    let _ = env_logger::try_init();
    let delay = Duration::from_millis(5000);

    let docker = clients::Cli::docker();
    let client = MempoolTestClient::default();

    let _mariadb = docker.run(client.mariadb_database);
    std::thread::sleep(delay); // there is some delay between running the docker and the port being really available

    let mempool = docker.run(client.mempool_backend);

    let block_data = block_events::MempoolSpaceWebSocketRequestData::Blocks;
    let ws_url = url::Url::parse(
        format!(
            "ws://{}:{}/api/v1/ws",
            HOST_IP,
            mempool.get_host_port_ipv4(8999)
        )
        .as_str(),
    )
    .unwrap();

    // subscribe to websocket (mempool/backend docker container), generate `block_num` new blocks
    // check for all generate blocks being consumed and listened by websocket client

    // get block-events stream
    let block_events = fetch_data_stream(&ws_url, &block_data).await.unwrap();

    // generate new blocks through bitcoind rpc-client
    let rpc_client = &client.bitcoind.client;

    // generate `block_num` new blocks
    let block_num = 10;
    let generated_blocks = rpc_client
        .generate_to_address(block_num, &rpc_client.get_new_address(None, None).unwrap())
        .unwrap();

    // consume new blocks from block-events stream
    pin_mut!(block_events);

    for i in 0..=block_num {
        let block_hash = generated_blocks.get(i as usize);
        let block_event = block_events.next().await.unwrap();

        // should produce a BlockEvent::Connected result for each block event
        assert!(matches!(block_event, BlockEvent::Connected { .. }));

        // should parse the BlockEvent::Connected successfully
        let connected_block = match block_event {
            BlockEvent::Connected(block) => block,
            _ => unreachable!("This test is supposed to have only connected blocks, please check why it's generating disconnected and/or errors at the moment."),
        };

        assert_eq!(block_hash.unwrap().to_owned(), connected_block.id);
    }
}
