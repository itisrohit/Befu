import React, { useState, useEffect } from 'react'
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

function App(): React.JSX.Element {
  const [bridgeStatus, setBridgeStatus] = useState('INITIALIZING...')
  const [pingCount, setPingCount] = useState(0)
  const [appInfo, setAppInfo] = useState<AppInfo>({
    version: '0.0.0',
    hot_reload: false,
  })
  const [backendMode, setBackendMode] = useState<'jni' | 'fallback' | 'ios' | 'unavailable'>(
    'unavailable',
  )
  const [helloResult, setHelloResult] = useState<string | null>(null)
  const [helloLoading, setHelloLoading] = useState(false)
  const [reloadLoading, setReloadLoading] = useState(false)

  // Use a ref as a synchronous mutex to prevent concurrent reloads
  const reloadInProgressRef = React.useRef(false)

  useEffect(() => {
    let disposed = false
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
      if (disposed) return nativeResponse
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

    const init = async () => {
      try {
        const pingResult = (await invoke('ping')) as { pong: string }
        if (disposed) return
        const info = (await invoke('app.info')) as { version: string; hot_reload: boolean }
        if (disposed) return
        setBridgeStatus(pingResult.pong === 'pong' ? 'BRIDGE LIVE' : 'DISCONNECTED')
        const hotReload = info.hot_reload === true
        setAppInfo({ version: info.version, hot_reload: hotReload })

        // Auto-reload the dynamic module on startup so fresh library is
        // picked up without requiring the user to press the button.
        if (hotReload) {
          try {
            await invoke('befu.reload')
          } catch {
            // Not fatal — app commands may not be ready yet on first cold boot
          }
        }
      } catch (e) {
        console.error('[Befu] Bridge initialization failed:', e)
        if (!disposed) setBridgeStatus(`ERROR`)
      }
    }
    void init()

    return () => {
      disposed = true
    }
  }, [])

  const handlePing = async () => {
    try {
      const result = (await invoke('ping')) as { pong: string }
      if (result.pong === 'pong') {
        setPingCount((v: number) => v + 1)
        setBridgeStatus('BRIDGE LIVE')
      }
    } catch (e) {
      console.error('[Befu] Ping failed:', e)
      setBridgeStatus('DISCONNECTED')
    }
  }

  const handleReload = async () => {
    if (reloadInProgressRef.current) return
    reloadInProgressRef.current = true
    setReloadLoading(true)
    setBridgeStatus('RELOADING...')
    setHelloResult(null)
    try {
      await invoke('befu.reload')
      setBridgeStatus('MODULE READY')
      const info = (await invoke('app.info')) as { version: string; hot_reload: boolean }
      setAppInfo({ version: info.version, hot_reload: info.hot_reload === true })
    } catch (e) {
      console.error('[Befu] Bridge reload failed:', e)
      setBridgeStatus(`RELOAD FAILED`)
    } finally {
      setReloadLoading(false)
      reloadInProgressRef.current = false
    }
  }

  const handleTestHello = async () => {
    setHelloLoading(true)
    setHelloResult(null)
    try {
      const res = (await invoke('hello', { name: 'Befu' })) as { message: string }
      setHelloResult(res.message)
    } catch (e) {
      setHelloResult(`Error: ${String(e)}`)
    } finally {
      setHelloLoading(false)
    }
  }

  return (
    <main className="app-shell">
      <header>
        <h1>Befu Native</h1>
        <p className="subtitle">Next-Gen Rust Mobile Runtime</p>
      </header>

      <section className="status-card">
        <div className="status-row">
          <span className="status-label">Bridge Status</span>
          <span className="status-value">{bridgeStatus}</span>
        </div>

        <div className="status-row">
          <span className="status-label">Hot Reload</span>
          <div className="dot-box">
            <span className={`dot ${appInfo.hot_reload ? 'dot-active' : ''}`} />
            <span className="status-value">{appInfo.hot_reload ? 'ACTIVE' : 'OFFLINE'}</span>
          </div>
        </div>

        <div className="status-row">
          <span className="status-label">Backend</span>
          <span className="status-value">{backendMode.toUpperCase()}</span>
        </div>

        <div className="status-row">
          <span className="status-label">Core v{appInfo.version}</span>
          <span className="status-value">RUST-DYNAMIC</span>
        </div>
      </section>

      <div role="status" aria-live="polite">
        {helloResult !== null && (
          <div className="result-card">
            <span className="result-label">Hello Result</span>
            <span className="result-value">{helloResult}</span>
          </div>
        )}
      </div>

      <div className="actions">
        <button onClick={() => void handlePing()}>Ping Native Bridge ({pingCount})</button>

        <button
          className="secondary"
          onClick={() => void handleTestHello()}
          disabled={helloLoading}
        >
          {helloLoading ? 'Calling...' : 'Test Hello Command ⚡️'}
        </button>

        {appInfo.hot_reload && (
          <button
            className="secondary"
            onClick={() => void handleReload()}
            disabled={reloadLoading}
          >
            {reloadLoading ? 'Reloading...' : '🔄 Reload Rust Module'}
          </button>
        )}
      </div>

      <footer className="footer-note">
        <p>
          Befu uses <code>JNI/FFI</code> binary bridging for maximum speed.
        </p>
      </footer>
    </main>
  )
}

export default App
