import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import mdx from '@mdx-js/rollup'
import remarkFrontmatter from 'remark-frontmatter'
import remarkRemoveFrontmatter from './scripts/remark-remove-frontmatter.mjs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

export default defineConfig({
  plugins: [
    react(),
    mdx({
      jsxImportSource: 'react',
      development: process.env.NODE_ENV !== 'production',
      remarkPlugins: [remarkFrontmatter, remarkRemoveFrontmatter],
    }),
  ],
  resolve: {
    alias: {
      '@docs': path.resolve(__dirname, '../backlog/docs')
    }
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true
  }
})

