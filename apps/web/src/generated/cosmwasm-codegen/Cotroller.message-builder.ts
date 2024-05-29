// @ts-nocheck
          /**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { InstantiateMsg, ExecuteMsg, QueryMsg, AccountTrace, ChainName, AccountId, MigrateMsg, ConfigResponse, StatusResponse, UserPositionResponse } from "./Cotroller.types";
import { CamelCasedProperties } from "type-fest";
export abstract class CotrollerExecuteMsgBuilder {
static setStatus = ({
  status
}: CamelCasedProperties<Extract<ExecuteMsg, {
  set_status: unknown;
}>["set_status"]>): ExecuteMsg => {
  return {
    set_status: ({
      status
    } as const)
  };
};
static updateConfig = (): ExecuteMsg => {
  return {
    update_config: ({} as const)
  };
};
static join = (): ExecuteMsg => {
  return {
    join: ({} as const)
  };
};
static rollDice = (): ExecuteMsg => {
  return {
    roll_dice: ({} as const)
  };
};
}
export abstract class CotrollerQueryMsgBuilder {
static status = ({
  accountId
}: CamelCasedProperties<Extract<QueryMsg, {
  status: unknown;
}>["status"]>): QueryMsg => {
  return {
    status: ({
      account_id: accountId
    } as const)
  };
};
static config = (): QueryMsg => {
  return {
    config: ({} as const)
  };
};
static userPosition = ({
  userAddress
}: CamelCasedProperties<Extract<QueryMsg, {
  user_position: unknown;
}>["user_position"]>): QueryMsg => {
  return {
    user_position: ({
      user_address: userAddress
    } as const)
  };
};
}