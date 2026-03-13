import js from '@eslint/js'
import globals from 'globals'
import solid from 'eslint-plugin-solid/configs/typescript'
import eslintConfigPrettier from 'eslint-config-prettier'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  {
    ignores: [
      '**/node_modules/**',
      '**/dist/**',
      '**/coverage/**',
      '**/.turbo/**',
      '**/*.d.ts',
      'eslint.config.mjs',
    ],
  },
  js.configs.recommended,
  {
    files: ['**/*.{ts,tsx}'],
    extends: [...tseslint.configs.recommendedTypeChecked, ...tseslint.configs.stylisticTypeChecked],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    files: ['apps/web/**/*.{ts,tsx}'],
    ...solid,
    languageOptions: {
      ...solid.languageOptions,
      globals: {
        ...globals.browser,
      },
      parserOptions: {
        ...solid.languageOptions?.parserOptions,
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    files: ['packages/bridge/**/*.ts'],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
  eslintConfigPrettier,
)
