"use client";

import { motion } from "framer-motion";
import { CheckCircle2, Clock, Circle } from "lucide-react";
import { Progress } from "@/components/ui/progress";
import { Badge } from "@/components/ui/badge";
import type { Milestone } from "@prisma/client";

interface MilestonesSectionProps {
  milestones: Milestone[];
}

const statusIcons = {
  UPCOMING: Circle,
  IN_PROGRESS: Clock,
  COMPLETED: CheckCircle2,
};

const statusStyles = {
  UPCOMING: "text-muted-foreground",
  IN_PROGRESS: "text-primary",
  COMPLETED: "text-green-400",
};

export function MilestonesSection({ milestones }: MilestonesSectionProps) {
  if (milestones.length === 0) {
    return (
      <section className="py-20 px-8 border-t border-border/50">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">The Journey</h2>
          <p className="text-muted-foreground">
            Milestones will appear here as the journey unfolds.
          </p>
        </div>
      </section>
    );
  }

  return (
    <section className="py-20 px-8 border-t border-border/50">
      <div className="max-w-4xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
        >
          <h2 className="text-3xl font-bold mb-2 text-center">The Journey</h2>
          <p className="text-muted-foreground text-center mb-12">
            Milestones on the path to completion
          </p>

          <div className="space-y-6">
            {milestones.map((milestone, index) => {
              const Icon = statusIcons[milestone.status];
              return (
                <motion.div
                  key={milestone.id}
                  initial={{ opacity: 0, x: -20 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.4, delay: index * 0.1 }}
                  viewport={{ once: true }}
                  className="relative"
                >
                  {/* Timeline connector */}
                  {index < milestones.length - 1 && (
                    <div className="absolute left-[19px] top-12 w-0.5 h-[calc(100%-20px)] bg-border" />
                  )}

                  <div className="flex gap-4">
                    {/* Status icon */}
                    <div
                      className={`shrink-0 ${statusStyles[milestone.status]}`}
                    >
                      <div
                        className={`p-2 rounded-full bg-card border border-border ${
                          milestone.status === "IN_PROGRESS"
                            ? "glow-cyan-sm"
                            : ""
                        }`}
                      >
                        <Icon className="h-5 w-5" />
                      </div>
                    </div>

                    {/* Content */}
                    <div
                      className={`flex-1 p-4 rounded-lg border border-border bg-card hover-glow ${
                        milestone.status === "COMPLETED" ? "opacity-60" : ""
                      }`}
                    >
                      <div className="flex items-start justify-between gap-2 mb-2 flex-wrap">
                        <h3
                          className={`font-semibold ${
                            milestone.status === "COMPLETED"
                              ? "line-through"
                              : ""
                          }`}
                        >
                          {milestone.title}
                        </h3>
                        <Badge
                          variant="outline"
                          className={`${
                            statusStyles[milestone.status]
                          } text-xs`}
                        >
                          {milestone.status.replace("_", " ")}
                        </Badge>
                      </div>

                      {milestone.description && (
                        <p className="text-sm text-muted-foreground mb-3">
                          {milestone.description}
                        </p>
                      )}

                      <div className="flex items-center gap-3">
                        <Progress
                          value={milestone.progress}
                          className="h-2 flex-1"
                        />
                        <span className="text-sm font-data text-muted-foreground shrink-0">
                          {milestone.progress}%
                        </span>
                      </div>

                      {milestone.targetDate && (
                        <p className="text-xs text-muted-foreground mt-2 font-data">
                          Target:{" "}
                          {new Date(milestone.targetDate).toLocaleDateString()}
                        </p>
                      )}
                    </div>
                  </div>
                </motion.div>
              );
            })}
          </div>
        </motion.div>
      </div>
    </section>
  );
}
