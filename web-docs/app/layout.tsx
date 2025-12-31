import { Footer, Layout, Navbar } from "nextra-theme-docs";
import { Banner, Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import "./globals.css";
import type { ReactNode } from "react";

export const metadata = {
  title: "Roro Kube Documentation",
  description: "Documentation for Roro Kube - Docker Compose for Kubernetes",
};

const banner = (
  <Banner storageKey="roro-kube-banner">
    Welcome to Roro Kube Documentation
  </Banner>
);
const navbar = <Navbar logo={<b>Roro Kube</b>} />;
const footer = <Footer>MIT {new Date().getFullYear()} © Roro Kube.</Footer>;

export default async function RootLayout({
  children,
}: {
  children: ReactNode;
}) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head>
        {/* Your additional tags should be passed as `children` of `<Head>` element */}
      </Head>
      <body>
        <Layout
          banner={banner}
          navbar={navbar}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/your-org/roro-kube/tree/main/backlog"
          footer={footer}
        >
          {children}
        </Layout>
      </body>
    </html>
  );
}
