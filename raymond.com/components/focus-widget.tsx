"use client";

import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Zap } from "lucide-react";

interface FocusTask {
  id: string;
  content: string;
  isActive: boolean;
}

export function FocusWidget() {
  const [focus, setFocus] = useState<FocusTask | null>(null);
  const [isMinimized, setIsMinimized] = useState(false);

  useEffect(() => {
    fetch("/api/focus")
      .then((res) => res.json())
      .then((data) => {
        if (data && data.isActive) {
          setFocus(data);
        }
      })
      .catch(() => {});
  }, []);

  if (!focus) return null;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, x: 100 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: 100 }}
        className="fixed bottom-6 right-6 z-50"
      >
        <motion.button
          onClick={() => setIsMinimized(!isMinimized)}
          className="flex items-center gap-2 bg-card border border-border rounded-lg px-4 py-3 hover-glow cursor-pointer shadow-lg"
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
        >
          <Zap className="h-4 w-4 text-amber-400 animate-pulse" />

          <AnimatePresence mode="wait">
            {!isMinimized ? (
              <motion.div
                key="expanded"
                initial={{ width: 0, opacity: 0 }}
                animate={{ width: "auto", opacity: 1 }}
                exit={{ width: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="flex flex-col items-start">
                  <span className="text-xs text-muted-foreground font-data">
                    Currently focused on
                  </span>
                  <span className="text-sm font-medium max-w-[200px] truncate">
                    {focus.content}
                  </span>
                </div>
              </motion.div>
            ) : (
              <motion.span
                key="minimized"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="text-xs text-muted-foreground"
              >
                Focus
              </motion.span>
            )}
          </AnimatePresence>
        </motion.button>
      </motion.div>
    </AnimatePresence>
  );
}
