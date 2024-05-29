/**
* This file was automatically generated by @abstract-money/ts-codegen@0.35.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { CamelCasedProperties } from "type-fest";
import { SigningCosmWasmClient, ExecuteResult } from "@abstract-money/cli/cosmjs";
import { AbstractQueryClient, AbstractAccountQueryClient, AbstractAccountClient, AppExecuteMsg, AppExecuteMsgFactory, AbstractClient, AbstractAccountId } from "@abstract-money/core/legacy";
import { StdFee, Coin } from "@abstract-money/cli/cosmjs";
import { InstantiateMsg, ExecuteMsg, QueryMsg, AccountTrace, ChainName, AccountId, MigrateMsg, ConfigResponse, StatusResponse } from "./Cotroller.types";
import { CotrollerQueryMsgBuilder, CotrollerExecuteMsgBuilder } from "./Cotroller.message-builder";
export interface ICotrollerAppQueryClient {
  moduleId: string;
  accountQueryClient: AbstractAccountQueryClient;
  _moduleAddress: string | undefined;
  status: (params: CamelCasedProperties<Extract<QueryMsg, {
    status: unknown;
  }>["status"]>) => Promise<StatusResponse>;
  config: () => Promise<ConfigResponse>;
  connectSigningClient: (signingClient: SigningCosmWasmClient, address: string) => CotrollerAppClient;
  getAddress: () => Promise<string>;
}
export class CotrollerAppQueryClient implements ICotrollerAppQueryClient {
  accountQueryClient: AbstractAccountQueryClient;
  moduleId: string;
  _moduleAddress: string | undefined;

  constructor({
    abstractQueryClient,
    accountId,
    managerAddress,
    proxyAddress,
    moduleId
  }: {
    abstractQueryClient: AbstractQueryClient;
    accountId: AbstractAccountId;
    managerAddress: string;
    proxyAddress: string;
    moduleId: string;
  }) {
    this.accountQueryClient = new AbstractAccountQueryClient({
      abstract: abstractQueryClient,
      accountId,
      managerAddress,
      proxyAddress
    });
    this.moduleId = moduleId;
    this.status = this.status.bind(this);
    this.config = this.config.bind(this);
  }

  status = async (params: CamelCasedProperties<Extract<QueryMsg, {
    status: unknown;
  }>["status"]>): Promise<StatusResponse> => {
    return this._query(CotrollerQueryMsgBuilder.status(params));
  };
  config = async (): Promise<ConfigResponse> => {
    return this._query(CotrollerQueryMsgBuilder.config());
  };
  getAddress = async (): Promise<string> => {
    if (!this._moduleAddress) {
      const address = await this.accountQueryClient.getModuleAddress(this.moduleId);

      if (address === null) {
        throw new Error(`Module ${this.moduleId} not installed`);
      }

      this._moduleAddress = address;
    }

    return this._moduleAddress!;
  };
  connectSigningClient = (signingClient: SigningCosmWasmClient, address: string): CotrollerAppClient => {
    return new CotrollerAppClient({
      accountId: this.accountQueryClient.accountId,
      managerAddress: this.accountQueryClient.managerAddress,
      proxyAddress: this.accountQueryClient.proxyAddress,
      moduleId: this.moduleId,
      abstractClient: this.accountQueryClient.abstract.connectSigningClient(signingClient, address)
    });
  };
  _query = async (queryMsg: QueryMsg): Promise<any> => {
    return this.accountQueryClient.queryModule({
      moduleId: this.moduleId,
      moduleType: "adapter",
      queryMsg
    });
  };
}
export interface ICotrollerAppClient extends ICotrollerAppQueryClient {
  accountClient: AbstractAccountClient;
  setStatus: (params: CamelCasedProperties<Extract<ExecuteMsg, {
    set_status: unknown;
  }>["set_status"]>, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateConfig: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  join: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  rollDice: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CotrollerAppClient extends CotrollerAppQueryClient implements ICotrollerAppClient {
  accountClient: AbstractAccountClient;

  constructor({
    abstractClient,
    accountId,
    managerAddress,
    proxyAddress,
    moduleId
  }: {
    abstractClient: AbstractClient;
    accountId: AbstractAccountId;
    managerAddress: string;
    proxyAddress: string;
    moduleId: string;
  }) {
    super({
      abstractQueryClient: abstractClient,
      accountId,
      managerAddress,
      proxyAddress,
      moduleId
    });
    this.accountClient = AbstractAccountClient.fromQueryClient(this.accountQueryClient, abstractClient);
    this.setStatus = this.setStatus.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.join = this.join.bind(this);
    this.rollDice = this.rollDice.bind(this);
  }

  setStatus = async (params: CamelCasedProperties<Extract<ExecuteMsg, {
    set_status: unknown;
  }>["set_status"]>, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return this._execute(CotrollerExecuteMsgBuilder.setStatus(params), fee, memo, _funds);
  };
  updateConfig = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return this._execute(CotrollerExecuteMsgBuilder.updateConfig(), fee, memo, _funds);
  };
  join = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return this._execute(CotrollerExecuteMsgBuilder.join(), fee, memo, _funds);
  };
  rollDice = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return this._execute(CotrollerExecuteMsgBuilder.rollDice(), fee, memo, _funds);
  };
  _execute = async (msg: ExecuteMsg, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    // const moduleMsg: AppExecuteMsg<ExecuteMsg> = AppExecuteMsgFactory.executeApp(msg);
    return await this.accountClient.abstract.client.execute(this.accountClient.sender, await this.getAddress(), moduleMsg, fee, memo, _funds);
  };
}
