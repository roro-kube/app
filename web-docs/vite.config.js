import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import mdx from '@mdx-js/rollup'
import remarkFrontmatter from 'remark-frontmatter'
import remarkRemoveFrontmatter from './scripts/remark-remove-frontmatter.mjs'
import path from 'path'
import { fileURLToPath } from 'url'
import { readFileSync, existsSync } from 'fs'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// Read HTML entries from generated file or default to index.html
function getHtmlEntries() {
  const entriesFile = path.join(__dirname, 'html-entries.json')
  if (existsSync(entriesFile)) {
    try {
      const entries = JSON.parse(readFileSync(entriesFile, 'utf-8'))
      return entries.reduce((acc, entry) => {
        acc[entry.name] = path.resolve(__dirname, entry.path)
        return acc
      }, {})
    } catch (err) {
      console.warn('Could not read html-entries.json, using index.html only')
    }
  }
  return {
    index: path.resolve(__dirname, 'index.html')
  }
}

export default defineConfig({
  base: process.env.BASE_PATH || '/',
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
      '@docs': path.resolve(__dirname, '../backlog/docs'),
      '@decisions': path.resolve(__dirname, '../backlog/decisions')
    }
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: getHtmlEntries()
    }
  }
})

