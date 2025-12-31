import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import matter from "gray-matter";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.join(__dirname, "..");
const docsDir = path.join(rootDir, "..", "backlog", "docs");
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
function generatePageComponent(fileInfo, frontMatter) {
  // Calculate relative path to DocsLayout
  const depth = fileInfo.relativePath.split(path.sep).length;
  const layoutPath = "../".repeat(depth) + "layouts/DocsLayout";
  const componentsPath = "../".repeat(depth) + "components/MDXComponents";

  const title = frontMatter.title.replace(/'/g, "\\'")
  const id = frontMatter.id || ''
  const type = frontMatter.type || ''
  const createdDate = frontMatter.created_date || ''

  return `import React from 'react'
import DocsLayout from '${layoutPath}'
import DocContent from '@docs/${fileInfo.relativePath.replace(/\\/g, "/")}.md?mdx'
import { MDXWrapper } from '${componentsPath}'

export default function Page() {
  return (
    <DocsLayout>
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

export const frontMatter = ${JSON.stringify(frontMatter, null, 2)}
`;
}

// Generate page file
async function generatePageFile(fileInfo) {
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
  const defaultTitle = fileInfo.fileName
    .replace(/^doc-\d+\s*-\s*/, "") // Remove doc-XXXX - prefix if present
    .replace(/\.md$/, "")
    .trim();

  const frontMatter = {
    id: data.id || "",
    title: data.title || defaultTitle,
    type: data.type || "general",
    created_date: data.created_date || "",
    updated_date: data.updated_date || "",
    path: fileInfo.relativePath,
  };

  const pageContent = generatePageComponent(fileInfo, frontMatter);

  // Create directory structure
  const pageDir = path.join(pagesDir, path.dirname(fileInfo.relativePath));
  await fs.mkdir(pageDir, { recursive: true });

  // Write page file
  const pageFileName = path.basename(fileInfo.relativePath) + ".jsx";
  const pageFilePath = path.join(pageDir, pageFileName);
  await fs.writeFile(pageFilePath, pageContent, "utf-8");

  // Generate route from relative path (preserve folder structure)
  const route = "/" + fileInfo.relativePath.replace(/\\/g, "/");

  return {
    route,
    path: fileInfo.relativePath,
    title: frontMatter.title,
    type: frontMatter.type,
    filePath: pageFilePath,
  };
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
    console.log("🚀 Generating pages from backlog/docs...\n");

    await setupPagesDir();

    const markdownFiles = await findMarkdownFiles(docsDir);
    console.log(`Found ${markdownFiles.length} markdown files\n`);

    const pages = [];
    for (const fileInfo of markdownFiles) {
      const page = await generatePageFile(fileInfo);
      pages.push(page);
      console.log(`✓ Generated: ${page.path}.jsx`);
    }

    await generateDocPagesFile(pages);
    console.log(`\n✓ Generated docPages.js with ${pages.length} pages`);
    console.log("\n✅ Page generation complete!");
  } catch (error) {
    console.error("\n❌ Error:", error.message);
    console.error(error.stack);
    process.exit(1);
  }
}

main();
