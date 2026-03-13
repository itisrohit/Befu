export type BridgeTransport = (payload: {
  id: string
  command: string
  args?: Record<string, unknown>
}) => Promise<unknown>

let transport: BridgeTransport | null = null

export function configureBridge(nextTransport: BridgeTransport): void {
  transport = nextTransport
}

export async function invoke<T = unknown>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!transport) {
    throw new Error('Bridge transport is not configured')
  }

  const payload = {
    id: crypto.randomUUID(),
    command,
    args,
  }

  const result = await transport(payload)
  return result as T
}
