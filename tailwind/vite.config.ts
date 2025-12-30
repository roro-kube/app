import { defineConfig } from "vite";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  css: {
    postcss: "./postcss.config.js",
  },
  build: {
    cssCodeSplit: false,
    outDir: "../gui/assets",
    emptyOutDir: false,
    rollupOptions: {
      input: path.resolve(__dirname, "src/index.js"),
      output: {
        assetFileNames: "tailwind.css",
        entryFileNames: "tailwind.js", // Output minimal JS file (required by Vite)
      },
    },
  },
});

