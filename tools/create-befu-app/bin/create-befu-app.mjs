#!/usr/bin/env node

import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join, resolve } from 'node:path'
import { createInterface } from 'node:readline/promises'
import { stdin as input, stdout as output } from 'node:process'

const DEFAULT_PACKAGE_MANAGER = process.env.BEFU_PACKAGE_MANAGER ?? 'bun@1.2.16'
const DEFAULT_PLATFORM = 'both'

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url))
const TEMPLATE_ROOT = resolve(SCRIPT_DIR, '../../..')

const TEMPLATE_ENTRIES = [
  '.coderabbit.yaml',
  '.editorconfig',
  '.gitignore',
  '.prettierignore',
  '.prettierrc.json',
  'Cargo.toml',
  'Cargo.lock',
  'clippy.toml',
  'rustfmt.toml',
  'lefthook.yml',
  'eslint.config.mjs',
  'CONTRIBUTING.md',
  'README.md',
  'package.json',
  'bun.lock',
  '.github',
  'docs',
  'apps',
  'packages',
  'crates',
  'android',
  'ios',
  'scripts',
]

/**
 * Convert input text into a safe folder/package slug.
 */
function toSlug(value) {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9-\s]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')
}

/**
 * Write JSON content with trailing newline.
 */
function writeJson(filePath, obj) {
  writeFileSync(filePath, `${JSON.stringify(obj, null, 2)}\n`)
}

/**
 * Write plain text content to file.
 */
function writeText(filePath, content) {
  writeFileSync(filePath, content)
}

/**
 * Parse supported CLI flags.
 */
function parseArgs(argv) {
  const parsed = {
    name: '',
    platform: '',
    yes: false,
  }

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]
    if (arg === '--yes' || arg === '-y') {
      parsed.yes = true
      continue
    }

    if (arg === '--name' && argv[index + 1]) {
      parsed.name = argv[index + 1]
      index += 1
      continue
    }

    if (arg === '--platform' && argv[index + 1]) {
      parsed.platform = argv[index + 1]
      index += 1
      continue
    }
  }

  return parsed
}

/**
 * Recursively copy the canonical Befu template into destination.
 */
function copyTemplate(destinationDir) {
  for (const entry of TEMPLATE_ENTRIES) {
    cpSync(join(TEMPLATE_ROOT, entry), join(destinationDir, entry), { recursive: true })
  }
}

/**
 * Apply project-specific metadata updates after copying template.
 */
function applyProjectMetadata(projectDir, appName) {
  const packageJsonPath = join(projectDir, 'package.json')
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'))

  packageJson.name = appName
  packageJson.packageManager = DEFAULT_PACKAGE_MANAGER
  packageJson.workspaces = (packageJson.workspaces ?? []).filter(
    (workspace) => workspace !== 'tools/*',
  )
  delete packageJson.scripts?.['create:app']

  writeJson(packageJsonPath, packageJson)

  const readmePath = join(projectDir, 'README.md')
  const readme = readFileSync(readmePath, 'utf8')
  const withoutScaffoldSection = readme.replace(
    /## Scaffold a new app \(WIP\)[\s\S]*?## Code quality\n/m,
    '## Code quality\n',
  )
  writeText(readmePath, withoutScaffoldSection.replace(/^# Befu$/m, `# ${appName}`))
}

/**
 * Trim README sections that do not match selected platform.
 */
function pruneReadmeForPlatform(projectDir, platform) {
  if (platform === 'both') {
    return
  }

  const readmePath = join(projectDir, 'README.md')
  let readme = readFileSync(readmePath, 'utf8')

  if (platform === 'android') {
    readme = readme.replace(
      /## iOS shell \(early scaffold\)[\s\S]*?## Android shortcuts\n/m,
      '## Android shortcuts\n',
    )
    readme = readme.replace(/- Xcode `16\+` \(for iOS shell work\)\n/g, '')
    readme = readme.replace(
      /- iOS CI build: `iOS Simulator Build` \(asset prep \+ simulator compile\)\n/g,
      '',
    )
    readme = readme.replace(/bun run dev:mobile ios\n/g, '')
  }

  if (platform === 'ios') {
    readme = readme.replace(
      /## Android shell \(early scaffold\)[\s\S]*?## iOS shell \(early scaffold\)\n/m,
      '## iOS shell (early scaffold)\n',
    )
    readme = readme.replace(/## Android shortcuts[\s\S]*$/m, '')
    readme = readme.replace(
      /- Android CI build: `Android Debug Build` \(APK assemble \+ Rust JNI libs\)\n/g,
      '',
    )
    readme = readme.replace(/bun run dev:mobile android\n/g, '')
  }

  writeText(readmePath, readme)
}

/**
 * Remove scripts that do not apply to selected platform.
 */
function pruneScriptsForPlatform(packageJson, platform) {
  const isAndroidOnly = platform === 'android'
  const isIosOnly = platform === 'ios'

  if (isAndroidOnly) {
    for (const key of Object.keys(packageJson.scripts ?? {})) {
      if (key.startsWith('ios:') || key.startsWith('i:')) {
        delete packageJson.scripts[key]
      }
    }
  }

  if (isIosOnly) {
    for (const key of Object.keys(packageJson.scripts ?? {})) {
      if (key.startsWith('android:') || key.startsWith('a:')) {
        delete packageJson.scripts[key]
      }
    }
  }
}

/**
 * Rewrite platform-sensitive helper scripts in generated project.
 */
function rewritePlatformScripts(projectDir, platform) {
  const bootstrapPath = join(projectDir, 'scripts', 'bootstrap.sh')
  const bootstrap = `#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${'$'}{BASH_SOURCE[0]}")/.." && pwd)"

echo "[bootstrap] Installing JS dependencies..."
bun install

echo "[bootstrap] Installing git hooks..."
bun run --cwd "${'$'}ROOT_DIR" hooks:install

${
  platform === 'ios'
    ? ''
    : 'echo "[bootstrap] Setting up Android Rust toolchain..."\n' +
      'bash "${ROOT_DIR}/scripts/android/setup.sh"\n\n'
}
${
  platform === 'android'
    ? ''
    : 'echo "[bootstrap] Preparing iOS assets and Rust library..."\n' +
      'bun run --cwd "${ROOT_DIR}" ios:prepare\n\n'
}
echo "[bootstrap] Done."
echo "Next:"
echo "  - bun run dev"
${platform === 'ios' ? '' : 'echo "  - bun run a:up"\n'}${platform === 'android' ? '' : 'echo "  - bun run i:up"\n'}
`

  writeText(bootstrapPath, bootstrap)

  const doctorPath = join(projectDir, 'scripts', 'doctor.sh')
  const doctor = `#!/usr/bin/env bash

set -euo pipefail

check_cmd() {
  local cmd="${'$'}1"
  local label="${'$'}2"
  if command -v "${'$'}cmd" >/dev/null 2>&1; then
    echo "[ok] ${'$'}label"
  else
    echo "[missing] ${'$'}label"
  fi
}

echo "=== Befu Doctor ==="

echo
echo "[Core]"
check_cmd bun "Bun"
check_cmd rustup "rustup"
check_cmd cargo "cargo"
check_cmd rg "ripgrep (rg)"

${
  platform === 'ios'
    ? ''
    : `echo
echo "[Android]"
check_cmd java "Java"
check_cmd adb "adb"
check_cmd emulator "Android emulator"
if cargo ndk --version >/dev/null 2>&1; then
  echo "[ok] cargo-ndk"
else
  echo "[missing] cargo-ndk"
fi
`
}

${
  platform === 'android'
    ? ''
    : `echo
echo "[iOS]"
check_cmd xcodebuild "xcodebuild"
check_cmd xcrun "xcrun"
check_cmd xcodegen "xcodegen"
`
}

echo
echo "Doctor finished."
`
  writeText(doctorPath, doctor)

  const mobilePath = join(projectDir, 'scripts', 'dev', 'mobile.sh')
  const mobileScript = `#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${'$'}{BASH_SOURCE[0]}")/../.." && pwd)"

PLATFORM="${'$'}{1:-${platform}}"

case "${'$'}PLATFORM" in
  android)
    ${platform === 'ios' ? 'echo "[dev:mobile] Android flow not available in this template."\n    exit 1' : 'echo "[dev:mobile] Launching Android flow..."\n    bun run --cwd "${ROOT_DIR}" a:up'}
    ;;
  ios)
    ${platform === 'android' ? 'echo "[dev:mobile] iOS flow not available in this template."\n    exit 1' : 'echo "[dev:mobile] Launching iOS flow..."\n    bun run --cwd "${ROOT_DIR}" i:up'}
    ;;
  both)
    ${
      platform === 'both'
        ? 'echo "[dev:mobile] Launching Android then iOS..."\n    bun run --cwd "${ROOT_DIR}" a:up\n    bun run --cwd "${ROOT_DIR}" i:up'
        : 'echo "[dev:mobile] both is only available in both-platform template."\n    exit 1'
    }
    ;;
  *)
    echo "[dev:mobile] Unknown platform: ${'$'}PLATFORM"
    echo "Usage: bun run dev:mobile [android|ios|both]"
    exit 1
    ;;
esac

echo "[dev:mobile] Done."
`

  writeText(mobilePath, mobileScript)
}

/**
 * Remove files and directories for excluded platforms.
 */
function prunePlatformFiles(projectDir, platform) {
  if (platform === 'android') {
    rmSync(join(projectDir, 'ios'), { recursive: true, force: true })
    rmSync(join(projectDir, 'scripts', 'ios'), { recursive: true, force: true })
  }

  if (platform === 'ios') {
    rmSync(join(projectDir, 'android'), { recursive: true, force: true })
    rmSync(join(projectDir, 'scripts', 'android'), { recursive: true, force: true })
  }
}

/**
 * Apply platform choice to generated project content.
 */
function applyPlatformSelection(projectDir, platform) {
  const packageJsonPath = join(projectDir, 'package.json')
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'))
  pruneScriptsForPlatform(packageJson, platform)
  writeJson(packageJsonPath, packageJson)

  prunePlatformFiles(projectDir, platform)
  rewritePlatformScripts(projectDir, platform)
  pruneReadmeForPlatform(projectDir, platform)
}

/**
 * Scaffold a full Befu workspace.
 */
async function main() {
  const args = parseArgs(process.argv.slice(2))
  const needsPrompt = !args.yes && (args.name.length === 0 || args.platform.length === 0)
  const rl = needsPrompt ? createInterface({ input, output }) : null

  try {
    console.log('create-befu-app v0.1.0')
    console.log('Scaffold a full Befu workspace template.')

    const nameInput =
      args.name.length > 0
        ? args.name
        : rl
          ? await rl.question('App name (default: my-befu-app): ')
          : 'my-befu-app'

    const appName = toSlug(nameInput || 'my-befu-app') || 'my-befu-app'

    const platformInput =
      args.platform.length > 0
        ? args.platform
        : rl
          ? await rl.question('Target platform [android/ios/both] (default: both): ')
          : DEFAULT_PLATFORM

    const platformAnswer = platformInput.trim().toLowerCase()
    const platform = ['android', 'ios', 'both'].includes(platformAnswer)
      ? platformAnswer
      : DEFAULT_PLATFORM

    const projectDir = resolve(process.cwd(), appName)
    if (existsSync(projectDir)) {
      console.error(`Directory already exists: ${projectDir}`)
      process.exit(1)
    }

    mkdirSync(projectDir, { recursive: true })
    copyTemplate(projectDir)
    applyProjectMetadata(projectDir, appName)
    applyPlatformSelection(projectDir, platform)

    console.log('')
    console.log(`Created ${appName} at ${projectDir}`)
    console.log(`Target platform selection: ${platform}`)
    console.log('Next steps:')
    console.log(`  cd ${appName}`)
    console.log('  bun run bootstrap')
    console.log('  bun run dev')
  } finally {
    rl?.close()
  }
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})
