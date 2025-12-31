import nextra from 'nextra'
import type { NextConfig } from 'next'

const withNextra = nextra({
})

const config: NextConfig = withNextra({
  output: 'export',
  basePath: process.env.BASE_PATH || '',
})

export default config

