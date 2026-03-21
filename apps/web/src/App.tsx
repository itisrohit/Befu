import { createSignal, onMount, Show } from 'solid-js'
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
  const [bridgeStatus, setBridgeStatus] = createSignal('INITIALIZING...')
  const [pingCount, setPingCount] = createSignal(0)
  const [appInfo, setAppInfo] = createSignal<AppInfo>({
    version: '0.0.0',
    hot_reload: false,
  })
  const [backendMode, setBackendMode] = createSignal<'jni' | 'fallback' | 'ios' | 'unavailable'>(
    'unavailable',
  )
  const [helloResult, setHelloResult] = createSignal<string | null>(null)
  const [helloLoading, setHelloLoading] = createSignal(false)
  const [reloadLoading, setReloadLoading] = createSignal(false)

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

      if (payload.command === 'hello') {
        const name = (payload.args as { name: string }).name || 'Befu'
        return Promise.resolve({
          id: payload.id,
          ok: true,
          result: { message: `Hello ${name} (Web Mock)` as const },
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
        setBridgeStatus(pingResult.pong === 'pong' ? 'BRIDGE LIVE' : 'DISCONNECTED')
        setAppInfo({ version: info.version, hot_reload: info.hot_reload === true })
      } catch (e) {
        console.error('[Befu] Bridge initialization failed:', e)
        setBridgeStatus(`ERROR`)
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
    if (reloadLoading()) return
    setReloadLoading(true)
    setBridgeStatus('RELOADING...')
    setHelloResult(null)
    try {
      await invoke('befu.reload')
      setBridgeStatus('MODULE READY')
      const info = await invoke('app.info')
      setAppInfo({ version: info.version, hot_reload: info.hot_reload === true })
    } catch (e) {
      console.error('[Befu] Bridge reload failed:', e)
      setBridgeStatus(`RELOAD FAILED`)
    } finally {
      setReloadLoading(false)
    }
  }

  const handleTestHello = async () => {
    setHelloLoading(true)
    setHelloResult(null)
    try {
      const res = await invoke('hello', { name: 'Befu' })
      setHelloResult(res.message)
    } catch (e) {
      setHelloResult(`Error: ${String(e)}`)
    } finally {
      setHelloLoading(false)
    }
  }

  return (
    <main class="app-shell">
      <header>
        <h1>Befu Native</h1>
        <p class="subtitle">Next-Gen Rust Mobile Runtime</p>
      </header>

      <section class="status-card">
        <div class="status-row">
          <span class="status-label">Bridge Status</span>
          <span class="status-value">{bridgeStatus()}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Hot Reload</span>
          <div class="dot-box">
            <span class={`dot ${appInfo().hot_reload ? 'dot-active' : ''}`} />
            <span class="status-value">{appInfo().hot_reload ? 'ACTIVE' : 'OFFLINE'}</span>
          </div>
        </div>

        <div class="status-row">
          <span class="status-label">Backend</span>
          <span class="status-value">{backendMode().toUpperCase()}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Core v{appInfo().version}</span>
          <span class="status-value">RUST-DYNAMIC</span>
        </div>
      </section>

      <div role="status" aria-live="polite">
        <Show when={helloResult() !== null}>
          <div class="result-card">
            <span class="result-label">Hello Result</span>
            <span class="result-value">{helloResult()}</span>
          </div>
        </Show>
      </div>

      <div class="actions">
        <button onClick={() => void handlePing()}>Ping Native Bridge ({pingCount()})</button>

        <button class="secondary" onClick={() => void handleTestHello()} disabled={helloLoading()}>
          {helloLoading() ? 'Calling...' : 'Test Hello Command ⚡️'}
        </button>

        {appInfo().hot_reload && (
          <button class="secondary" onClick={() => void handleReload()} disabled={reloadLoading()}>
            {reloadLoading() ? 'Reloading...' : '🔄 Reload Rust Module'}
          </button>
        )}
      </div>

      <footer class="footer-note">
        <p>
          Befu uses <code>JNI/FFI</code> binary bridging for maximum speed.
        </p>
      </footer>
    </main>
  )
}

export default App
