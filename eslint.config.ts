import js from '@eslint/js';
import perfectionist from 'eslint-plugin-perfectionist';
import prettierRecommended from 'eslint-plugin-prettier/recommended';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import { defineConfig } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig(
    { ignores: ['dist', 'src-tauri'] },
    {
        extends: [
            js.configs.recommended,
            ...tseslint.configs.recommended,
            reactHooks.configs.flat.recommended,
        ],

        files: ['**/*.{ts,tsx}'],

        languageOptions: {
            ecmaVersion: 2020,
            globals: globals.browser,
        },

        plugins: {
            'react-refresh': reactRefresh,
            perfectionist,
        },

        rules: {
            'react-refresh/only-export-components': [
                'warn',
                { allowConstantExport: true },
            ],
            eqeqeq: ['error', 'smart'],

            'no-unused-vars': 'off',
            '@typescript-eslint/no-unused-vars': [
                'error',
                {
                    args: 'all',
                    argsIgnorePattern: '^_',
                    caughtErrors: 'all',
                    caughtErrorsIgnorePattern: '^_',
                    destructuredArrayIgnorePattern: '^_',
                    varsIgnorePattern: '^_',
                    ignoreRestSiblings: true,
                },
            ],
            'perfectionist/sort-imports': [
                'error',
                {
                    groups: [
                        'type',
                        'internal-type',
                        ['parent-type', 'sibling-type', 'index-type'],
                        'builtin',
                        'external',
                        'internal',
                        ['parent', 'sibling', 'index'],
                        'side-effect',
                        'object',
                        'unknown',
                    ],
                    order: 'asc',
                    type: 'natural',
                    newlinesBetween: 'always',
                },
            ],
            'perfectionist/sort-named-exports': 'error',
            'perfectionist/sort-named-imports': 'error',
        },
    },
    prettierRecommended,
    {
        rules: { curly: ['error', 'all'] },
    },
);
