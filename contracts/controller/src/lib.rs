pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod state;

pub use contract::interface::MyAdapterInterface;

/// The version of your Adapter
pub const CONTROLLER_VERSION: &str = env!("CARGO_PKG_VERSION");
