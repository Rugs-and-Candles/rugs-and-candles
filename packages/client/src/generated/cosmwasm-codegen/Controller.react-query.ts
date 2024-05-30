/**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.8.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from "@tanstack/react-query";
import { ExecuteResult } from "@abstract-money/cli/cosmjs";
import { StdFee, Coin } from "@abstract-money/cli/cosmjs";
import { InstantiateMsg, ExecuteMsg, QueryMsg, AccountTrace, ChainName, AccountId, MigrateMsg, ConfigResponse, Addr, ParticipantsResponse, StatusResponse, UserPositionResponse } from "./Controller.types";
import { ControllerAppQueryClient, ControllerAppClient } from "./Controller.client";
export const controllerQueryKeys = {
  contract: ([{
    contract: "controller"
  }] as const),
  address: (contractAddress: string | undefined) => ([{ ...controllerQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  status: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...controllerQueryKeys.address(contractAddress)[0],
    method: "status",
    args
  }] as const),
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...controllerQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const),
  userPosition: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...controllerQueryKeys.address(contractAddress)[0],
    method: "user_position",
    args
  }] as const),
  participants: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...controllerQueryKeys.address(contractAddress)[0],
    method: "participants",
    args
  }] as const)
};
export interface ControllerReactQuery<TResponse, TData = TResponse> {
  client: ControllerAppQueryClient | undefined;
  options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
    initialData?: undefined;
  };
}
export interface ControllerParticipantsQuery<TData> extends ControllerReactQuery<ParticipantsResponse, TData> {}
export function useControllerParticipantsQuery<TData = ParticipantsResponse>({
  client,
  options
}: ControllerParticipantsQuery<TData>) {
  return useQuery<ParticipantsResponse, Error, TData>(controllerQueryKeys.participants(client?._moduleAddress), () => client ? client.participants() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ControllerUserPositionQuery<TData> extends ControllerReactQuery<UserPositionResponse, TData> {
  args: undefined | {
    userAddress: string;
  };
}
export function useControllerUserPositionQuery<TData = UserPositionResponse>({
  client,
  args,
  options
}: ControllerUserPositionQuery<TData>) {
  return useQuery<UserPositionResponse, Error, TData>(controllerQueryKeys.userPosition(client?._moduleAddress, args), () => client && args ? client.userPosition({
    userAddress: args.userAddress
  }) : Promise.reject(new Error("Invalid client or args")), { ...options, enabled: !!args &&  !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ControllerConfigQuery<TData> extends ControllerReactQuery<ConfigResponse, TData> {}
export function useControllerConfigQuery<TData = ConfigResponse>({
  client,
  options
}: ControllerConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(controllerQueryKeys.config(client?._moduleAddress), () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ControllerStatusQuery<TData> extends ControllerReactQuery<StatusResponse, TData> {
  args: undefined | {
    accountId: AccountId;
  };
}
export function useControllerStatusQuery<TData = StatusResponse>({
  client,
  args,
  options
}: ControllerStatusQuery<TData>) {
  return useQuery<StatusResponse, Error, TData>(controllerQueryKeys.status(client?._moduleAddress, args), () => client && args ? client.status({
    accountId: args.accountId
  }) : Promise.reject(new Error("Invalid client or args")), { ...options, enabled: !!args &&  !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ControllerRollDiceMutation {
  client: ControllerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useControllerRollDiceMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ControllerRollDiceMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ControllerRollDiceMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.rollDice(fee, memo, funds), options);
}
export interface ControllerJoinMutation {
  client: ControllerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useControllerJoinMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ControllerJoinMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ControllerJoinMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.join(fee, memo, funds), options);
}
export interface ControllerUpdateConfigMutation {
  client: ControllerAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useControllerUpdateConfigMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ControllerUpdateConfigMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ControllerUpdateConfigMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.updateConfig(fee, memo, funds), options);
}
export interface ControllerSetStatusMutation {
  client: ControllerAppClient;
  msg: {
    status: string;
  };
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useControllerSetStatusMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ControllerSetStatusMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ControllerSetStatusMutation>(({
    client,
    msg,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.setStatus(msg, fee, memo, funds), options);
}