import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(async () => ({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
      // 使用轮询模式 - 在 Windows 上更可靠
      usePolling: true,
      // 轮询间隔（毫秒）
      interval: 1000,
    },
  },
  // 优化文件监听
  optimizeDeps: {
    include: ['react', 'react-dom', 'react-router-dom'],
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
  },
}))
