import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: true, // Allows the dev server to be accessible from outside the container
    port: 5173, // Sets a consistent development port
    strictPort: true, // Ensures Vite fails if the port is unavailable
    watch: {
      usePolling: true, // Needed for HMR to work reliably in some Docker environments
    },
  },
})
