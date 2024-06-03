/**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.8.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  set_status: {
    status: string;
  };
} | {
  update_config: {};
} | {
  join: {};
} | {
  roll_dice: {};
};
export type QueryMsg = {
  status: {
    account_id: AccountId;
  };
} | {
  config: {};
} | {
  user_position: {
    user_address: string;
  };
} | {
  participants: {};
};
export type AccountTrace = "local" | {
  remote: ChainName[];
};
export type ChainName = string;
export interface AccountId {
  seq: number;
  trace: AccountTrace;
}
export interface MigrateMsg {
  [k: string]: unknown;
}
export interface ConfigResponse {}
export type Addr = string;
export interface ParticipantsResponse {
  participants: [Addr, number][];
}
export interface StatusResponse {
  status?: string | null;
}
export interface UserPositionResponse {
  position?: number | null;
}