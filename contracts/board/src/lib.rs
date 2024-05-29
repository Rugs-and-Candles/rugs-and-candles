pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::BoardInterface;
pub use error::BoardError;
pub use msg::{BoardExecuteMsg, BoardInstantiateMsg};

/// The version of your Adapter
pub const BOARD_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const BOARD_NAMESPACE: &str = "rugsandcandles";
pub const BOARD_NAME: &str = "board";
pub const BOARD_ID: &str = const_format::formatcp!("{BOARD_NAMESPACE}:{BOARD_NAME}");
