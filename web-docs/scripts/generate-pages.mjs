import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import matter from "gray-matter";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.join(__dirname, "..");
const docsDir = path.join(rootDir, "..", "backlog", "docs");
const decisionsDir = path.join(rootDir, "..", "backlog", "decisions");
const pagesDir = path.join(rootDir, "src", "pages");

// Clean and create pages directory
async function setupPagesDir() {
  try {
    await fs.rm(pagesDir, { recursive: true, force: true });
  } catch (err) {
    // Ignore if doesn't exist
  }
  await fs.mkdir(pagesDir, { recursive: true });
}

// Recursively find all markdown files
async function findMarkdownFiles(dir, basePath = "") {
  const files = [];
  const entries = await fs.readdir(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    const relativePath = path.join(basePath, entry.name);

    if (entry.isDirectory()) {
      const subFiles = await findMarkdownFiles(fullPath, relativePath);
      files.push(...subFiles);
    } else if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push({
        fullPath,
        relativePath: relativePath.replace(/\.md$/, ""),
        fileName: entry.name,
      });
    }
  }

  return files;
}

// Generate page component from markdown file
function generatePageComponent(fileInfo, frontMatter, importPath, route) {
  // Calculate relative path to DocsLayout
  const depth = fileInfo.relativePath.split(path.sep).length;
  const layoutPath = "../".repeat(depth) + "layouts/DocsLayout";
  const componentsPath = "../".repeat(depth) + "components/MDXComponents";

  const title = frontMatter.title.replace(/'/g, "\\'")
  const id = frontMatter.id || ''
  const type = frontMatter.type || ''
  const createdDate = frontMatter.created_date || ''

  return `import React from 'react'
import ReactDOM from 'react-dom/client'
import DocsLayout from '${layoutPath}'
import DocContent from '${importPath}'
import { MDXWrapper } from '${componentsPath}'

function Page() {
  return (
    <DocsLayout currentPath="${route}">
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-8">
        <header className="mb-6 pb-6 border-b border-gray-200">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">${title}</h1>
          <div className="flex flex-wrap gap-3">
            <span className="inline-flex items-center px-3 py-1 rounded-full text-sm bg-gray-100 text-gray-700">
              <svg className="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              ID: ${id}
            </span>
            <span className="inline-flex items-center px-3 py-1 rounded-full text-sm bg-gray-100 text-gray-700">
              <svg className="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              ${type}
            </span>
${createdDate ? `            <span className="inline-flex items-center px-3 py-1 rounded-full text-sm bg-gray-100 text-gray-700">
              <svg className="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              Created: ${createdDate}
            </span>` : ''}
          </div>
        </header>
        <article className="prose prose-lg prose-slate max-w-none">
          <MDXWrapper>
            <DocContent />
          </MDXWrapper>
        </article>
      </div>
    </DocsLayout>
  )
}

// Auto-render on load
ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <Page />
  </React.StrictMode>
)

// Export for docPages.js
export default Page

export const frontMatter = ${JSON.stringify(frontMatter, null, 2)}
`;
}

// Generate page file
async function generatePageFile(fileInfo, category = "docs") {
  let data;
  try {
    const fileContent = await fs.readFile(fileInfo.fullPath, "utf-8");
    const parsed = matter(fileContent);
    data = parsed.data;
  } catch (error) {
    console.error(`Error reading ${fileInfo.fullPath}:`, error.message);
    throw error;
  }

  // Extract title from filename if not in front-matter
  // Remove decision-XXXX - or doc-XXXX - prefix
  const defaultTitle = fileInfo.fileName
    .replace(/^(decision|doc)-\d+\s*-\s*/, "")
    .replace(/\.md$/, "")
    .trim();

  const frontMatter = {
    id: data.id || "",
    title: data.title || defaultTitle,
    type: data.type || category,
    created_date: data.created_date || data.date || "",
    updated_date: data.updated_date || "",
    path: fileInfo.relativePath,
    category: category,
  };

  // For decisions, update the import path to use decisions folder
  const importPath = category === "decisions" 
    ? `@decisions/${fileInfo.fileName.replace(/\.md$/, "")}.md?mdx`
    : `@docs/${fileInfo.relativePath.replace(/\\/g, "/")}.md?mdx`;

  // Generate route from relative path (preserve folder structure)
  const route = "/" + fileInfo.relativePath.replace(/\\/g, "/");
  
  const pageContent = generatePageComponent(fileInfo, frontMatter, importPath, route);

  // Create directory structure
  const pageDir = path.join(pagesDir, path.dirname(fileInfo.relativePath));
  await fs.mkdir(pageDir, { recursive: true });

  // Write page file
  const pageFileName = path.basename(fileInfo.relativePath) + ".jsx";
  const pageFilePath = path.join(pageDir, pageFileName);
  await fs.writeFile(pageFilePath, pageContent, "utf-8");

  return {
    route,
    path: fileInfo.relativePath,
    title: frontMatter.title,
    type: frontMatter.type,
    id: frontMatter.id,
    category: category,
    filePath: pageFilePath,
  };
}

// Generate index page component
async function generateIndexPage() {
  const indexPageContent = `import React from 'react'
import ReactDOM from 'react-dom/client'
import DocsLayout from '../layouts/DocsLayout'
import { docPages } from '../lib/docPages'

function HomePage() {
  const basePath = import.meta.env.BASE_URL || '/'
  
  const getHref = (route) => {
    if (basePath === '/') return route;
    return basePath + route.replace(/^\\//, '');
  };
  
  return (
    <DocsLayout currentPath="/">
      <div className="max-w-4xl mx-auto px-6 py-12">
        <h1 className="text-4xl font-bold mb-6">Roro Kube Documentation</h1>
        <p className="text-lg text-gray-600 mb-8">
          Welcome to the Roro Kube documentation. Navigate using the sidebar to
          explore different topics.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {docPages.length > 0 ? (
            docPages.map((page) => (
              <a
                key={page.route}
                href={getHref(page.route)}
                className="block p-4 border border-gray-200 rounded-lg hover:border-blue-500 hover:shadow-md transition"
              >
                <h2 className="text-xl font-semibold mb-2">{page.title}</h2>
                <p className="text-sm text-gray-500 capitalize">{page.type}</p>
              </a>
            ))
          ) : (
            <p className="text-gray-500">
              No pages found. Run{" "}
              <code className="bg-gray-100 px-2 py-1 rounded">
                npm run generate
              </code>{" "}
              to generate pages.
            </p>
          )}
        </div>
      </div>
    </DocsLayout>
  )
}

// Auto-render on load
ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <HomePage />
  </React.StrictMode>
)

// Export for docPages.js
export default HomePage
`;

  await fs.writeFile(
    path.join(pagesDir, 'index.jsx'),
    indexPageContent,
    'utf-8'
  );
  console.log('✓ Generated: index.jsx');
}

// Generate HTML entry points for each page
async function generateHtmlEntries(pages) {
  const basePath = process.env.BASE_PATH || '/';
  const htmlDir = rootDir;
  const entries = [];
  
  // Generate homepage
  // For script path, use base path if set, otherwise absolute path
  const scriptPath = basePath !== '/' 
    ? basePath + 'src/pages/index.jsx'
    : '/src/pages/index.jsx';
  
  const homepageHtml = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="${basePath}branding/logo.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="${basePath}tailwind.css" />
    <title>Roro Kube Documentation</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="${scriptPath}"></script>
  </body>
</html>`;
  
  await fs.writeFile(path.join(htmlDir, 'index.html'), homepageHtml, 'utf-8');
  entries.push({ name: 'index', path: 'index.html' });
  
  // Generate HTML files for each page
  for (const page of pages) {
    if (page.route === '/') continue; // Skip homepage, already generated
    
    const htmlPath = page.route.startsWith('/') 
      ? page.route.slice(1) + '.html' 
      : page.route + '.html';
    const fullHtmlPath = path.join(htmlDir, htmlPath);
    const htmlDirPath = path.dirname(fullHtmlPath);
    
    await fs.mkdir(htmlDirPath, { recursive: true });
    
    // Calculate paths based on base path
    // If basePath is /app/, we need to use absolute paths with base path
    // If basePath is /, we can use relative paths
    const htmlDepth = htmlPath.split('/').length - 1;
    const relativeBase = '../'.repeat(htmlDepth);
    
    let scriptPath, assetPath;
    if (basePath !== '/') {
      // Use absolute paths with base path
      scriptPath = basePath + 'src/pages/' + page.path.replace(/\\/g, '/') + '.jsx';
      assetPath = basePath;
    } else {
      // Use relative paths
      scriptPath = relativeBase + 'src/pages/' + page.path.replace(/\\/g, '/') + '.jsx';
      assetPath = relativeBase;
    }
    
    const pageHtml = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="${assetPath}branding/logo.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="${assetPath}tailwind.css" />
    <title>${page.title} - Roro Kube Documentation</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="${scriptPath}"></script>
  </body>
</html>`;
    
    await fs.writeFile(fullHtmlPath, pageHtml, 'utf-8');
    
    // Generate entry name from route (sanitize for Vite)
    const entryName = htmlPath.replace(/\.html$/, '').replace(/\//g, '_').replace(/^_/, '') || 'index';
    entries.push({ name: entryName, path: htmlPath });
  }
  
  // Write entries file for Vite
  await fs.writeFile(
    path.join(htmlDir, 'html-entries.json'),
    JSON.stringify(entries, null, 2),
    'utf-8'
  );
}

// Generate docPages.js file that exports all pages
async function generateDocPagesFile(pages) {
  const libDir = path.join(rootDir, "src", "lib");
  await fs.mkdir(libDir, { recursive: true });

  const imports = pages
    .map((page, index) => {
      // Calculate relative path from src/lib/ to src/pages/
      const importPath = "../pages/" + page.path.replace(/\\/g, "/") + ".jsx";
      return `import Page${index} from '${importPath}'`;
    })
    .join("\n");

  const routes = pages
    .map((page, index) => {
      return `  {
    route: '${page.route}',
    path: '${page.path}',
    title: ${JSON.stringify(page.title)},
    type: ${JSON.stringify(page.type)},
    id: ${JSON.stringify(page.id)},
    category: ${JSON.stringify(page.category || "docs")},
    component: Page${index}
  }`;
    })
    .join(",\n");

  const docPagesContent = `${imports}

export const docPages = [
${routes}
]
`;

  await fs.writeFile(
    path.join(libDir, "docPages.js"),
    docPagesContent,
    "utf-8",
  );
}

async function main() {
  try {
    console.log("🚀 Generating pages from backlog/docs and backlog/decisions...\n");

    await setupPagesDir();

    // Find docs
    const docsFiles = await findMarkdownFiles(docsDir);
    console.log(`Found ${docsFiles.length} documentation files\n`);

    // Find decisions
    const decisionsFiles = await findMarkdownFiles(decisionsDir, "decisions");
    console.log(`Found ${decisionsFiles.length} decision files\n`);

    const pages = [];
    
    // Generate doc pages
    for (const fileInfo of docsFiles) {
      const page = await generatePageFile(fileInfo, "docs");
      pages.push(page);
      console.log(`✓ Generated: ${page.path}.jsx`);
    }

    // Generate decision pages
    for (const fileInfo of decisionsFiles) {
      const page = await generatePageFile(fileInfo, "decisions");
      pages.push(page);
      console.log(`✓ Generated: ${page.path}.jsx`);
    }

    // Sort pages by doc/decision number (extract number from id like "doc-0001" or "decision-0001")
    pages.sort((a, b) => {
      const numA = parseInt(a.id?.match(/\d+/)?.[0] || "0", 10);
      const numB = parseInt(b.id?.match(/\d+/)?.[0] || "0", 10);
      return numA - numB;
    });

    await generateDocPagesFile(pages);
    await generateIndexPage();
    await generateHtmlEntries(pages);
    console.log(`\n✓ Generated docPages.js with ${pages.length} pages (sorted by number)`);
    console.log("\n✅ Page generation complete!");
  } catch (error) {
    console.error("\n❌ Error:", error.message);
    console.error(error.stack);
    process.exit(1);
  }
}

main();
