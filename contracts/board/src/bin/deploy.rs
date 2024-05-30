//! Deploys Abstract and the Adapter module to a local Junod instance. See how to spin up a local chain here: <https://docs.junonetwork.io/developer-guides/junod-local-dev-setup>
//! You can also start a juno container by running `just juno-local`.
//!
//! Ensure the local juno is running before executing this script.
//! Also make sure port 9090 is exposed on the local juno container. This port is used to communicate with the chain.
//!
//! # Run
//!
//! `RUST_LOG=info cargo run --bin local_daemon --features="daemon-bin" --package my-adapter`
use board::{contract::interface::BoardInterface, BoardExecuteMsg, BoardInstantiateMsg, BOARD_ID};

use abstract_adapter::{objects::namespace::Namespace, std::adapter::AdapterRequestMsg};
use abstract_client::{AbstractClient, Publisher};
use common::{board::BoardQueryMsgFns, config::board_chains_instantiate_msgs};
use cw_orch::{anyhow, prelude::*, tokio::runtime::Runtime};
use networks::parse_network;

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let runtime = Runtime::new()?;

    let chain_messages = board_chains_instantiate_msgs();
    for (chain_info, board_instantiate_msg) in chain_messages {
        let network = parse_network(&chain_info).unwrap();
        // add logs for starting this chain
        println!("[CHAIN: {}] Start Board deployment...", network.chain_id);

        let daemon = Daemon::builder()
            .chain(network.clone())
            .mnemonic(LOCAL_MNEMONIC) // TODO: Replace this by env one that will own the publisher and the namespace
            .handle(runtime.handle())
            .build()
            .unwrap();

        let adapter_namespace = Namespace::from_id(BOARD_ID)?;

        // Create an [`AbstractClient`]
        // Note: AbstractClient Builder used because Abstract is not yet deployed on the chain
        let abstract_client: AbstractClient<Daemon> =
            AbstractClient::builder(daemon.clone()).build()?;
        let publisher: Publisher<_> = abstract_client
            .publisher_builder(adapter_namespace.clone())
            .build()?;

        // Ensure the current sender owns the namespace
        assert_eq!(
            publisher.account().owner()?,
            daemon.sender(),
            "The current sender can not publish to this namespace. Please use the wallet that owns the Account that owns the Namespace."
        );

        println!("[CHAIN: {}] publisher done", network.chain_id);

        // Publish the Adapter to the Abstract Platform
        publisher.publish_adapter::<BoardInstantiateMsg, BoardInterface<Daemon>>(
            board_instantiate_msg.clone(),
        )?;

        // Install the Adapter on a new account
        let account = abstract_client.account_builder().build()?;
        // Installs the adapter on the Account
        let adapter = account.install_adapter::<BoardInterface<_>>(&[])?;

        // // Import adapter's endpoint function traits for easy interactions.
        let status_response = adapter.status(adapter.account().id()?)?;
        assert!(status_response.status.is_none());

        // Execute the Adapter
        adapter.execute(
            &AdapterRequestMsg {
                // Adapter need to know on which account action is performed
                proxy_address: Some(adapter.account().proxy()?.to_string()),
                request: BoardExecuteMsg::SetStatus {
                    status: "new_status".to_owned(),
                },
            }
            .into(),
            None,
        )?;

        // Query the Adapter again
        let status_response = adapter.status(adapter.account().id()?)?;
        assert_eq!(status_response.status, Some("new_status".to_owned()));

        // Note: the Adapter is installed on a sub-account of the main account!
        assert_ne!(account.id()?, adapter.account().id()?);
    }

    Ok(())
}
