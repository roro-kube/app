import React from "react";
import Sidebar from "../components/Sidebar";

export default function DocsLayout({ children, currentPath = "/" }) {
  return (
    <div className="flex min-h-screen bg-white">
      <Sidebar currentPath={currentPath} />
      <main className="flex-1 lg:ml-64">
        <div className="max-w-4xl mx-auto px-6 py-8">{children}</div>
      </main>
    </div>
  );
}
