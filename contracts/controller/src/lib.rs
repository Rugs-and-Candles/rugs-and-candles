pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod state;

pub use contract::interface::MyAdapterInterface;

/// The version of your Adapter
pub const CONTROLLER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CONTROLLER_NAMESPACE: &str = "rugsandcandles";
pub const CONTROLLER_NAME: &str = "controller";
pub const CONTROLLER_ID: &str = const_format::formatcp!("{CONTROLLER_NAMESPACE}:{CONTROLLER_NAME}");
