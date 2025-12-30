import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.join(__dirname, "..");
const heroiconsDir = path.join(rootDir, "node_modules", "heroicons");
const iconsOutputDir = path.join(rootDir, "..", "gui", "assets", "icons");
const brandingOutputDir = path.join(rootDir, "..", "gui", "assets", "branding");
const distDir = path.join(rootDir, "dist");

const iconMap = {
  home: "home",
  upload: "arrow-up-tray",
  workflows: "squares-2x2",
  executions: "clock",
  prompts: "chat-bubble-left-right",
  settings: "cog-6-tooth",
  plus: "plus",
  x: "x-mark",
  chevron_left: "chevron-left",
  chevron_right: "chevron-right",
  arrow_left: "arrow-left",
  trash: "trash",
  exclamation_circle: "exclamation-circle",
  play: "play",
  stop: "stop",
  bars_3: "bars-3",
  check_circle: "check-circle",
  pause: "pause",
  copy: "document-duplicate",
  save: "arrow-down-on-square",
  code_bracket: "code-bracket",
  terminal: "command-line",
  cursor: "cursor-arrow-rays",
};

if (!fs.existsSync(iconsOutputDir)) {
  fs.mkdirSync(iconsOutputDir, { recursive: true });
}

console.log("Copying Heroicons...");

Object.entries(iconMap).forEach(([ourName, heroiconName]) => {
  const outlineSrc = path.join(
    heroiconsDir,
    "24",
    "outline",
    `${heroiconName}.svg`,
  );
  const outlineDest = path.join(iconsOutputDir, `${ourName}-outline.svg`);
  if (fs.existsSync(outlineSrc)) {
    fs.copyFileSync(outlineSrc, outlineDest);
    console.log(`✓ Copied ${ourName}-outline.svg`);
  } else {
    console.log(`✗ Missing: ${outlineSrc}`);
  }
});

console.log("\nCopying branding icons...");

if (!fs.existsSync(brandingOutputDir)) {
  fs.mkdirSync(brandingOutputDir, { recursive: true });
}

// Copy source SVG
const svgSrc = path.join(rootDir, "branding", "logo.svg");
const svgDest = path.join(brandingOutputDir, "logo.svg");
if (fs.existsSync(svgSrc)) {
  fs.copyFileSync(svgSrc, svgDest);
  console.log("✓ Copied logo.svg");
} else {
  console.log("✗ Missing: branding/logo.svg");
}

// Copy generated PNGs if they exist
const pngSizes = [32, 64, 128, 256];
for (const size of pngSizes) {
  const pngSrc = path.join(distDir, "png", size.toString(), "logo.png");
  const pngDest = path.join(brandingOutputDir, `logo-${size}.png`);
  if (fs.existsSync(pngSrc)) {
    fs.copyFileSync(pngSrc, pngDest);
    console.log(`✓ Copied logo-${size}.png`);
  } else {
    console.log(`⚠ Skipping logo-${size}.png (not generated yet - run mise run icons-build-branding first)`);
  }
}

// Copy bundle icons (for app packaging)
console.log("\nCopying bundle icons...");

// Copy .icns for macOS
const icnsSrc = path.join(distDir, "macos", "logo.icns");
const icnsDest = path.join(brandingOutputDir, "logo.icns");
if (fs.existsSync(icnsSrc)) {
  fs.copyFileSync(icnsSrc, icnsDest);
  console.log("✓ Copied logo.icns (macOS)");
} else {
  console.log("⚠ Skipping logo.icns (not generated yet)");
}

// Copy .ico for Windows
const icoSrc = path.join(distDir, "windows", "logo.ico");
const icoDest = path.join(brandingOutputDir, "logo.ico");
if (fs.existsSync(icoSrc)) {
  fs.copyFileSync(icoSrc, icoDest);
  console.log("✓ Copied logo.ico (Windows)");
} else {
  console.log("⚠ Skipping logo.ico (not generated yet)");
}

console.log("\nIcon copying complete!");

