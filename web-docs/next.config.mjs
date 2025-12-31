import nextra from 'nextra'
 
/**
 * @type {import('next').NextConfig}
 */
// Read BASE_PATH from environment variable, default to empty string for local development
// Remove trailing slash if present (Next.js basePath should not have trailing slash)
const basePath = process.env.BASE_PATH 
  ? process.env.BASE_PATH.replace(/\/$/, '') 
  : ''

const nextConfig = {
  output: 'export',
  basePath: basePath,
  images: {
    unoptimized: true // mandatory, otherwise won't export
  }
  // Optional: Change the output directory `out` -> `dist`
  // distDir: "build"
}
const withNextra = nextra({
  // ... other Nextra config options
})
 
export default withNextra(nextConfig)