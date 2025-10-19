import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import path from 'path'

export default defineConfig({
  plugins: [
    vue(),
    wasm(),
    topLevelAwait()
  ],
  resolve: {
    alias: {
      '@wasm': path.resolve(__dirname, '../wasm_frontend/pkg')
    }
  },
  server: {
    fs: {
      allow: ['..']
    }
  }
})
