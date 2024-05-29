pub mod api;
pub mod contract;
mod handlers;
pub mod state;

pub use contract::interface::ControllerInterface;

/// The version of your Adapter
pub const CONTROLLER_VERSION: &str = env!("CARGO_PKG_VERSION");
