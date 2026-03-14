import { describe, expect, it } from 'bun:test'
import { BridgeInvokeError, configureBridge, invoke } from './index'

describe('bridge invoke', () => {
  it('returns pong via configured transport', async () => {
    configureBridge((payload) => {
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
})
