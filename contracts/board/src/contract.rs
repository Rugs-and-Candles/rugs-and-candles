use crate::{handlers, BoardError, BOARD_ID, BOARD_VERSION};

use common::board::*;

use cosmwasm_std::Response;

/// The type of the result returned by your Adapter's entry points.
pub type BoardResult<T = Response> = Result<T, BoardError>;

const BOARD: BoardAdapter = BoardAdapter::new(BOARD_ID, BOARD_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_replies(&[(0, handlers::reply::reply_handler)])
    .with_module_ibc(handlers::module_ibc_handler);
// .with_ibc_callbacks(&[
//     "ReplyOnBoardUpdate",
// ]);

// Export handlers
#[cfg(feature = "export")]
abstract_adapter::export_endpoints!(BOARD, BoardAdapter);

abstract_adapter::cw_orch_interface!(BOARD, BoardAdapter, BoardInstantiateMsg, BoardInterface);
