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
  useBoardParticipantsQuery,
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
  useControllerRollDiceMutation,
  ControllerRollDiceMutation,
  useControllerJoinMutation,
  ControllerJoinMutation,
  useControllerUpdateConfigMutation,
  ControllerUpdateConfigMutation,
  useControllerSetStatusMutation,
  ControllerSetStatusMutation,
  useControllerParticipantsQuery,
  useControllerUserPositionQuery,
  useControllerConfigQuery,
  useControllerStatusQuery,
} from './cosmwasm-codegen/Controller.react-query'

import * as ControllerTypes from './cosmwasm-codegen/Controller.types'

import {
  ControllerAppQueryClient,
  ControllerAppClient,
} from './cosmwasm-codegen/Controller.client'

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Board
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Controller
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// React
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const BOARD_MODULE_ID = 'rugspaceandcandles:board'

export const board = {
  queries: {
    useParticipants: ({
      options,
      ...rest
    }: Omit<
      Parameters<
        typeof useBoardParticipantsQuery<BoardTypes.ParticipantsResponse>
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

      return useBoardParticipantsQuery({
        client: boardAppQueryClient,
        options,
      })
    },
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

export const CONTROLLER_MODULE_ID = 'rugspaceandcandles:controller'

export const controller = {
  queries: {
    useParticipants: ({
      options,
      ...rest
    }: Omit<
      Parameters<
        typeof useControllerParticipantsQuery<ControllerTypes.ParticipantsResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: controllerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: CONTROLLER_MODULE_ID,
        ...rest,
        Module: ControllerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useControllerParticipantsQuery({
        client: controllerAppQueryClient,
        options,
      })
    },
    useUserPosition: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<
        typeof useControllerUserPositionQuery<ControllerTypes.UserPositionResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: controllerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: CONTROLLER_MODULE_ID,
        ...rest,
        Module: ControllerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useControllerUserPositionQuery({
        client: controllerAppQueryClient,
        options,
        args,
      })
    },
    useConfig: ({
      options,
      ...rest
    }: Omit<
      Parameters<
        typeof useControllerConfigQuery<ControllerTypes.ConfigResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: controllerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: CONTROLLER_MODULE_ID,
        ...rest,
        Module: ControllerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useControllerConfigQuery({
        client: controllerAppQueryClient,
        options,
      })
    },
    useStatus: ({
      options,
      args,
      ...rest
    }: Omit<
      Parameters<
        typeof useControllerStatusQuery<ControllerTypes.StatusResponse>
      >[0],
      'client'
    > & {
      accountId: AccountId | undefined
      chainName: string | undefined
    }) => {
      const { data: controllerAppQueryClient } = useAbstractModuleQueryClient({
        moduleId: CONTROLLER_MODULE_ID,
        ...rest,
        Module: ControllerAppQueryClient,
        query: { enabled: options?.enabled },
      })

      return useControllerStatusQuery({
        client: controllerAppQueryClient,
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
          Omit<ControllerRollDiceMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: controllerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isControllerAppClientLoading,
        // isError: isControllerAppClientError,
        // error: controllerAppClientError,
      } = useAbstractModuleClient({
        moduleId: CONTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: ControllerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useControllerRollDiceMutation(options)

      const mutate = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: controllerAppClient, ...variables }, options)
      }, [mutate_, controllerAppClient])

      const mutateAsync = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) =>
          mutateAsync_({ client: controllerAppClient, ...variables }, options)
      }, [mutateAsync_, controllerAppClient])

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
          Omit<ControllerJoinMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: controllerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isControllerAppClientLoading,
        // isError: isControllerAppClientError,
        // error: controllerAppClientError,
      } = useAbstractModuleClient({
        moduleId: CONTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: ControllerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useControllerJoinMutation(options)

      const mutate = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: controllerAppClient, ...variables }, options)
      }, [mutate_, controllerAppClient])

      const mutateAsync = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) =>
          mutateAsync_({ client: controllerAppClient, ...variables }, options)
      }, [mutateAsync_, controllerAppClient])

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
          Omit<ControllerUpdateConfigMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: controllerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isControllerAppClientLoading,
        // isError: isControllerAppClientError,
        // error: controllerAppClientError,
      } = useAbstractModuleClient({
        moduleId: CONTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: ControllerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useControllerUpdateConfigMutation(options)

      const mutate = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: controllerAppClient, ...variables }, options)
      }, [mutate_, controllerAppClient])

      const mutateAsync = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) =>
          mutateAsync_({ client: controllerAppClient, ...variables }, options)
      }, [mutateAsync_, controllerAppClient])

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
          Omit<ControllerSetStatusMutation, 'client'>
        >,
        'mutationFn'
      >,
    ) => {
      const {
        data: controllerAppClient,
        // TODO: figure out what to do with those
        // isLoading: isControllerAppClientLoading,
        // isError: isControllerAppClientError,
        // error: controllerAppClientError,
      } = useAbstractModuleClient({
        moduleId: CONTROLLER_MODULE_ID,
        accountId,
        chainName,
        sender,

        Module: ControllerAppClient,
      })

      const {
        mutate: mutate_,
        mutateAsync: mutateAsync_,
        ...rest
      } = useControllerSetStatusMutation(options)

      const mutate = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutate_>[0], 'client'>,
          options?: Parameters<typeof mutate_>[1],
        ) => mutate_({ client: controllerAppClient, ...variables }, options)
      }, [mutate_, controllerAppClient])

      const mutateAsync = useMemo(() => {
        if (!controllerAppClient) return undefined

        return (
          variables: Omit<Parameters<typeof mutateAsync_>[0], 'client'>,
          options?: Parameters<typeof mutateAsync_>[1],
        ) =>
          mutateAsync_({ client: controllerAppClient, ...variables }, options)
      }, [mutateAsync_, controllerAppClient])

      return { mutate, mutateAsync, ...rest } as const
    },
  },
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Vanilla
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export * from './cosmwasm-codegen/Board.client'
export * from './cosmwasm-codegen/Board.message-builder'
export * from './cosmwasm-codegen/Board.message-composer'

export { BoardTypes }

export * from './cosmwasm-codegen/Controller.client'
export * from './cosmwasm-codegen/Controller.message-builder'
export * from './cosmwasm-codegen/Controller.message-composer'

export { ControllerTypes }
