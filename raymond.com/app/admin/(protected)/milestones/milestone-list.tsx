"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { Badge } from "@/components/ui/badge";
import { Trash2, Target } from "lucide-react";
import type { Milestone } from "@prisma/client";

interface MilestoneListProps {
  milestones: Milestone[];
}

const statusColors = {
  UPCOMING: "bg-muted text-muted-foreground",
  IN_PROGRESS: "bg-primary/20 text-primary",
  COMPLETED: "bg-green-500/20 text-green-400",
};

export function MilestoneList({ milestones }: MilestoneListProps) {
  const router = useRouter();

  async function handleDelete(id: string) {
    if (!confirm("Delete this milestone?")) return;

    await fetch(`/api/milestones/${id}`, { method: "DELETE" });
    router.refresh();
  }

  async function handleStatusChange(id: string, newStatus: string) {
    await fetch(`/api/milestones/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ status: newStatus }),
    });
    router.refresh();
  }

  if (milestones.length === 0) {
    return (
      <div className="text-center py-12">
        <Target className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
        <p className="text-muted-foreground">
          No milestones yet. Create your first goal above!
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {milestones.map((milestone) => (
        <div
          key={milestone.id}
          className="p-4 rounded-lg border border-border hover-glow bg-card"
        >
          <div className="flex items-start justify-between gap-4">
            <div className="flex-1 space-y-2">
              <div className="flex items-center gap-2 flex-wrap">
                <h3 className="font-semibold">{milestone.title}</h3>
                <Badge className={statusColors[milestone.status]}>
                  {milestone.status.replace("_", " ")}
                </Badge>
              </div>

              {milestone.description && (
                <p className="text-sm text-muted-foreground">
                  {milestone.description}
                </p>
              )}

              <div className="flex items-center gap-4">
                <div className="flex-1 max-w-xs">
                  <Progress value={milestone.progress} className="h-2" />
                </div>
                <span className="text-sm font-data text-muted-foreground">
                  {milestone.progress}%
                </span>
              </div>

              {milestone.targetDate && (
                <p className="text-xs text-muted-foreground font-data">
                  Target: {new Date(milestone.targetDate).toLocaleDateString()}
                </p>
              )}

              <div className="flex gap-2 pt-2">
                {(["UPCOMING", "IN_PROGRESS", "COMPLETED"] as const).map(
                  (s) => (
                    <Button
                      key={s}
                      variant={milestone.status === s ? "default" : "ghost"}
                      size="sm"
                      onClick={() => handleStatusChange(milestone.id, s)}
                      className="text-xs h-7"
                    >
                      {s.replace("_", " ")}
                    </Button>
                  )
                )}
              </div>
            </div>

            <Button
              variant="ghost"
              size="icon"
              onClick={() => handleDelete(milestone.id)}
              className="text-muted-foreground hover:text-destructive shrink-0"
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </div>
      ))}
    </div>
  );
}
