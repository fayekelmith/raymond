import type { Metadata } from "next";
import { Unbounded, Inter, Kode_Mono } from "next/font/google";
import "./globals.css";
import { FocusWidget } from "@/components/focus-widget";
import { IdeaForm } from "@/components/idea-form";

// Headings only - bold, expressive
const unbounded = Unbounded({
  variable: "--font-unbounded",
  subsets: ["latin"],
  display: "swap",
});

// Body text - clean, readable, futuristic
const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
});

// Data, stats, code - monospace accent
const kodeMono = Kode_Mono({
  variable: "--font-kode-mono",
  subsets: ["latin"],
  display: "swap",
});

export const metadata: Metadata = {
  title: "Raymond | Progress Tracker",
  description:
    "Tracking the journey of building Raymond - an embedded Rust project",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${unbounded.variable} ${inter.variable} ${kodeMono.variable} font-sans antialiased`}
      >
        {children}
        <FocusWidget />
        <IdeaForm />
      </body>
    </html>
  );
}
