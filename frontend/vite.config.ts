import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import viteCompression from "vite-plugin-compression2";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
    viteCompression({
      include: /\.(js|css|html)$/,
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    // generiert .vite/manifest.json in outDir
    manifest: true,
    rollupOptions: {
      // Überschreibe den Standard-.html-Einstieg
      input: path.resolve(__dirname, "./src/main.tsx"),
    },
  },
});
