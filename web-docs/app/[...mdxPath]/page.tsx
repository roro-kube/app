import { generateStaticParamsFor, importPage } from "nextra/pages";
import { useMDXComponents as getMDXComponents } from "../../mdx-components";
import type { Metadata } from "next";
import { notFound } from "next/navigation";

export async function generateStaticParams() {
  const generateParams = generateStaticParamsFor("mdxPath");
  return await generateParams();
}

export async function generateMetadata(props: {
  params: Promise<{ mdxPath: string[] }>;
}): Promise<Metadata> {
  const params = await props.params;
  // Exclude .well-known paths (e.g., Chrome DevTools requests)
  if (params.mdxPath[0] === ".well-known") {
    notFound();
  }
  const { metadata } = await importPage(params.mdxPath);
  return metadata;
}

const Wrapper = getMDXComponents().wrapper;

export default async function Page(props: {
  params: Promise<{ mdxPath: string[] }>;
}) {
  const params = await props.params;
  // Exclude .well-known paths (e.g., Chrome DevTools requests)
  if (params.mdxPath[0] === ".well-known") {
    notFound();
  }
  const {
    default: MDXContent,
    toc,
    metadata,
    sourceCode,
  } = await importPage(params.mdxPath);
  return (
    <Wrapper toc={toc} metadata={metadata} sourceCode={sourceCode}>
      <MDXContent {...props} params={params} />
    </Wrapper>
  );
}
