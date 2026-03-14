import { afterEach, describe, expect, it } from 'bun:test'
import {
  BridgeInvokeError,
  configureBridge,
  createNativeTransport,
  getNativeBackendMode,
  invoke,
} from './index'

const originalWindow = globalThis.window

afterEach(() => {
  globalThis.window = originalWindow
})

describe('bridge invoke', () => {
  it('returns pong via configured transport', async () => {
    configureBridge((payload) => {
      if (payload.command === 'hello') {
        return Promise.resolve({
          id: payload.id,
          ok: true,
          result: { message: `Hello ${(payload.args as { name: string }).name}` },
        })
      }

      if (payload.command === 'ping') {
        return Promise.resolve({
          id: payload.id,
          ok: true,
          result: { pong: 'pong' as const },
        })
      }

      return Promise.resolve({
        id: payload.id,
        ok: false,
        error: {
          code: 'UNKNOWN_COMMAND',
          message: 'unknown command',
        },
      })
    })

    const result = await invoke('ping')
    expect(result).toEqual({ pong: 'pong' })

    const helloResult = await invoke('hello', { name: 'Developer' })
    expect(helloResult).toEqual({ message: 'Hello Developer' })
  })

  it('throws typed bridge errors', () => {
    configureBridge((payload) => {
      return Promise.resolve({
        id: payload.id,
        ok: false,
        error: {
          code: 'APP_INFO_UNAVAILABLE',
          message: 'app info unavailable',
          details: { reason: 'not initialized' },
        },
      })
    })

    expect(invoke('app.info')).rejects.toBeInstanceOf(BridgeInvokeError)
  })

  it('returns unavailable mode when native bridge is absent', () => {
    globalThis.window = undefined as unknown as Window & typeof globalThis
    expect(getNativeBackendMode()).toBe('unavailable')
  })

  it('returns fallback mode when native bridge mode function is missing', () => {
    globalThis.window = {
      BefuNative: {
        invokeRaw: () => JSON.stringify({ id: '1', ok: true, result: { pong: 'pong' } }),
      },
    } as unknown as Window & typeof globalThis

    expect(getNativeBackendMode()).toBe('fallback')
  })

  it('returns fallback mode when native mode function throws', () => {
    globalThis.window = {
      BefuNative: {
        invokeRaw: () => JSON.stringify({ id: '1', ok: true, result: { pong: 'pong' } }),
        backendMode: () => {
          throw new Error('bad native state')
        },
      },
    } as unknown as Window & typeof globalThis

    expect(getNativeBackendMode()).toBe('fallback')
  })

  it('returns typed native bridge errors for invalid JSON response', async () => {
    globalThis.window = {
      BefuNative: {
        invokeRaw: () => '{invalid json}',
      },
    } as unknown as Window & typeof globalThis

    const nativeTransport = createNativeTransport()
    const response = await nativeTransport({ id: 'x', command: 'ping', args: undefined })

    expect(response.ok).toBe(false)
    if (!response.ok) {
      expect(response.error.code).toBe('NATIVE_BRIDGE_ERROR')
    }
  })
})
