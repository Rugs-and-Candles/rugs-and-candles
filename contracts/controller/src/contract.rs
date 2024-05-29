use crate::{
    error::ControllerError,
    handlers,
    msg::{ControllerExecuteMsg, ControllerInstantiateMsg, ControllerQueryMsg},
    CONTROLLER_ID, CONTROLLER_VERSION,
};

use abstract_adapter::AdapterContract;
use cosmwasm_std::Response;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type Controller = AdapterContract<
    ControllerError,
    ControllerInstantiateMsg,
    ControllerExecuteMsg,
    ControllerQueryMsg,
>;
/// The type of the result returned by your Adapter's entry points.
pub type AdapterResult<T = Response> = Result<T, ControllerError>;

const CONTROLLER: Controller = Controller::new(CONTROLLER_ID, CONTROLLER_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_module_ibc(handlers::module_ibc_handler);

// Export handlers
#[cfg(feature = "export")]
abstract_adapter::export_endpoints!(CONTROLLER, Controller);

abstract_adapter::cw_orch_interface!(
    CONTROLLER,
    Controller,
    ControllerInstantiateMsg,
    MyAdapterInterface
);
