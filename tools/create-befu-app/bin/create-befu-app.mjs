#!/usr/bin/env node

import { cpSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
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
  writeText(readmePath, readme.replace(/^# Befu$/m, `# ${appName}`))
}

/**
 * Scaffold a minimal Befu workspace.
 */
async function main() {
  const args = parseArgs(process.argv.slice(2))
  const needsPrompt = !args.yes && (args.name.length === 0 || args.platform.length === 0)
  const rl = needsPrompt ? createInterface({ input, output }) : null

  try {
    console.log('create-befu-app v0.1.0')
    console.log('Scaffold a full Befu workspace template.')

    const nameInput =
      args.name.length > 0 ? args.name : await rl.question('App name (default: my-befu-app): ')

    const appName = toSlug(nameInput || 'my-befu-app') || 'my-befu-app'

    const platformInput =
      args.platform.length > 0
        ? args.platform
        : await rl.question('Target platform [android/ios/both] (default: both): ')

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

    console.log('')
    console.log(`Created ${appName} at ${projectDir}`)
    console.log(`Target platform selection: ${platform}`)
    console.log('Note: v1 currently generates the full template for all platforms.')
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
