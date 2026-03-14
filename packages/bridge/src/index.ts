export interface BridgeCommandMap {
  ping: {
    args: undefined
    result: { pong: 'pong' }
  }
  'app.info': {
    args: undefined
    result: {
      name: string
      version: string
      runtime: 'befu'
    }
  }
}

export type BridgeCommand = keyof BridgeCommandMap

export interface BridgeRequest<C extends BridgeCommand = BridgeCommand> {
  id: string
  command: C
  args: BridgeCommandMap[C]['args']
}

export interface BridgeError {
  code: string
  message: string
  details?: unknown
}

export interface BridgeSuccess<T> {
  id: string
  ok: true
  result: T
}

export interface BridgeFailure {
  id: string
  ok: false
  error: BridgeError
}

export type BridgeResponse<T> = BridgeSuccess<T> | BridgeFailure

export type BridgeTransport = (payload: BridgeRequest) => Promise<BridgeResponse<unknown>>

interface NativeBridge {
  invokeRaw(payloadJson: string): string | Promise<string>
  backendMode?: () => 'jni' | 'fallback'
}

declare global {
  interface Window {
    BefuNative?: NativeBridge
  }
}

export class BridgeInvokeError extends Error {
  code: string
  details?: unknown

  constructor(error: BridgeError) {
    super(error.message)
    this.name = 'BridgeInvokeError'
    this.code = error.code
    this.details = error.details
  }
}

let transport: BridgeTransport | null = null

function createRequestId(): string {
  if (typeof globalThis.crypto?.randomUUID === 'function') {
    return globalThis.crypto.randomUUID()
  }

  return `req-${Date.now()}-${Math.random().toString(16).slice(2)}`
}

export function configureBridge(nextTransport: BridgeTransport): void {
  transport = nextTransport
}

export function createNativeTransport(): BridgeTransport {
  return async (payload) => {
    const nativeBridge = globalThis.window?.BefuNative
    if (!nativeBridge) {
      return Promise.resolve({
        id: payload.id,
        ok: false,
        error: {
          code: 'NATIVE_BRIDGE_UNAVAILABLE',
          message: 'window.BefuNative.invokeRaw is unavailable',
        },
      })
    }

    const responseJson = await Promise.resolve(nativeBridge.invokeRaw(JSON.stringify(payload)))
    return JSON.parse(responseJson) as BridgeResponse<unknown>
  }
}

export function getNativeBackendMode(): 'jni' | 'fallback' | 'unavailable' {
  const nativeBridge = globalThis.window?.BefuNative
  if (!nativeBridge) {
    return 'unavailable'
  }

  return nativeBridge.backendMode?.() ?? 'fallback'
}

export async function invoke<C extends BridgeCommand>(
  command: C,
  args?: BridgeCommandMap[C]['args'],
): Promise<BridgeCommandMap[C]['result']> {
  if (!transport) {
    throw new Error('Bridge transport is not configured')
  }

  const payload: BridgeRequest<C> = {
    id: createRequestId(),
    command,
    args: (args ?? undefined) as BridgeCommandMap[C]['args'],
  }

  const response = await transport(payload)
  if (!response.ok) {
    throw new BridgeInvokeError(response.error)
  }

  return response.result as BridgeCommandMap[C]['result']
}
