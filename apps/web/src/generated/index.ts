'use client'

import { ExecuteResult } from '@abstract-money/cli/cosmjs'
import { UseMutationOptions } from '@tanstack/react-query'
import { useMemo } from 'react'

import {
  useAbstractModuleClient,
  useAbstractModuleQueryClient,
} from '@abstract-money/react'

import { AccountId } from '@abstract-money/core'

import {
  useBoardRollDiceMutation,
  BoardRollDiceMutation,
  useBoardJoinMutation,
  BoardJoinMutation,
  useBoardUpdateConfigMutation,
  BoardUpdateConfigMutation,
  useBoardSetStatusMutation,
  BoardSetStatusMutation,
  useBoardUserPositionQuery,
  useBoardConfigQuery,
  useBoardStatusQuery,
} from './cosmwasm-codegen/Board.react-query'

import * as BoardTypes from './cosmwasm-codegen/Board.types'

import {
  BoardAppQueryClient,
  BoardAppClient,
} from './cosmwasm-codegen/Board.client'

import {
  useCotrollerRollDiceMutation,
  CotrollerRollDiceMutation,
  useCotrollerJoinMutation,
  CotrollerJoinMutation,
  useCotrollerUpdateConfigMutation,
  CotrollerUpdateConfigMutation,
  useCotrollerSetStatusMutation,
  CotrollerSetStatusMutation,
  useCotrollerUserPositionQuery,
  useCotrollerConfigQuery,
  useCotrollerStatusQuery,
} from './cosmwasm-codegen/Cotroller.react-query'

import * as CotrollerTypes from './cosmwasm-codegen/Cotroller.types'

import {
  CotrollerAppQueryClient,
  CotrollerAppClient,
} from './cosmwasm-codegen/Cotroller.client'

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Board
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Cotroller
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// React
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const BOARD_MODULE_ID = 'rugspaceandcandles:board'

export const board = {
  queries: {
    useUserPosition: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<
        typeof useBoardUserPositionQuery<BoardTypes.UserPositionResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: boardAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: BOARD_MODULE_ID,
        ...rest,
        Module: BoardAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useBoardUserPositionQuery({
        client: boardAppQueryClient,
        options,
        args,
      })
    },
    useConfig: ({
      options,
      ...rest
    }: Omit<
      Parameters<typeof useBoardConfigQuery<BoardTypes.ConfigResponse>>[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: boardAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: BOARD_MODULE_ID,
        ...rest,
        Module: BoardAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useBoardConfigQuery({
        client: boardAppQueryClient,
        options,
      })
    },
    useStatus: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<typeof useBoardStatusQuery<BoardTypes.StatusResponse>>[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: boardAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: BOARD_MODULE_ID,
        ...rest,
        Module: BoardAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useBoardStatusQuery({
        client: boardAppQueryClient,
        options,
        args,
      })
    },
  },
  mutations: {
    useRollDice: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<BoardRollDiceMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: boardAppClient,
        // TODO: figure out what to do with those
        // isLoading: isBoardAppClientLoading,
        // isError: isBoardAppClientError,
        // error: boardAppClientError,
      } = useAbstractModuleClient({
        moduleId: BOARD_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: BoardAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useBoardRollDiceMutation(options)

      const mutate = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: boardAppClient, ...variables }, options)
      }, [mutate_, boardAppClient])

      const mutateAsync = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: boardAppClient, ...variables }, options)
      }, [mutateAsync_, boardAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useJoin: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<BoardJoinMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: boardAppClient,
        // TODO: figure out what to do with those
        // isLoading: isBoardAppClientLoading,
        // isError: isBoardAppClientError,
        // error: boardAppClientError,
      } = useAbstractModuleClient({
        moduleId: BOARD_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: BoardAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useBoardJoinMutation(options)

      const mutate = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: boardAppClient, ...variables }, options)
      }, [mutate_, boardAppClient])

      const mutateAsync = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: boardAppClient, ...variables }, options)
      }, [mutateAsync_, boardAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useUpdateConfig: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<BoardUpdateConfigMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: boardAppClient,
        // TODO: figure out what to do with those
        // isLoading: isBoardAppClientLoading,
        // isError: isBoardAppClientError,
        // error: boardAppClientError,
      } = useAbstractModuleClient({
        moduleId: BOARD_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: BoardAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useBoardUpdateConfigMutation(options)

      const mutate = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: boardAppClient, ...variables }, options)
      }, [mutate_, boardAppClient])

      const mutateAsync = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: boardAppClient, ...variables }, options)
      }, [mutateAsync_, boardAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useSetStatus: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<BoardSetStatusMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: boardAppClient,
        // TODO: figure out what to do with those
        // isLoading: isBoardAppClientLoading,
        // isError: isBoardAppClientError,
        // error: boardAppClientError,
      } = useAbstractModuleClient({
        moduleId: BOARD_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: BoardAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useBoardSetStatusMutation(options)

      const mutate = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: boardAppClient, ...variables }, options)
      }, [mutate_, boardAppClient])

      const mutateAsync = useMemo(() => {
        if (!boardAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: boardAppClient, ...variables }, options)
      }, [mutateAsync_, boardAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
  },
}

export const COTROLLER_MODULE_ID = 'rugspaceandcandles:cotroller'

export const cotroller = {
  queries: {
    useUserPosition: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<
        typeof useCotrollerUserPositionQuery<CotrollerTypes.UserPositionResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: cotrollerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: COTROLLER_MODULE_ID,
        ...rest,
        Module: CotrollerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useCotrollerUserPositionQuery({
        client: cotrollerAppQueryClient,
        options,
        args,
      })
    },
    useConfig: ({
      options,
      ...rest
    }: Omit<
      Parameters<
        typeof useCotrollerConfigQuery<CotrollerTypes.ConfigResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: cotrollerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: COTROLLER_MODULE_ID,
        ...rest,
        Module: CotrollerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useCotrollerConfigQuery({
        client: cotrollerAppQueryClient,
        options,
      })
    },
    useStatus: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<
        typeof useCotrollerStatusQuery<CotrollerTypes.StatusResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: cotrollerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: COTROLLER_MODULE_ID,
        ...rest,
        Module: CotrollerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useCotrollerStatusQuery({
        client: cotrollerAppQueryClient,
        options,
        args,
      })
    },
  },
  mutations: {
    useRollDice: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<CotrollerRollDiceMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: cotrollerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isCotrollerAppClientLoading,
        // isError: isCotrollerAppClientError,
        // error: cotrollerAppClientError,
      } = useAbstractModuleClient({
        moduleId: COTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: CotrollerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useCotrollerRollDiceMutation(options)

      const mutate = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutate_, cotrollerAppClient])

      const mutateAsync = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutateAsync_, cotrollerAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useJoin: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<CotrollerJoinMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: cotrollerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isCotrollerAppClientLoading,
        // isError: isCotrollerAppClientError,
        // error: cotrollerAppClientError,
      } = useAbstractModuleClient({
        moduleId: COTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: CotrollerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useCotrollerJoinMutation(options)

      const mutate = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutate_, cotrollerAppClient])

      const mutateAsync = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutateAsync_, cotrollerAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useUpdateConfig: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<CotrollerUpdateConfigMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: cotrollerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isCotrollerAppClientLoading,
        // isError: isCotrollerAppClientError,
        // error: cotrollerAppClientError,
      } = useAbstractModuleClient({
        moduleId: COTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: CotrollerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useCotrollerUpdateConfigMutation(options)

      const mutate = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutate_, cotrollerAppClient])

      const mutateAsync = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutateAsync_, cotrollerAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
    useSetStatus: (
      {
        accountId,
        chainName,
        sender,
      }: {
        accountId: AccountId | undefined
        chainName: string | undefined
        sender?: string | undefined
      },
      options?: Omit<
        UseMutationOptions<
          ExecuteResult,
          Error,
          Omit<CotrollerSetStatusMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: cotrollerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isCotrollerAppClientLoading,
        // isError: isCotrollerAppClientError,
        // error: cotrollerAppClientError,
      } = useAbstractModuleClient({
        moduleId: COTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: CotrollerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useCotrollerSetStatusMutation(options)

      const mutate = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutate_, cotrollerAppClient])

      const mutateAsync = useMemo(() => {
        if (!cotrollerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) => mutateAsync_({ client: cotrollerAppClient, ...variables }, options)
      }, [mutateAsync_, cotrollerAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
  },
}
