import { Metadata } from "next/types";
import "./globals.css";
import { Providers } from "./providers";
import Header from "@/components/header";

export const metadata: Metadata = {
  title: "Computational Math",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ru">
      <link rel="icon" href="/favicon.ico" />
      <body>
        <main className="light text-foreground bg-background">
          <Providers>
            <Header />
            {children}
            </Providers>
        </main>
      </body>
    </html>
  );
}
