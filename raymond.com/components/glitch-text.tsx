"use client";

import { motion } from "framer-motion";

interface GlitchTextProps {
  text: string;
  className?: string;
}

export function GlitchText({ text, className = "" }: GlitchTextProps) {
  return (
    <div className={`relative inline-block group ${className}`}>
      <span className="relative z-10">{text}</span>
      <motion.span
        className="absolute top-0 left-0 -z-10 w-full h-full text-primary opacity-0 group-hover:opacity-70"
        animate={{
          x: [-2, 2, -1, 0],
          y: [1, -1, 0],
        }}
        transition={{
          repeat: Infinity,
          repeatType: "mirror",
          duration: 0.2,
          repeatDelay: 0.5,
        }}
      >
        {text}
      </motion.span>
      <motion.span
        className="absolute top-0 left-0 -z-10 w-full h-full text-accent opacity-0 group-hover:opacity-70"
        animate={{
          x: [2, -2, 1, 0],
          y: [-1, 1, 0],
        }}
        transition={{
          repeat: Infinity,
          repeatType: "mirror",
          duration: 0.3,
          repeatDelay: 0.5,
        }}
      >
        {text}
      </motion.span>
    </div>
  );
}
