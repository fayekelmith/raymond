"use client";

import { motion } from "framer-motion";
import Link from "next/link";
import { Rocket, ArrowDown } from "lucide-react";

export function HeroSection() {
  return (
    <section className="relative min-h-screen flex flex-col items-center justify-center p-8 overflow-hidden">
      {/* Background grid effect */}
      <div
        className="absolute inset-0 opacity-[0.03]"
        style={{
          backgroundImage: `
            linear-gradient(rgba(0, 212, 255, 0.5) 1px, transparent 1px),
            linear-gradient(90deg, rgba(0, 212, 255, 0.5) 1px, transparent 1px)
          `,
          backgroundSize: "50px 50px",
        }}
      />

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
        className="text-center z-10"
      >
        <motion.div
          initial={{ scale: 0.8 }}
          animate={{ scale: 1 }}
          transition={{ duration: 0.5, delay: 0.2 }}
          className="mb-8"
        >
          <Rocket className="h-20 w-20 mx-auto text-primary glow-cyan rounded-full p-4 bg-primary/10" />
        </motion.div>

        <h1 className="text-5xl md:text-7xl font-bold mb-4 text-glow-cyan">
          Raymond
        </h1>

        <p className="text-xl md:text-2xl font-semibold text-muted-foreground mb-2 max-w-2xl mx-auto">
          An Embedded Rust Journey
        </p>

        <p className="text-sm text-muted-foreground mb-8 font-data">
          Building something cool, one commit at a time
        </p>
      </motion.div>

      {/* Scroll indicator */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 1.5 }}
        className="absolute bottom-8"
      >
        <motion.div
          animate={{ y: [0, 10, 0] }}
          transition={{ repeat: Infinity, duration: 2 }}
        >
          <ArrowDown className="h-6 w-6 text-muted-foreground" />
        </motion.div>
      </motion.div>

      {/* Hidden admin access - bottom right corner */}
      <Link
        href="/admin/login"
        className="absolute bottom-4 right-4 w-8 h-8 opacity-0 hover:opacity-10 transition-opacity"
        aria-label="Admin access"
      />
    </section>
  );
}
