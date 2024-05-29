pub mod api;
pub mod contract;
mod handlers;
pub mod state;

pub use contract::interface::BoardInterface;
pub use common::errors::BoardError;
pub use common::board::*;

/// The version of your Adapter
pub const BOARD_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use common::module_ids::BOARD_NAME;
pub use common::module_ids::BOARD_ID;
pub use common::module_ids::RUGS_N_CANDLES_NAMESPACE;
