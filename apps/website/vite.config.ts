import { tanstackStart } from '@tanstack/react-start/plugin/vite';
import react from '@vitejs/plugin-react';
import { nitro } from 'nitro/vite';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 7805,
  },
  ssr: {
    noExternal: ['tslib'],
  },
  plugins: [
    tanstackStart({
      spa: {
        enabled: false,
        prerender: {
          enabled: true,
          crawlLinks: true,
        },
      },
    }),
    react(),
    // Hosting guidance: https://tanstack.com/start/latest/docs/framework/react/guide/hosting#nitro
    nitro({
      preset: 'cloudflare-module',
      cloudflare: { deployConfig: false },
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      // Required to avoid ESM resolution issues for tslib.
      tslib: 'tslib/tslib.es6.js',
    },
  },
});