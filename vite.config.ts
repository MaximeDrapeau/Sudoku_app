import react from '@vitejs/plugin-react';
import postcssNested from 'postcss-nested';
import postcssPresetMantine from 'postcss-preset-mantine';
import postcssSimpleVars from 'postcss-simple-vars';
import { defineConfig } from 'vite';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
    plugins: [react()],
    css: {
        postcss: {
            plugins: [
                postcssPresetMantine,
                postcssSimpleVars({
                    variables: {
                        'mantine-breakpoint-xs': '36em',
                        'mantine-breakpoint-sm': '48em',
                        'mantine-breakpoint-md': '62em',
                        'mantine-breakpoint-lg': '75em',
                        'mantine-breakpoint-xl': '88em',
                    },
                }),
                postcssNested(),
            ],
        },
    },

    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: 'ws',
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ['**/src-tauri/**'],
        },
    },
}));
