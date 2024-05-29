use abstract_adapter::std::adapter::AdapterRequestMsg;
use abstract_interchain_tests::setup::ibc_connect_abstract;
use abstract_interface::ManagerExecFns;
use board::{
    BoardExecuteMsgFns, BoardInstantiateMsg, BoardInterface, TileAction, RUGS_N_CANDLES_NAMESPACE,
};
use common::controller::{
    ConfigResponse, ControllerExecuteMsg, ControllerExecuteMsgFns, ControllerInstantiateMsg,
    ControllerQueryMsgFns,
};

use abstract_adapter::std::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use common::module_ids::CONTROLLER_ID;
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
            MockBech32InterchainEnv::new(vec![("neutron-1", "ntrn"), ("kujira-1", "kuji")]);
        let neutron = interchain.chain("neutron-1")?;
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

        ibc_connect_abstract(&interchain, "neutron-1", "kujira-1")?;
        ibc_connect_abstract(&interchain, "kujira-1", "neutron-1")?;

        let abstract_account = neturn_abs_client
            .account_builder()
            .install_on_sub_account(false)
            .build()?;
        abstract_account.install_adapter::<ControllerInterface<_>>(&[])?;
        let tx_result = abstract_account.as_ref().manager.execute_on_module(
            CONTROLLER_ID,
            common::controller::ExecuteMsg::Module(AdapterRequestMsg {
                proxy_address: Some(abstract_account.proxy()?.to_string()),
                request: ControllerExecuteMsg::Join {},
            }),
        )?;
        interchain.check_ibc("neutron-1", tx_result)?;

        Ok(TestEnv {
            controller: neutron_controller,
            board: kuji_board,
            interchain,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let tiles_actions = vec![(0, TileAction::Action { action: None })];
    let env = TestEnv::setup("kujira".to_string(), tiles_actions)?;
    let controller = env.controller;
    let config = controller.config()?;

    assert_eq!(config, ConfigResponse {});
    Ok(())
}

// #[test]
// fn basic_execute() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     // let controller = env.controller;
//     // let board = env.board;
//     // let interchain = &env.interchain;
//
//     let TestEnv::<_> {
//         controller,
//         board,
//         interchain,
//     } = env;
//     let controller_chain = controller.get_chain();
//     let board_chain = board.get_chain();
//
//     let response = controller.join()?;
//     let tx_result = controller
//         .call_as(&controller_chain.addr_make("test1"))
//         .join()?;
//     interchain.check_ibc("neutron-1", tx_result)?;
//
//     let tx_result = board
//         .call_as(&board_chain.addr_make("test1"))
//         .perform_action()?;
//     interchain.check_ibc("neutron-1", tx_result)?;
//
//     Ok(())
// }
//
// #[test]
// fn test_rugg_or_candle_flow() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let TestEnv::<_> {
//         controller,
//         board,
//         interchain,
//     } = env;
//     let controller_chain = controller.get_chain();
//     let board_chain = board.get_chain();
//
//     // User registration
//     let response = controller.join()?;
//     let tx_result = controller
//         .call_as(&controller_chain.addr_make("test1"))
//         .join()?;
//     interchain.check_ibc("neutron-1", tx_result)?;
//
//     // Mock 'Rugg' condition and verify handling
//     let tx_result = board
//         .call_as(&board_chain.addr_make("test1"))
//         .perform_action()?;
//     assert!(false);
//
//     // Mock 'Candle' condition and verify handling
//     let tx_result = board
//         .call_as(&board_chain.addr_make("test1"))
//         .perform_action()?;
//     assert!(false);
//
//     Ok(())
// }
//
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

// #[test]
// fn update_config() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;

//     // Executing it on publisher account
//     // Note that it's not a requirement to have it installed in this case
//     let publisher_account = env
//         .abs
//         .publisher_builder(Namespace::new(RUGS_N_CANDLES_NAMESPACE).unwrap())
//         .build()?;

//     adapter.execute(
//         &AdapterRequestMsg {
//             proxy_address: Some(publisher_account.account().proxy()?.to_string()),
//             request: ControllerExecuteMsg::UpdateConfig {},
//         }
//         .into(),
//         None,
//     )?;

//     let config = adapter.config()?;
//     let expected_response = common::controller::ConfigResponse {};
//     assert_eq!(config, expected_response);

//     // Adapter installed on sub-account of the publisher so this should error
//     let err = adapter
//         .execute(
//             &AdapterRequestMsg {
//                 proxy_address: Some(adapter.account().proxy()?.to_string()),
//                 request: ControllerExecuteMsg::UpdateConfig {},
//             }
//             .into(),
//             None,
//         )
//         .unwrap_err();
//     assert_eq!(err.root().to_string(), "Unauthorized");

//     Ok(())
// }

// #[test]
// fn set_status() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let adapter = env.controller.set_status(
//         "my_status".to_owned(),
//     )?;

//     let first_status = "my_status".to_owned();
//     let second_status = "my_status".to_owned();

//     let subaccount = &env.publisher.account().sub_accounts()?[0];

//     subaccount.as_ref().manager.execute_on_module(
//         CONTROLLER_ID,
//         ExecuteMsg::Module(AdapterRequestMsg {
//             proxy_address: Some(subaccount.proxy()?.to_string()),
//             request: ControllerExecuteMsg::SetStatus {
//                 status: first_status.clone(),
//             },
//         }),
//     )?;

//     let new_account = env
//         .abs
//         .account_builder()
//         .install_adapter::<ControllerInterface<MockBech32>>()?
//         .build()?;

//     new_account.as_ref().manager.execute_on_module(
//         CONTROLLER_ID,
//         ExecuteMsg::Module(AdapterRequestMsg {
//             proxy_address: Some(new_account.proxy()?.to_string()),
//             request: ControllerExecuteMsg::SetStatus {
//                 status: second_status.clone(),
//             },
//         }),
//     )?;

//     let status_response = adapter.status(adapter.account().id()?)?;
//     assert_eq!(status_response.status, Some(first_status));

//     let status_response = adapter.status(new_account.id()?)?;
//     assert_eq!(status_response.status, Some(second_status));

//     Ok(())
// }
