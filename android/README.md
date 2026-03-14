# Befu Android Shell

Early Android shell scaffold for hosting Befu in a native `WebView`.

## What is implemented

- `MainActivity` with full-screen `WebView`
- JavaScript bridge object: `window.BefuNative`
- Native bridge method: `invokeRaw(payloadJson: string): string`
- Command stubs in native bridge:
  - `ping`
  - `app.info`
- JNI entrypoint path prepared for Rust:
  - Kotlin calls `invokeRawNative(...)`
  - Rust exports `Java_dev_befu_app_BefuNativeBridge_invokeRawNative`

Current behavior uses safe fallback command handling in Kotlin when `libbefu_core` is not loaded.

## Local run

1. Start web dev server from repo root:

```bash
bun run dev
```

2. Install Rust Android build tooling (one-time):

```bash
cargo install cargo-ndk
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

3. Ensure Android SDK/NDK are installed and configured:

- Set `ANDROID_HOME` or `ANDROID_SDK_ROOT`
- Install NDK via Android Studio SDK Manager

4. Build Android app:

```bash
cd android
./gradlew :app:assembleDebug
```

For debug dev server loading (`http://10.0.2.2:5173`), cleartext HTTP is enabled only in debug manifests.

During build, Gradle runs `cargo ndk` to produce Rust `.so` libraries under `app/src/main/jniLibs`.

For release builds, Gradle also runs a web asset pipeline:

- `bun run build` (from repo root)
- syncs `apps/web/dist` into `android/app/build/generated/befu-web-assets`

This allows release APKs to boot from bundled assets without a local dev server.

1. Run on emulator/device. The app points to `http://10.0.2.2:5173`.

Debug vs release app loading:

- Debug: `http://10.0.2.2:5173`
- Release: `https://appassets.androidplatform.net/assets/index.html`

6. Verify bridge mode in UI:

- `Native backend mode: jni` means Rust JNI path is active
- `Native backend mode: fallback` means Kotlin fallback is active
- `Native backend mode: unavailable` means native bridge is not present (desktop web/dev browser)

## Shortcuts (repo root)

```bash
bun run android:up
bun run android:status
bun run android:setup
bun run android:assemble:debug
bun run android:assemble:release
bun run android:install:debug
bun run android:app:restart
bun run android:logs
bun run android:down
```

Short aliases:

```bash
bun run a:up
bun run a:restart
bun run a:logs
bun run a:down
```

Smoke test flow:

```bash
bun run a:smoke
```

JNI is already integrated; use `Native backend mode` in the app to confirm runtime mode (`jni` or `fallback`).
