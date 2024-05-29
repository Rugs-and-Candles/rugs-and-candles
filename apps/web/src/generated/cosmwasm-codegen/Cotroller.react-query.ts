/**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from "@tanstack/react-query";
import { ExecuteResult } from "@abstract-money/cli/cosmjs";
import { StdFee, Coin } from "@abstract-money/cli/cosmjs";
import { InstantiateMsg, ExecuteMsg, QueryMsg, AccountTrace, ChainName, AccountId, MigrateMsg, ConfigResponse, StatusResponse } from "./Cotroller.types";
import { CotrollerAppQueryClient, CotrollerAppClient } from "./Cotroller.client";
export const cotrollerQueryKeys = {
  contract: ([{
    contract: "cotroller"
  }] as const),
  address: (contractAddress: string | undefined) => ([{ ...cotrollerQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  status: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...cotrollerQueryKeys.address(contractAddress)[0],
    method: "status",
    args
  }] as const),
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...cotrollerQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const)
};
export interface CotrollerReactQuery<TResponse, TData = TResponse> {
  client: CotrollerAppQueryClient | undefined;
  options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
    initialData?: undefined;
  };
}
export interface CotrollerConfigQuery<TData> extends CotrollerReactQuery<ConfigResponse, TData> {}
export function useCotrollerConfigQuery<TData = ConfigResponse>({
  client,
  options
}: CotrollerConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(cotrollerQueryKeys.config(client?._moduleAddress), () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface CotrollerStatusQuery<TData> extends CotrollerReactQuery<StatusResponse, TData> {
  args: undefined | {
    accountId: AccountId;
  };
}
export function useCotrollerStatusQuery<TData = StatusResponse>({
  client,
  args,
  options
}: CotrollerStatusQuery<TData>) {
  return useQuery<StatusResponse, Error, TData>(cotrollerQueryKeys.status(client?._moduleAddress, args), () => client && args ? client.status({
    accountId: args.accountId
  }) : Promise.reject(new Error("Invalid client or args")), { ...options, enabled: !!args &&  !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface CotrollerRollDiceMutation {
  client: CotrollerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useCotrollerRollDiceMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, CotrollerRollDiceMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, CotrollerRollDiceMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.rollDice(fee, memo, funds), options);
}
export interface CotrollerJoinMutation {
  client: CotrollerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useCotrollerJoinMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, CotrollerJoinMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, CotrollerJoinMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.join(fee, memo, funds), options);
}
export interface CotrollerUpdateConfigMutation {
  client: CotrollerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useCotrollerUpdateConfigMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, CotrollerUpdateConfigMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, CotrollerUpdateConfigMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.updateConfig(fee, memo, funds), options);
}
export interface CotrollerSetStatusMutation {
  client: CotrollerAppClient;
  msg: {
    status: string;
  };
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useCotrollerSetStatusMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, CotrollerSetStatusMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, CotrollerSetStatusMutation>(({
    client,
    msg,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.setStatus(msg, fee, memo, funds), options);
}