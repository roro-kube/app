# Roro Kube Web Documentation

Documentation site for Roro Kube, built with Vite, React, and MDX.

## Local Development

### Quick Start (using mise)

1. Build assets (icons and Tailwind CSS):
   ```bash
   mise run web-docs-build-full
   ```

2. Copy assets to web-docs/public:
   ```bash
   mkdir -p web-docs/public
   cp -r gui/assets/* web-docs/public/
   ```

3. Install web-docs dependencies:
   ```bash
   mise run web-docs-install
   ```

4. Start dev server:
   ```bash
   cd web-docs && npm run dev
   ```

The dev server will automatically:
- Generate pages from `backlog/docs/`
- Start Vite dev server (usually at http://localhost:5173)

### Manual Steps

If you prefer to run steps individually:

1. Build icons:
   ```bash
   mise run icons-build
   ```

2. Build Tailwind CSS:
   ```bash
   mise run tailwind-build
   ```

3. Copy assets:
   ```bash
   mkdir -p web-docs/public
   cp -r gui/assets/* web-docs/public/
   ```

4. Install dependencies:
   ```bash
   cd web-docs && npm install
   ```

5. Generate pages:
   ```bash
   cd web-docs && npm run generate
   ```

6. Start dev server:
   ```bash
   cd web-docs && npm run dev
   ```

## Build

Build the documentation site for production:
```bash
cd web-docs && npm run build
```

This will:
1. Generate page files from `backlog/docs/`
2. Build the site with Vite
3. Output static files to `dist/`

## Notes

- Assets must be copied to `web-docs/public/` before running the dev server
- The `generate` script runs automatically before `dev` and `build`
- Generated files in `src/pages/` are not committed to git (see `.gitignore`)

## Structure

- `src/pages/` - Generated page components (not committed to git)
- `src/layouts/` - Layout components
- `src/components/` - Reusable components
- `src/lib/` - Utility functions and generated page index
- `scripts/generate-pages.mjs` - Pre-build script that generates pages from markdown

## Assets

Assets are copied from `gui/assets/` during the GitHub Actions build process. This includes:
- Tailwind CSS (`tailwind.css`)
- Icons (`icons/`)
- Branding assets (`branding/`)

## Source of Truth

The documentation source files are in `backlog/docs/`. The site reads from these files and generates static pages.

