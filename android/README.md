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

During build, Gradle runs `cargo ndk` to produce Rust `.so` libraries under `app/src/main/jniLibs`.

5. Run on emulator/device. The app points to `http://10.0.2.2:5173`.

6. Verify bridge mode in UI:

- `Android backend mode: jni` means Rust JNI path is active
- `Android backend mode: fallback` means Kotlin fallback is active
- `Android backend mode: unavailable` means native bridge is not present (desktop web/dev browser)

## Next step

- Build and package `libbefu_core.so` for Android ABIs so JNI path is active on-device.
