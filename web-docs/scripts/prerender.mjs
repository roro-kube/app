import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)
const rootDir = join(__dirname, '..')
const distDir = join(rootDir, 'dist')
const srcDir = join(rootDir, 'src')

// Read docPages from the generated file
let docPages = []
try {
  const docPagesPath = join(srcDir, 'lib', 'docPages.js')
  const docPagesContent = readFileSync(docPagesPath, 'utf-8')
  // Extract routes from the docPages array
  const routeMatches = docPagesContent.matchAll(/route:\s*'([^']+)'/g)
  for (const match of routeMatches) {
    docPages.push({ route: match[1] })
  }
} catch (error) {
  console.error('❌ Could not read docPages.js:', error.message)
  process.exit(1)
}

// Read the built index.html
const indexPath = join(distDir, 'index.html')
if (!existsSync(indexPath)) {
  console.error('❌ dist/index.html not found. Run vite build first.')
  process.exit(1)
}

const indexHtml = readFileSync(indexPath, 'utf-8')
const basePath = process.env.BASE_PATH || '/'

// Extract the script and link tags from index.html
const scriptMatch = indexHtml.match(/<script[^>]*src="([^"]+)"[^>]*><\/script>/)
const linkMatch = indexHtml.match(/<link[^>]*href="([^"]+)"[^>]*>/g)

if (!scriptMatch) {
  console.error('❌ Could not find script tag in index.html')
  process.exit(1)
}

const scriptSrc = scriptMatch[1]
const links = linkMatch ? linkMatch.join('\n    ') : ''

// Generate HTML for each route
const routes = ['/', ...docPages.map(page => page.route)]

console.log(`🚀 Pre-rendering ${routes.length} routes...\n`)

for (const route of routes) {
  // Normalize route path
  let filePath = route === '/' ? 'index.html' : route + '.html'
  
  // Remove leading slash and handle base path
  if (filePath.startsWith('/')) {
    filePath = filePath.slice(1)
  }
  
  // Adjust paths for base path
  const adjustedScriptSrc = basePath !== '/' 
    ? scriptSrc.replace(/^\//, basePath)
    : scriptSrc
  
  const adjustedLinks = basePath !== '/'
    ? links.replace(/href="\//g, `href="${basePath}`)
    : links

  // Create directory if needed
  const fullPath = join(distDir, filePath)
  const dir = dirname(fullPath)
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true })
  }

  // Generate HTML with proper paths
  const html = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="${basePath}branding/logo.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    ${adjustedLinks}
    <title>Roro Kube Documentation</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="${adjustedScriptSrc}"></script>
  </body>
</html>`

  writeFileSync(fullPath, html, 'utf-8')
  console.log(`✓ Generated: ${filePath}`)
}

console.log(`\n✅ Pre-rendering complete! Generated ${routes.length} HTML files.`)

