use common::controller::{ControllerExecuteMsg, ControllerQueryMsg};

use crate::contract::Controller;

abstract_adapter::adapter_msg_types!(Controller, ControllerExecuteMsg, ControllerQueryMsg);