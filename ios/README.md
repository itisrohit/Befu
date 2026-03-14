# Befu iOS Shell

Minimal iOS shell scaffold using `WKWebView` with a JavaScript bridge contract compatible with the Android flow.

## What is implemented

- `WKWebView` host app (`ios/App/ViewController.swift`)
- JS bridge injected as `window.BefuNative.invokeRaw(payloadJson)`
- JSON envelope fallback handlers for:
  - `ping`
  - `app.info`
- Debug load mode: local dev server (`http://localhost:5173`)
- Release load mode: bundled web assets from app resources

## Prerequisites

- Xcode installed
- `xcodegen` installed (`brew install xcodegen`)

## Prepare project and assets

From repo root:

```bash
bun run ios:prepare
```

This runs:

- web build and sync to `ios/App/Resources/web`
- Xcode project generation at `ios/BefuIOS.xcodeproj`

## Run in Xcode

1. Open `ios/BefuIOS.xcodeproj`
2. Select `Befu` scheme
3. Run on simulator/device

## Run via CLI (simulator)

```bash
xcodebuild -project ios/BefuIOS.xcodeproj -scheme Befu -destination "platform=iOS Simulator,name=iPhone 17" -derivedDataPath ios/build build
xcrun simctl install booted ios/build/Build/Products/Debug-iphonesimulator/Befu.app
xcrun simctl launch booted dev.befu.ios
```

## Debug vs release loading behavior

- Debug: `http://localhost:5173` (requires `bun run dev`)
- Release-style fallback: bundled assets from `ios/App/Resources/web`

## Current status

- iOS bridge currently uses in-process Swift fallback handlers.
- Next step is wiring Swift bridge calls into Rust FFI entrypoints for full parity with Android JNI path.
