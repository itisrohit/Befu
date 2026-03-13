import { createSignal, onMount } from 'solid-js'
import { configureBridge, invoke } from '@befu/bridge'
import './App.css'

function App() {
  const [bridgeStatus, setBridgeStatus] = createSignal('Checking bridge...')
  const [pingCount, setPingCount] = createSignal(0)

  onMount(() => {
    configureBridge((payload) => {
      if (payload.command === 'ping') {
        return Promise.resolve('pong')
      }

      return Promise.reject(new Error(`Unknown command: ${payload.command}`))
    })

    void (async () => {
      try {
        const result = await invoke<string>('ping')
        setBridgeStatus(`Bridge is live (${result})`)
      } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error'
        setBridgeStatus(`Bridge failed: ${message}`)
      }
    })()
  })

  const handlePing = async () => {
    const result = await invoke<string>('ping')
    if (result === 'pong') {
      setPingCount((value) => value + 1)
    }
  }

  return (
    <main class="app-shell">
      <section class="card">
        <p class="eyebrow">Befu Runtime</p>
        <h1>Solid + Bridge + Rust scaffold</h1>
        <p class="status">{bridgeStatus()}</p>
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
          Frontend calls <code>invoke('ping')</code> through <code>@befu/bridge</code>.
        </p>
        <p>
          Rust crate exposes <code>ping()</code> in <code>crates/core</code> as the backend
          counterpart.
        </p>
      </section>
    </main>
  )
}

export default App
