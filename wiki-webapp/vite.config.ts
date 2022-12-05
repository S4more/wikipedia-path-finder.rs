import { defineConfig } from 'vite'
import wasmPack from 'vite-plugin-wasm-pack';
import react from '@vitejs/plugin-react'
import viteCompression from 'vite-plugin-compression';

const compressionOptions = {
  filter: /\.(js|mjs|json|css|html|wasm)$/,
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    wasmPack('../node-visualizer'),
    viteCompression(compressionOptions)
  ]
})
