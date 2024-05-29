use common::board::{BoardExecuteMsg, BoardQueryMsg};

use crate::contract::BoardAdapter;

abstract_adapter::adapter_msg_types!(BoardAdapter, BoardExecuteMsg, BoardQueryMsg);