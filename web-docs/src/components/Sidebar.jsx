import React from "react";
import { Link, useLocation } from "react-router-dom";
import { docPages } from "../lib/docPages";

function Sidebar() {
  const location = useLocation();

  // Sort function to extract number from id
  const sortByNumber = (a, b) => {
    const numA = parseInt(a.id?.match(/\d+/)?.[0] || "0", 10);
    const numB = parseInt(b.id?.match(/\d+/)?.[0] || "0", 10);
    return numA - numB;
  };

  // Separate docs and decisions
  const docs = docPages.filter((page) => page.category !== "decisions").sort(sortByNumber);
  const decisions = docPages.filter((page) => page.category === "decisions").sort(sortByNumber);

  return (
    <aside className="fixed left-0 top-0 h-full w-64 bg-gray-50 border-r border-gray-200 overflow-y-auto z-10">
      <div className="p-4">
        <Link to="/" className="block mb-6">
          <h1 className="text-xl font-bold text-gray-900">Roro Kube</h1>
          <p className="text-sm text-gray-500">Documentation</p>
        </Link>
        {docPages.length > 0 ? (
          <nav className="space-y-4">
            {/* Documentation Section */}
            {docs.length > 0 && (
              <div>
                <h2 className="text-sm font-semibold text-gray-900 uppercase tracking-wide mb-2">
                  Documentation
                </h2>
                <div className="space-y-1">
                  {docs.map((page) => (
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
                </div>
              </div>
            )}

            {/* Decisions Section */}
            {decisions.length > 0 && (
              <div>
                <h2 className="text-sm font-semibold text-gray-900 uppercase tracking-wide mb-2">
                  Decisions
                </h2>
                <div className="space-y-1">
                  {decisions.map((page) => (
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
                </div>
              </div>
            )}
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
