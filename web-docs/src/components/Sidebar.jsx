import React from "react";
import { Link, useLocation } from "react-router-dom";
import { docPages } from "../lib/docPages";

function Sidebar() {
  const location = useLocation();

  return (
    <aside className="fixed left-0 top-0 h-full w-64 bg-gray-50 border-r border-gray-200 overflow-y-auto z-10">
      <div className="p-4">
        <Link to="/" className="block mb-6">
          <h1 className="text-xl font-bold text-gray-900">Roro Kube</h1>
          <p className="text-sm text-gray-500">Documentation</p>
        </Link>
        {docPages.length > 0 ? (
          <nav className="space-y-1">
            {docPages
              .slice()
              .sort((a, b) => {
                // Extract doc number from id (e.g., "doc-0001" -> 1)
                const numA = parseInt(a.id?.match(/\d+/)?.[0] || "0", 10);
                const numB = parseInt(b.id?.match(/\d+/)?.[0] || "0", 10);
                return numA - numB;
              })
              .map((page) => (
              <Link
                key={page.route}
                to={page.route}
                className={`block py-2 px-3 rounded ${
                  location.pathname === page.route
                    ? "bg-blue-50 text-blue-700 font-medium"
                    : "text-gray-600 hover:bg-gray-100"
                }`}
              >
                {page.title}
              </Link>
            ))}
          </nav>
        ) : (
          <p className="text-sm text-gray-500">
            Run{" "}
            <code className="bg-gray-200 px-1 rounded">npm run generate</code>{" "}
            to generate pages.
          </p>
        )}
      </div>
    </aside>
  );
}

export default Sidebar;
