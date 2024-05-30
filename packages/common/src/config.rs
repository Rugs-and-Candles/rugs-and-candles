// chainname and BoardConfig
use crate::board::ActionType;
use cosmwasm_std::Uint128;
use cosmwasm_std::Coin;
use crate::board::RequiredAction;
use crate::board::TileAction;
use crate::board::BoardInstantiateMsg;
#[cosmwasm_schema::cw_serde]
pub struct PositionRange {
    pub start: u32,
    pub end: u32,
}

pub fn controller_boards() -> Vec<(String, PositionRange)> {
    return vec![
        ("kujira".to_string(), PositionRange { start: 1, end: 10 }),
        ("juno".to_string(), PositionRange { start: 11, end: 20 }),
        ("osmosis".to_string(), PositionRange { start: 21, end: 30 }),
        ("terra2".to_string(), PositionRange { start: 31, end: 50 }),
        ]
}

pub fn board_chains_instantiate_msgs() -> Vec<(String, BoardInstantiateMsg)> {
    vec![
        //Kujira Board
        (
            "harpoon-1".to_string(),
            BoardInstantiateMsg {
                chain: "kujira".to_string(),
                tiles_actions: vec![
                    (1, TileAction::Action { action: None }),
                    (2, TileAction::Candle { n_tile: 10 }),
                    (3, TileAction::Action { action: None }),
                    (4, TileAction::Action { action: None }),
                    (
                        5,
                        TileAction::Action {
                            action: Some(RequiredAction {
                                required_funds: vec![Coin {
                                    denom: "ukuji".to_string(),
                                    amount: Uint128::new(1000000),
                                }],
                                actions: vec![ActionType::Lend],
                            }),
                        },
                    ),
                    (6, TileAction::Action { action: None }),
                    (7, TileAction::Action { action: None }),
                    (8, TileAction::Candle { n_tile: 10 }),
                    (9, TileAction::Action { action: None }),
                    (10, TileAction::Action { action: None }),
                ],
                tiles_number: 10,
            },
        ),
        (
            // Juno Board (11 to 20, candle at 19)
            "uni-6".to_string(),
            BoardInstantiateMsg {
                chain: "Juno".to_string(),
                tiles_actions: vec![
                    (11, TileAction::Action { action: None }),
                    (12, TileAction::Action { action: None }),
                    (13, TileAction::Action { action: None }),
                    (14, TileAction::Rugg { n_tile: 10 }),
                    (
                        15,
                        TileAction::Action {
                            action: Some(RequiredAction {
                                required_funds: vec![Coin {
                                    denom: "ujuno".to_string(),
                                    amount: Uint128::new(1000000),
                                }],
                                actions: vec![ActionType::Lend],
                            }),
                        },
                    ),
                    (16, TileAction::Rugg { n_tile: 11 }),
                    (17, TileAction::Action { action: None }),
                    (18, TileAction::Action { action: None }),
                    (19, TileAction::Candle { n_tile: 20 }),
                    (20, TileAction::Action { action: None }),
                ],
                tiles_number: 10,
            },
        ),
        // Osmosis Board
        (
            "osmosis-1".to_string(), // TODO: Change this to the actual chain id of osmosis testnet
            BoardInstantiateMsg {
                chain: "Osmosis".to_string(),
                tiles_actions: vec![
                    // Osmosis Board (21 to 30, candle at 29)
                    (21, TileAction::Action { action: None }),
                    (22, TileAction::Action { action: None }),
                    (23, TileAction::Action { action: None }),
                    (24, TileAction::Rugg { n_tile: 10 }),
                    (25, TileAction::Action { action: None }),
                    (26, TileAction::Rugg { n_tile: 10 }),
                    (27, TileAction::Candle { n_tile: 20 }),
                    (28, TileAction::Action { action: None }),
                    (29, TileAction::Action { action: None }),
                    (30, TileAction::Action { action: None }),
                ],
                tiles_number: 1,
            },
        ),
        // Stargaze Board (31 to 50, rugg at 41 and 43)
        (
            "pisco-1".to_string(),
            BoardInstantiateMsg {
                chain: "Stargaze".to_string(),
                tiles_actions: vec![
                    (31, TileAction::Action { action: None }),
                    (32, TileAction::Action { action: None }),
                    (33, TileAction::Action { action: None }),
                    (34, TileAction::Action { action: None }),
                    (35, TileAction::Action { action: None }),
                    (36, TileAction::Action { action: None }),
                    (37, TileAction::Action { action: None }),
                    (38, TileAction::Action { action: None }),
                    (39, TileAction::Action { action: None }),
                    (40, TileAction::Action { action: None }),
                    (41, TileAction::Rugg { n_tile: 10 }),
                    (42, TileAction::Action { action: None }),
                    (43, TileAction::Rugg { n_tile: 10 }),
                    (44, TileAction::Action { action: None }),
                    (45, TileAction::Action { action: None }),
                    (46, TileAction::Action { action: None }),
                    (47, TileAction::Action { action: None }),
                    (48, TileAction::Action { action: None }),
                    (49, TileAction::Action { action: None }),
                    (50, TileAction::Action { action: None }),
                ],
                tiles_number: 20,
            },
        ),
    ]
}
