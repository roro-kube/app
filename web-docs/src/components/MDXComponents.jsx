import React from 'react'
import { MDXProvider } from '@mdx-js/react'

export const MDXComponents = {
  h1: (props) => <h1 className="text-3xl font-bold mt-8 mb-4" {...props} />,
  h2: (props) => <h2 className="text-2xl font-semibold mt-6 mb-3" {...props} />,
  h3: (props) => <h3 className="text-xl font-semibold mt-4 mb-2" {...props} />,
  p: (props) => <p className="mb-4 leading-7" {...props} />,
  ul: (props) => <ul className="list-disc list-inside mb-4 space-y-2" {...props} />,
  ol: (props) => <ol className="list-decimal list-inside mb-4 space-y-2" {...props} />,
  li: (props) => <li className="ml-4" {...props} />,
  code: (props) => (
    <code className="bg-gray-100 px-1.5 py-0.5 rounded text-sm font-mono" {...props} />
  ),
  pre: (props) => (
    <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto mb-4" {...props} />
  ),
  blockquote: (props) => (
    <blockquote className="border-l-4 border-gray-300 pl-4 italic my-4" {...props} />
  ),
  a: (props) => <a className="text-blue-600 hover:underline" {...props} />,
  strong: (props) => <strong className="font-semibold" {...props} />,
  em: (props) => <em className="italic" {...props} />,
}

export function MDXWrapper({ children }) {
  return <MDXProvider components={MDXComponents}>{children}</MDXProvider>
}

