/**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.8.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from "@tanstack/react-query";
import { ExecuteResult } from "@abstract-money/cli/cosmjs";
import { StdFee, Coin } from "@abstract-money/cli/cosmjs";
import { InstantiateMsg, ExecuteMsg, QueryMsg, AccountTrace, ChainName, AccountId, MigrateMsg, ConfigResponse, Addr, ParticipantsResponse, StatusResponse, UserPositionResponse } from "./Board.types";
import { BoardAppQueryClient, BoardAppClient } from "./Board.client";
export const boardQueryKeys = {
  contract: ([{
    contract: "board"
  }] as const),
  address: (contractAddress: string | undefined) => ([{ ...boardQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  status: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...boardQueryKeys.address(contractAddress)[0],
    method: "status",
    args
  }] as const),
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...boardQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const),
  userPosition: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...boardQueryKeys.address(contractAddress)[0],
    method: "user_position",
    args
  }] as const),
  participants: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...boardQueryKeys.address(contractAddress)[0],
    method: "participants",
    args
  }] as const)
};
export interface BoardReactQuery<TResponse, TData = TResponse> {
  client: BoardAppQueryClient | undefined;
  options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
    initialData?: undefined;
  };
}
export interface BoardParticipantsQuery<TData> extends BoardReactQuery<ParticipantsResponse, TData> {}
export function useBoardParticipantsQuery<TData = ParticipantsResponse>({
  client,
  options
}: BoardParticipantsQuery<TData>) {
  return useQuery<ParticipantsResponse, Error, TData>(boardQueryKeys.participants(client?._moduleAddress), () => client ? client.participants() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface BoardUserPositionQuery<TData> extends BoardReactQuery<UserPositionResponse, TData> {
  args: undefined | {
    userAddress: string;
  };
}
export function useBoardUserPositionQuery<TData = UserPositionResponse>({
  client,
  args,
  options
}: BoardUserPositionQuery<TData>) {
  return useQuery<UserPositionResponse, Error, TData>(boardQueryKeys.userPosition(client?._moduleAddress, args), () => client && args ? client.userPosition({
    userAddress: args.userAddress
  }) : Promise.reject(new Error("Invalid client or args")), { ...options, enabled: !!args &&  !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface BoardConfigQuery<TData> extends BoardReactQuery<ConfigResponse, TData> {}
export function useBoardConfigQuery<TData = ConfigResponse>({
  client,
  options
}: BoardConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(boardQueryKeys.config(client?._moduleAddress), () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface BoardStatusQuery<TData> extends BoardReactQuery<StatusResponse, TData> {
  args: undefined | {
    accountId: AccountId;
  };
}
export function useBoardStatusQuery<TData = StatusResponse>({
  client,
  args,
  options
}: BoardStatusQuery<TData>) {
  return useQuery<StatusResponse, Error, TData>(boardQueryKeys.status(client?._moduleAddress, args), () => client && args ? client.status({
    accountId: args.accountId
  }) : Promise.reject(new Error("Invalid client or args")), { ...options, enabled: !!args &&  !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface BoardRollDiceMutation {
  client: BoardAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useBoardRollDiceMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, BoardRollDiceMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, BoardRollDiceMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.rollDice(fee, memo, funds), options);
}
export interface BoardJoinMutation {
  client: BoardAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useBoardJoinMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, BoardJoinMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, BoardJoinMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.join(fee, memo, funds), options);
}
export interface BoardUpdateConfigMutation {
  client: BoardAppClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useBoardUpdateConfigMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, BoardUpdateConfigMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, BoardUpdateConfigMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.updateConfig(fee, memo, funds), options);
}
export interface BoardSetStatusMutation {
  client: BoardAppClient;
  msg: {
    status: string;
  };
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useBoardSetStatusMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, BoardSetStatusMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, BoardSetStatusMutation>(({
    client,
    msg,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.setStatus(msg, fee, memo, funds), options);
}