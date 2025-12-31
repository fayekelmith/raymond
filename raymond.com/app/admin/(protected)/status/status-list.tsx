"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Trash2 } from "lucide-react";
import type { StatusUpdate } from "@prisma/client";

interface StatusListProps {
  statuses: StatusUpdate[];
}

export function StatusList({ statuses }: StatusListProps) {
  const router = useRouter();

  async function handleDelete(id: string) {
    if (!confirm("Delete this status update?")) return;

    await fetch(`/api/status/${id}`, { method: "DELETE" });
    router.refresh();
  }

  if (statuses.length === 0) {
    return (
      <p className="text-muted-foreground text-center py-8">
        No status updates yet. Create your first one above!
      </p>
    );
  }

  return (
    <div className="space-y-4">
      {statuses.map((status) => (
        <div
          key={status.id}
          className="p-4 rounded-lg border border-border hover-glow bg-card"
        >
          <div className="flex items-start justify-between gap-4">
            <div className="flex-1 space-y-1">
              <p className="font-medium">{status.content}</p>
              {status.blockers && (
                <p className="text-sm text-amber-400">
                  <span className="font-medium">Blockers:</span>{" "}
                  {status.blockers}
                </p>
              )}
              {status.tomorrow && (
                <p className="text-sm text-muted-foreground">
                  <span className="font-medium">Next:</span> {status.tomorrow}
                </p>
              )}
              <p className="text-xs text-muted-foreground font-data">
                {new Date(status.createdAt).toLocaleString()}
              </p>
            </div>
            <Button
              variant="ghost"
              size="icon"
              onClick={() => handleDelete(status.id)}
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
