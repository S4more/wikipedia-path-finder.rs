import { defineConfig } from 'vite'
import wasmPack from 'vite-plugin-wasm-pack';
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasmPack('../node-visualizer')]
})
