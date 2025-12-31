import React from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import DocsLayout from "./layouts/DocsLayout";
import { docPages } from "./lib/docPages";

function HomePage() {
  return (
    <DocsLayout>
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
                href={page.route}
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
  );
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        {docPages.map((page) => {
          const PageComponent = page.component;
          if (!PageComponent) {
            return null;
          }
          return (
            <Route
              key={page.route}
              path={page.route}
              element={<PageComponent />}
            />
          );
        })}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
