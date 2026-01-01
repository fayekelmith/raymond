"use client";

import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";

const START_DATE = new Date("2026-01-01T00:00:00");
const DAILY_RATE = 0.1;

function getDaysSinceStart(): number {
  const now = new Date();
  const diffTime = now.getTime() - START_DATE.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24)) + 1; // +1 because Day 1 is Jan 1
  return Math.max(1, diffDays);
}

function calculateCompound(days: number): number {
  return Math.pow(1 + DAILY_RATE, days);
}

function formatResult(value: number): string {
  if (value >= 1e12) {
    return value.toExponential(2);
  } else if (value >= 1000) {
    return value.toLocaleString("en-US", { maximumFractionDigits: 0 });
  }
  return value.toFixed(2);
}

// Animated digit component for odometer effect
function AnimatedDigit({ digit }: { digit: string }) {
  return (
    <span className="relative inline-block overflow-hidden h-[1.2em]">
      <AnimatePresence mode="popLayout">
        <motion.span
          key={digit}
          initial={{ y: -20, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: 20, opacity: 0 }}
          transition={{ duration: 0.3, ease: "easeOut" }}
          className="inline-block"
        >
          {digit}
        </motion.span>
      </AnimatePresence>
    </span>
  );
}

// Animated result display
function AnimatedResult({ value }: { value: string }) {
  return (
    <span className="font-mono">
      {value.split("").map((char, i) => (
        <AnimatedDigit key={`${i}-${char}`} digit={char} />
      ))}
    </span>
  );
}

export function CompoundCounter() {
  const [days, setDays] = useState(1);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    setDays(getDaysSinceStart());

    // Update at midnight
    const now = new Date();
    const tomorrow = new Date(now);
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(0, 0, 0, 0);
    const msUntilMidnight = tomorrow.getTime() - now.getTime();

    const timeout = setTimeout(() => {
      setDays(getDaysSinceStart());
    }, msUntilMidnight);

    return () => clearTimeout(timeout);
  }, []);

  const result = calculateCompound(days);
  const formattedResult = formatResult(result);

  // Prevent hydration mismatch
  if (!mounted) {
    return (
      <div className="mt-8 p-4 rounded-xl bg-black/30 backdrop-blur-sm border border-primary/20">
        <div className="text-center space-y-2">
          <div className="text-sm text-muted-foreground font-mono">
            Loading...
          </div>
        </div>
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: 0.5, duration: 0.6 }}
      className="mt-8 p-4 px-6 rounded-xl bg-black/30 backdrop-blur-sm border border-primary/20 shadow-[0_0_30px_rgba(0,212,255,0.1)]"
    >
      <div className="text-center space-y-3">
        {/* Formula display */}
        <div className="text-lg md:text-xl font-mono text-muted-foreground">
          <span className="text-foreground/80">(</span>
          <span className="text-primary">1</span>
          <span className="text-foreground/80"> + </span>
          <span className="text-primary">0.1</span>
          <span className="text-foreground/80">)</span>
          <sup className="text-accent ml-0.5">
            <motion.span
              key={days}
              initial={{ scale: 1.2, color: "var(--accent)" }}
              animate={{ scale: 1, color: "var(--accent)" }}
              transition={{ duration: 0.3 }}
            >
              {days}
            </motion.span>
          </sup>
        </div>

        {/* Equals sign with glow */}
        <div className="text-2xl text-primary/60">=</div>

        {/* Result with odometer animation */}
        <div className="text-3xl md:text-4xl font-bold text-glow-cyan font-mono">
          <AnimatedResult value={formattedResult} />
          <span className="text-lg text-primary/60 ml-1">×</span>
        </div>

        {/* Day counter */}
        <div className="text-xs text-muted-foreground/60 pt-2 border-t border-primary/10">
          Day {days} of the journey
        </div>
      </div>
    </motion.div>
  );
}
