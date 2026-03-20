import { createSignal, onMount } from 'solid-js'
import {
  configureBridge,
  createNativeTransport,
  getNativeBackendMode,
  invoke,
  type BridgeTransport,
} from '@befu/bridge'
import './App.css'

interface AppInfo {
  version: string
  hot_reload: boolean
}

function App() {
  const [bridgeStatus, setBridgeStatus] = createSignal('Checking bridge...')
  const [pingCount, setPingCount] = createSignal(0)
  const [appInfo, setAppInfo] = createSignal<AppInfo>({
    version: '...',
    hot_reload: false,
  })
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
            hot_reload: false,
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
        const info = await invoke('app.info')
        setBridgeStatus(`Bridge is live (${pingResult.pong})`)
        setAppInfo({ version: info.version, hot_reload: info.hot_reload === true })
      } catch {
        setBridgeStatus(`Bridge disconnected`)
      }
    })()
  })

  const handlePing = async () => {
    const result = await invoke('ping')
    if (result.pong === 'pong') {
      setPingCount((v) => v + 1)
    }
  }

  const handleReload = async () => {
    setBridgeStatus('Reloading Rust...')
    try {
      await invoke('befu.reload')
      setBridgeStatus('Bridge reloaded')
    } catch {
      setBridgeStatus(`Reload failed`)
    }
  }

  const handleTestHello = async () => {
    try {
      const res = await invoke('hello', { name: 'Befu' })
      alert(res.message)
    } catch (e) {
      alert(`Error calling hello: ${String(e)}`)
    }
  }

  return (
    <main class="app-shell">
      <header>
        <h1>Befu Native</h1>
        <p class="subtitle">Next-generation cross-platform mobile runtime</p>
      </header>

      <section class="status-grid">
        <div class="status-row">
          <span class="status-label">Runtime Connection</span>
          <span class="status-value">{bridgeStatus()}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Core Version</span>
          <span class="status-value">{appInfo().version}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Backend</span>
          <span class="status-value">{backendMode()}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Hot Reloading</span>
          <span class="status-value">
            <span class={`dot ${appInfo().hot_reload ? 'dot-active' : ''}`} />
            {appInfo().hot_reload ? 'Active' : 'Offline'}
          </span>
        </div>
      </section>

      <div class="actions">
        <button onClick={() => void handlePing()}>Ping Bridge ({pingCount()})</button>

        <button class="secondary" onClick={() => void handleTestHello()}>
          Test Hello Command ⚡️
        </button>

        {appInfo().hot_reload && (
          <button class="secondary" onClick={() => void handleReload()}>
            🔄 Reload Rust
          </button>
        )}
      </div>

      <footer class="footer-note">
        <p>
          Befu uses a JNI/FFI binary bridge to native Rust crates for maximum performance and low
          memory footprint.
        </p>
      </footer>
    </main>
  )
}

export default App
