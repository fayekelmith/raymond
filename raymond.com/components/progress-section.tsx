"use client";

import { motion } from "framer-motion";
import { Progress } from "@/components/ui/progress";

interface ProgressSectionProps {
  progress: number;
  milestoneCount: number;
}

export function ProgressSection({
  progress,
  milestoneCount,
}: ProgressSectionProps) {
  return (
    <section className="py-20 px-8 border-t border-border/50">
      <div className="max-w-4xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="text-center"
        >
          <h2 className="text-3xl font-bold mb-2">Mission Progress</h2>
          <p className="text-muted-foreground mb-8">
            Tracking {milestoneCount} milestone{milestoneCount !== 1 ? "s" : ""}
          </p>

          <div className="relative">
            <div className="text-6xl md:text-8xl font-bold font-data text-glow-cyan mb-4">
              {progress}%
            </div>

            <div className="max-w-md mx-auto">
              <Progress value={progress} className="h-3 glow-cyan-sm" />
            </div>

            <p className="text-sm text-muted-foreground mt-4">
              {progress < 25 && "Just getting started..."}
              {progress >= 25 && progress < 50 && "Making good progress!"}
              {progress >= 50 && progress < 75 && "Over halfway there!"}
              {progress >= 75 && progress < 100 && "Almost at the finish line!"}
              {progress === 100 && "Mission accomplished! 🎉"}
            </p>
          </div>
        </motion.div>
      </div>
    </section>
  );
}
