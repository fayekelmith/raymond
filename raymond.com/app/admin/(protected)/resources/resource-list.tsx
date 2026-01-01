"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Trash2, ExternalLink, BookOpen } from "lucide-react";
import type { Resource } from "@prisma/client";

interface ResourceListProps {
  resources: Resource[];
}

const categoryColors: Record<string, string> = {
  Docs: "bg-blue-500/20 text-blue-400",
  Videos: "bg-red-500/20 text-red-400",
  Tools: "bg-green-500/20 text-green-400",
  Articles: "bg-purple-500/20 text-purple-400",
};

const difficultyColors: Record<string, string> = {
  Beginner: "bg-green-500/20 text-green-400",
  Intermediate: "bg-amber-500/20 text-amber-400",
  Advanced: "bg-red-500/20 text-red-400",
};

export function ResourceList({ resources }: ResourceListProps) {
  const router = useRouter();

  async function handleDelete(id: string) {
    if (!confirm("Delete this resource?")) return;

    await fetch(`/api/resources/${id}`, { method: "DELETE" });
    router.refresh();
  }

  if (resources.length === 0) {
    return (
      <div className="text-center py-12">
        <BookOpen className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
        <p className="text-muted-foreground">
          No resources yet. Add your first one above!
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {resources.map((resource) => (
        <div
          key={resource.id}
          className="p-4 rounded-lg border border-border hover-glow bg-card"
        >
          <div className="flex items-start justify-between gap-4">
            <div className="flex-1 space-y-2">
              <div className="flex items-center gap-2 flex-wrap">
                <a
                  href={resource.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="font-semibold hover:text-primary transition-colors flex items-center gap-1"
                >
                  {resource.title}
                  <ExternalLink className="h-3 w-3" />
                </a>
                <Badge className={categoryColors[resource.category] || ""}>
                  {resource.category}
                </Badge>
                {resource.difficulty && (
                  <Badge
                    className={difficultyColors[resource.difficulty] || ""}
                  >
                    {resource.difficulty}
                  </Badge>
                )}
              </div>

              {resource.notes && (
                <p className="text-sm text-muted-foreground">
                  {resource.notes}
                </p>
              )}

              <p className="text-xs text-muted-foreground font-data">
                Added {new Date(resource.createdAt).toLocaleDateString()}
              </p>
            </div>

            <Button
              variant="ghost"
              size="icon"
              onClick={() => handleDelete(resource.id)}
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
