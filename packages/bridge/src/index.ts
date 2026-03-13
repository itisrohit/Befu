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

export function configureBridge(nextTransport: BridgeTransport): void {
  transport = nextTransport
}

export async function invoke<C extends BridgeCommand>(
  command: C,
  args?: BridgeCommandMap[C]['args'],
): Promise<BridgeCommandMap[C]['result']> {
  if (!transport) {
    throw new Error('Bridge transport is not configured')
  }

  const payload: BridgeRequest<C> = {
    id: crypto.randomUUID(),
    command,
    args: (args ?? undefined) as BridgeCommandMap[C]['args'],
  }

  const response = await transport(payload)
  if (!response.ok) {
    throw new BridgeInvokeError(response.error)
  }

  return response.result as BridgeCommandMap[C]['result']
}
