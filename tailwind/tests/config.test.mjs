import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.join(__dirname, "..");

async function readFile(filePath) {
  try {
    return await fs.readFile(filePath, "utf-8");
  } catch (err) {
    throw new Error(`Failed to read ${filePath}: ${err.message}`);
  }
}

function test(name, fn) {
  try {
    fn();
    console.log(`✓ ${name}`);
    return true;
  } catch (err) {
    console.error(`✗ ${name}: ${err.message}`);
    return false;
  }
}

async function runTests() {
  console.log("🧪 Running Tailwind CSS configuration tests\n");

  let allPassed = true;

  // Test 1: Tailwind config content paths
  const tailwindConfigPath = path.join(rootDir, "tailwind.config.js");
  const tailwindConfig = await readFile(tailwindConfigPath);
  
  allPassed = test(
    "tailwind.config.js includes ../gui/src/**/*.rs in content paths",
    () => {
      if (!tailwindConfig.includes("../gui/src/**/*.rs")) {
        throw new Error("Content paths do not include ../gui/src/**/*.rs");
      }
    }
  ) && allPassed;

  // Test 2: Dark mode class strategy
  allPassed = test(
    "tailwind.config.js enables dark mode with 'class' strategy",
    () => {
      if (!tailwindConfig.includes('darkMode: "class"')) {
        throw new Error("Dark mode is not set to 'class' strategy");
      }
    }
  ) && allPassed;

  // Test 3: Theme extension exists
  allPassed = test(
    "tailwind.config.js includes theme extension",
    () => {
      if (!tailwindConfig.includes("theme:") || !tailwindConfig.includes("extend:")) {
        throw new Error("Theme extension is missing");
      }
    }
  ) && allPassed;

  // Test 4: PostCSS config includes required plugins
  const postcssConfigPath = path.join(rootDir, "postcss.config.js");
  const postcssConfig = await readFile(postcssConfigPath);
  
  allPassed = test(
    "postcss.config.js includes Tailwind plugin",
    () => {
      if (!postcssConfig.includes("tailwindcss")) {
        throw new Error("PostCSS config does not include tailwindcss plugin");
      }
    }
  ) && allPassed;

  allPassed = test(
    "postcss.config.js includes autoprefixer plugin",
    () => {
      if (!postcssConfig.includes("autoprefixer")) {
        throw new Error("PostCSS config does not include autoprefixer plugin");
      }
    }
  ) && allPassed;

  // Test 5: Vite config outputs to correct location
  const viteConfigPath = path.join(rootDir, "vite.config.ts");
  const viteConfig = await readFile(viteConfigPath);
  
  allPassed = test(
    "vite.config.ts outputs to ../gui/assets/tailwind.css",
    () => {
      if (!viteConfig.includes("../gui/assets") && !viteConfig.includes("tailwind.css")) {
        throw new Error("Vite config does not output to ../gui/assets/tailwind.css");
      }
    }
  ) && allPassed;

  // Test 6: Source CSS includes directives
  const cssSourcePath = path.join(rootDir, "src", "tailwind.css");
  const cssSource = await readFile(cssSourcePath);
  
  allPassed = test(
    "src/tailwind.css includes @tailwind base directive",
    () => {
      if (!cssSource.includes("@tailwind base")) {
        throw new Error("Missing @tailwind base directive");
      }
    }
  ) && allPassed;

  allPassed = test(
    "src/tailwind.css includes @tailwind components directive",
    () => {
      if (!cssSource.includes("@tailwind components")) {
        throw new Error("Missing @tailwind components directive");
      }
    }
  ) && allPassed;

  allPassed = test(
    "src/tailwind.css includes @tailwind utilities directive",
    () => {
      if (!cssSource.includes("@tailwind utilities")) {
        throw new Error("Missing @tailwind utilities directive");
      }
    }
  ) && allPassed;

  // Test 7: package.json has build script
  const packageJsonPath = path.join(rootDir, "package.json");
  const packageJsonContent = await readFile(packageJsonPath);
  const packageJson = JSON.parse(packageJsonContent);
  
  allPassed = test(
    "package.json includes build script",
    () => {
      if (!packageJson.scripts || !packageJson.scripts.build) {
        throw new Error("package.json does not include build script");
      }
    }
  ) && allPassed;

  console.log("\n" + "=".repeat(50));
  if (allPassed) {
    console.log("✅ All configuration tests passed!");
    process.exit(0);
  } else {
    console.log("❌ Some configuration tests failed");
    process.exit(1);
  }
}

runTests().catch((err) => {
  console.error("❌ Test execution failed:", err);
  process.exit(1);
});

