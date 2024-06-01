/// Project namespace.
pub const RUGS_N_CANDLES_NAMESPACE: &str = "rugsandcandles";

// Controller contract info. These variables are used by AbstractSDK.
pub const CONTROLLER_NAME: &str = "controller";
pub const CONTROLLER_ID: &str =
    const_format::formatcp!("{RUGS_N_CANDLES_NAMESPACE}:{CONTROLLER_NAME}");

// Board contract info. These variables are used by AbstractSDK.
pub const BOARD_NAME: &str = "board";
pub const BOARD_ID: &str = const_format::formatcp!("{RUGS_N_CANDLES_NAMESPACE}:{BOARD_NAME}");
