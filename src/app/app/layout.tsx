import type { Metadata } from "next";
import { Inter, Patrick_Hand } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });
const patrick = Patrick_Hand({ weight: "400", subsets: ["latin"], variable: "--font-patrick" });

export const metadata: Metadata = {
  title: "SlapWin — The Funniest Windows Utility You'll Ever Use",
  description: "Slap your laptop, get a scream. SlapWin is the viral Windows utility that turns every desk slap into comedy gold. Under 10MB.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={`${inter.variable} ${patrick.variable}`}>
      <body>{children}</body>
    </html>
  );
}
