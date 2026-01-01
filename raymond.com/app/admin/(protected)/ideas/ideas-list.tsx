"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Trash2, MessageSquare, Check } from "lucide-react";
import type { Idea } from "@prisma/client";

interface IdeasListProps {
  ideas: Idea[];
}

export function IdeasList({ ideas }: IdeasListProps) {
  const router = useRouter();

  async function toggleRead(id: string, currentlyRead: boolean) {
    await fetch(`/api/ideas/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ isRead: !currentlyRead }),
    });
    router.refresh();
  }

  async function handleDelete(id: string) {
    if (!confirm("Delete this idea?")) return;

    await fetch(`/api/ideas/${id}`, { method: "DELETE" });
    router.refresh();
  }

  if (ideas.length === 0) {
    return (
      <div className="text-center py-12">
        <MessageSquare className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
        <p className="text-muted-foreground">
          No ideas yet. They&apos;ll appear here when visitors submit them.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {ideas.map((idea) => (
        <div
          key={idea.id}
          className={`p-4 rounded-lg border transition-all ${
            idea.isRead
              ? "border-border bg-card/50"
              : "border-primary/30 bg-primary/5"
          }`}
        >
          <div className="flex items-start justify-between gap-4">
            <div
              className="flex-1 cursor-pointer"
              onClick={() => toggleRead(idea.id, idea.isRead)}
            >
              <div className="flex items-center gap-2 mb-2">
                {idea.author ? (
                  <Badge variant="secondary">{idea.author}</Badge>
                ) : (
                  <Badge variant="outline">Anonymous</Badge>
                )}
                {!idea.isRead && (
                  <Badge className="bg-primary/20 text-primary text-xs">
                    New
                  </Badge>
                )}
              </div>
              <p className="text-foreground">{idea.content}</p>
              <p className="text-xs text-muted-foreground mt-2 font-data">
                {new Date(idea.createdAt).toLocaleString()}
              </p>
            </div>

            <div className="flex gap-1 shrink-0">
              <Button
                variant="ghost"
                size="icon"
                onClick={() => toggleRead(idea.id, idea.isRead)}
                className={
                  idea.isRead ? "text-green-500" : "text-muted-foreground"
                }
              >
                <Check className="h-4 w-4" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => handleDelete(idea.id)}
                className="text-muted-foreground hover:text-destructive"
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
