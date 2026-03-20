import { createSignal, onMount } from 'solid-js'
import {
  configureBridge,
  createNativeTransport,
  getNativeBackendMode,
  invoke,
  type BridgeTransport,
} from '@befu/bridge'
import './App.css'

function App() {
  const [bridgeStatus, setBridgeStatus] = createSignal('Checking bridge...')
  const [pingCount, setPingCount] = createSignal(0)
  const [appVersion, setAppVersion] = createSignal('unknown')
  const [backendMode, setBackendMode] = createSignal<'jni' | 'fallback' | 'ios' | 'unavailable'>(
    'unavailable',
  )

  onMount(() => {
    const mockTransport: BridgeTransport = (payload) => {
      if (payload.command === 'ping') {
        return Promise.resolve({
          id: payload.id,
          ok: true,
          result: { pong: 'pong' as const },
        })
      }

      if (payload.command === 'app.info') {
        return Promise.resolve({
          id: payload.id,
          ok: true,
          result: {
            name: 'Befu',
            version: '0.1.0-dev',
            runtime: 'befu' as const,
          },
        })
      }

      return Promise.resolve({
        id: payload.id,
        ok: false,
        error: {
          code: 'UNKNOWN_COMMAND',
          message: `Unknown command: ${String(payload.command)}`,
        },
      })
    }

    const nativeTransport = createNativeTransport()
    configureBridge(async (payload) => {
      const nativeResponse = await nativeTransport(payload)
      if (nativeResponse.ok) {
        setBackendMode(getNativeBackendMode())
        return nativeResponse
      }

      if (nativeResponse.error.code !== 'NATIVE_BRIDGE_UNAVAILABLE') {
        setBackendMode(getNativeBackendMode())
        return nativeResponse
      }

      setBackendMode('unavailable')
      return mockTransport(payload)
    })

    void (async () => {
      try {
        const pingResult = await invoke('ping')
        const appInfo = await invoke('app.info')
        setBridgeStatus(`Bridge is live (${pingResult.pong})`)
        setAppVersion(appInfo.version)
        console.log(`[befu:bridge:ok] bridge initialized with version: ${appInfo.version}`)
        console.log(`[befu:bridge:ping] ${pingResult.pong}`)
      } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error'
        setBridgeStatus(`Bridge failed: ${message}`)
        console.error(`[befu:bridge:error] ${message}`)
      }
    })()
  })

  const handlePing = async () => {
    const result = await invoke('ping')
    if (result.pong === 'pong') {
      setPingCount((value) => value + 1)
    }
  }

  return (
    <main class="app-shell">
      <section class="card">
        <p class="eyebrow">Befu Runtime</p>
        <h1>Solid + Bridge + Rust scaffold</h1>
        <p class="status">{bridgeStatus()}</p>
        <p class="status">
          App version from bridge: <code>{appVersion()}</code>
        </p>
        <p class="status">
          Native backend mode: <code>{backendMode()}</code>
        </p>
        <div class="actions">
          <button
            class="counter"
            onClick={() => {
              void handlePing()
            }}
          >
            Ping bridge ({pingCount()})
          </button>
        </div>
      </section>
      <section class="notes">
        <p>
          Frontend calls <code>invoke('ping')</code> and <code>invoke('app.info')</code> through{' '}
          <code>@befu/bridge</code>.
        </p>
        <p>
          Rust crate exposes protocol handlers in <code>crates/core</code> as the backend
          counterpart contract.
        </p>
      </section>
    </main>
  )
}

export default App
