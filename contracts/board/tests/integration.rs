use abstract_interchain_tests::setup::ibc_connect_abstract;
use board::{
    ActionType, BoardInstantiateMsg, BoardInterface, BoardQueryMsgFns, RequiredAction, TileAction,
    RUGS_N_CANDLES_NAMESPACE,
};
use common::controller::{ControllerExecuteMsgFns, ControllerInstantiateMsg};
use cosmwasm_std::Api;

use abstract_adapter::std::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use controller::ControllerInterface;
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};
use cw_orch_interchain::prelude::*;

struct TestEnv<Env: CwEnv> {
    controller: Application<Env, ControllerInterface<Env>>,
    board: Application<Env, BoardInterface<Env>>,
    interchain: MockBech32InterchainEnv,
}

impl TestEnv<MockBech32> {
    /// Set up the test environment with an Account that has the Adapter installed
    #[allow(clippy::type_complexity)]
    fn setup(
        chain: String,
        tiles_actions: Vec<(u32, TileAction)>,
    ) -> anyhow::Result<TestEnv<MockBech32>> {
        // Create mock chains
        let interchain =
            MockBech32InterchainEnv::new(vec![("juno-1", "juno"), ("kujira-1", "kuji")]);
        let neutron = interchain.chain("juno-1")?;
        let kujira = interchain.chain("kujira-1")?;

        let namespace = Namespace::new(RUGS_N_CANDLES_NAMESPACE)?;

        // You can set up Abstract with a builder.
        let neturn_abs_client = AbstractClient::builder(neutron.clone()).build()?;
        let kujira_abs_client = AbstractClient::builder(kujira).build()?;

        // The adapter supports setting balances for addresses and configuring ANS.

        // Publish the adapter
        let publisher = neturn_abs_client
            .publisher_builder(namespace.clone())
            .build()?;
        publisher.publish_adapter::<ControllerInstantiateMsg, ControllerInterface<_>>(
            ControllerInstantiateMsg {},
        )?;

        let neutron_controller = publisher
            .account()
            .install_adapter::<ControllerInterface<_>>(&[])?;

        let publisher = kujira_abs_client.publisher_builder(namespace).build()?;
        publisher.publish_adapter::<BoardInstantiateMsg, BoardInterface<_>>(
            BoardInstantiateMsg {
                chain,
                tiles_actions,
                tiles_number: 10,
            },
        )?;

        let kuji_board = publisher
            .account()
            .install_adapter::<BoardInterface<_>>(&[])?;

        ibc_connect_abstract(&interchain, "juno-1", "kujira-1")?;
        ibc_connect_abstract(&interchain, "kujira-1", "juno-1")?;

        println!("Installation of Controller completed");
        let sender = neutron_controller.get_chain().addr_make("testuser");
        let tx_result = neutron_controller.call_as(&sender).join()?;
        interchain.check_ibc("juno-1", tx_result)?;

        Ok(TestEnv {
            controller: neutron_controller,
            board: kuji_board,
            interchain,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let tiles_actions = vec![(1, TileAction::Action { action: None })];
    let env = TestEnv::setup("kujira".to_string(), tiles_actions.clone())?;
    let controller = env.controller;

    let sender = controller.get_chain().addr_make("testuser");
    let sender_canonical = controller
        .get_chain()
        .app
        .borrow()
        .api()
        .addr_canonicalize(sender.as_str())?;
    // let sender_humanized = env
    //     .board
    //     .get_chain()
    //     .app
    //     .borrow()
    //     .api()
    //     .addr_humanize(&sender_canonical)?;
    // let rep_unser = env
    //     .board
    //     .get_chain()
    //     .app
    //     .borrow()
    //     .wrap()
    //     .query_wasm_raw(
    //         env.board.address()?,
    //         ONGOING_ACTIONS.key(&sender_humanized).to_vec(),
    //     )?
    //     .unwrap();
    let resp = env.board.ongoing_action_from_canonical(sender_canonical)?;
    assert_eq!(resp.tile_id, 1);
    assert_eq!(resp.action, tiles_actions[0].1);
    Ok(())
}

#[test]
fn basic_execute() -> anyhow::Result<()> {
    let tiles_actions = vec![(
        0,
        TileAction::Action {
            action: Some(RequiredAction {
                required_funds: vec![Coin::new(1, "ukuji")],
                actions: vec![ActionType::Lend],
            }),
        },
    )];
    let env = TestEnv::setup("kujira".to_string(), tiles_actions.clone())?;
    let controller = env.controller;
    let board = env.board;

    let sender = controller.get_chain().addr_make("testuser");
    let sender_canonical = controller
        .get_chain()
        .app
        .borrow()
        .api()
        .addr_canonicalize(sender.as_str())?;

    let resp = board.ongoing_action_from_canonical(sender_canonical)?;
    assert_eq!(resp.tile_id, 0);
    assert_eq!(resp.action, tiles_actions[0].1);
    Ok(())
}

#[test]
fn test_rugg_or_candle_flow() -> anyhow::Result<()> {
    let tiles_actions = vec![(0, TileAction::Candle { n_tile: 3 })];
    let env = TestEnv::setup("kujira".to_string(), tiles_actions.clone())?;
    let controller = env.controller;
    let board = env.board;

    let sender = controller.get_chain().addr_make("testuser");
    let sender_canonical = controller
        .get_chain()
        .app
        .borrow()
        .api()
        .addr_canonicalize(sender.as_str())?;

    let resp = board.ongoing_action_from_canonical(sender_canonical)?;
    assert_eq!(resp.tile_id, 0);
    assert_eq!(resp.action, tiles_actions[0].1);

    Ok(())
}

// #[test]
// fn test_invalid_conditions() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let TestEnv::<_> {
//         controller,
//         board,
//         interchain,
//     } = env;
//     let controller_chain = controller.get_chain();
//     let board_chain = board.get_chain();
//
//     // Attempt to perform action without registration
//     let tx_result = board
//         .call_as(&board_chain.addr_make("test1"))
//         .perform_action();
//     assert!(
//         tx_result.is_err(),
//         "Action should fail if user is not registered"
//     );
//
//     Ok(())
// }
