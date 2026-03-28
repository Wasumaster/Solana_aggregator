import type { Metadata } from "next";
import "./globals.css";
import AppWalletProvider from "./components/AppWalletProvider";
import Navbar from "./components/Navbar";

export const metadata: Metadata = {
  title: "SolBalance",
  description: "Delta-Neutral Yield Harvester",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-[#050505] text-neutral-300 font-sans selection:bg-neutral-800">
        <AppWalletProvider>
          <Navbar />
          {children}
        </AppWalletProvider>
      </body>
    </html>
  );
}
