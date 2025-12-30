import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.join(__dirname, "..");

async function checkFile(filePath, description) {
  try {
    await fs.access(filePath);
    console.log(`✓ ${description}`);
    return true;
  } catch (err) {
    console.error(`✗ Missing: ${description} (${filePath})`);
    return false;
  }
}

async function checkFileContent(filePath, description, validator) {
  try {
    const content = await fs.readFile(filePath, "utf-8");
    if (validator(content)) {
      console.log(`✓ ${description}`);
      return true;
    } else {
      console.error(`✗ Invalid: ${description} (${filePath})`);
      return false;
    }
  } catch (err) {
    console.error(`✗ Error reading: ${description} (${filePath}): ${err.message}`);
    return false;
  }
}

async function verify() {
  console.log("🔍 Verifying Tailwind CSS infrastructure\n");

  let allPassed = true;

  // Check required files
  console.log("Required Files:");
  const packageJsonPath = path.join(rootDir, "package.json");
  const tailwindConfigPath = path.join(rootDir, "tailwind.config.js");
  const postcssConfigPath = path.join(rootDir, "postcss.config.js");
  const viteConfigPath = path.join(rootDir, "vite.config.ts");
  const cssSourcePath = path.join(rootDir, "src", "tailwind.css");

  const packageJsonExists = await checkFile(
    packageJsonPath,
    "package.json"
  );
  allPassed = allPassed && packageJsonExists;

  const tailwindConfigExists = await checkFile(
    tailwindConfigPath,
    "tailwind.config.js"
  );
  allPassed = allPassed && tailwindConfigExists;

  const postcssConfigExists = await checkFile(
    postcssConfigPath,
    "postcss.config.js"
  );
  allPassed = allPassed && postcssConfigExists;

  const viteConfigExists = await checkFile(
    viteConfigPath,
    "vite.config.ts"
  );
  allPassed = allPassed && viteConfigExists;

  const cssSourceExists = await checkFile(
    cssSourcePath,
    "src/tailwind.css"
  );
  allPassed = allPassed && cssSourceExists;

  // Check package.json content
  if (packageJsonExists) {
    console.log("\npackage.json Configuration:");
    const packageJsonValid = await checkFileContent(
      packageJsonPath,
      "package.json has build script",
      (content) => {
        try {
          const pkg = JSON.parse(content);
          return pkg.scripts && pkg.scripts.build;
        } catch {
          return false;
        }
      }
    );
    allPassed = allPassed && packageJsonValid;

    const dependenciesValid = await checkFileContent(
      packageJsonPath,
      "package.json has required dependencies (tailwindcss, postcss, vite, autoprefixer)",
      (content) => {
        try {
          const pkg = JSON.parse(content);
          const hasTailwind = pkg.dependencies?.tailwindcss || pkg.devDependencies?.tailwindcss;
          const hasPostcss = pkg.dependencies?.postcss || pkg.devDependencies?.postcss;
          const hasVite = pkg.dependencies?.vite || pkg.devDependencies?.vite;
          const hasAutoprefixer = pkg.dependencies?.autoprefixer || pkg.devDependencies?.autoprefixer;
          return hasTailwind && hasPostcss && hasVite && hasAutoprefixer;
        } catch {
          return false;
        }
      }
    );
    allPassed = allPassed && dependenciesValid;
  }

  // Check tailwind.config.js content
  if (tailwindConfigExists) {
    console.log("\ntailwind.config.js Configuration:");
    const contentPathsValid = await checkFileContent(
      tailwindConfigPath,
      "tailwind.config.js includes content paths for Rust files",
      (content) => {
        return content.includes("../gui/src/**/*.rs");
      }
    );
    allPassed = allPassed && contentPathsValid;

    const darkModeValid = await checkFileContent(
      tailwindConfigPath,
      "tailwind.config.js enables dark mode with 'class' strategy",
      (content) => {
        return content.includes('darkMode: "class"') || content.includes('darkMode: "class"');
      }
    );
    allPassed = allPassed && darkModeValid;

    const themeExtendValid = await checkFileContent(
      tailwindConfigPath,
      "tailwind.config.js includes theme extension",
      (content) => {
        return content.includes("theme:") && content.includes("extend:");
      }
    );
    allPassed = allPassed && themeExtendValid;
  }

  // Check postcss.config.js content
  if (postcssConfigExists) {
    console.log("\npostcss.config.js Configuration:");
    const postcssPluginsValid = await checkFileContent(
      postcssConfigPath,
      "postcss.config.js includes Tailwind and autoprefixer plugins",
      (content) => {
        return content.includes("tailwindcss") && content.includes("autoprefixer");
      }
    );
    allPassed = allPassed && postcssPluginsValid;
  }

  // Check vite.config.ts content
  if (viteConfigExists) {
    console.log("\nvite.config.ts Configuration:");
    const viteOutputValid = await checkFileContent(
      viteConfigPath,
      "vite.config.ts outputs to ../gui/assets/tailwind.css",
      (content) => {
        return content.includes("../gui/assets") || content.includes("tailwind.css");
      }
    );
    allPassed = allPassed && viteOutputValid;
  }

  // Check src/tailwind.css content
  if (cssSourceExists) {
    console.log("\nsrc/tailwind.css Configuration:");
    const directivesValid = await checkFileContent(
      cssSourcePath,
      "src/tailwind.css includes Tailwind directives",
      (content) => {
        return (
          content.includes("@tailwind base") &&
          content.includes("@tailwind components") &&
          content.includes("@tailwind utilities")
        );
      }
    );
    allPassed = allPassed && directivesValid;
  }

  console.log("\n" + "=".repeat(50));
  if (allPassed) {
    console.log("✅ All Tailwind CSS infrastructure files verified successfully!");
    process.exit(0);
  } else {
    console.log("❌ Some Tailwind CSS infrastructure files are missing or invalid");
    process.exit(1);
  }
}

verify();

