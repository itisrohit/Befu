import { describe, expect, it } from 'bun:test'
import { configureBridge, invoke } from './index'

describe('bridge invoke', () => {
  it('returns pong via configured transport', async () => {
    configureBridge((payload) => {
      if (payload.command === 'ping') {
        return Promise.resolve('pong')
      }

      return Promise.reject(new Error('unknown command'))
    })

    const result = await invoke<string>('ping')
    expect(result).toBe('pong')
  })
})
