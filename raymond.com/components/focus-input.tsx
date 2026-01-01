"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Zap, X } from "lucide-react";

interface FocusInputProps {
  currentFocus?: string | null;
}

export function FocusInput({ currentFocus }: FocusInputProps) {
  const router = useRouter();
  const [content, setContent] = useState(currentFocus || "");
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);

    try {
      await fetch("/api/focus", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ content: content.trim() }),
      });
      router.refresh();
    } finally {
      setLoading(false);
    }
  }

  async function handleClear() {
    setLoading(true);
    try {
      await fetch("/api/focus", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ content: "" }),
      });
      setContent("");
      router.refresh();
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} className="flex gap-2">
      <div className="relative flex-1">
        <Zap className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-amber-400" />
        <Input
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="What are you focused on?"
          className="pl-10 bg-background"
        />
      </div>
      <Button type="submit" disabled={loading} size="sm">
        {loading ? "..." : "Set"}
      </Button>
      {currentFocus && (
        <Button
          type="button"
          variant="ghost"
          size="icon"
          onClick={handleClear}
          disabled={loading}
          className="text-muted-foreground hover:text-destructive"
        >
          <X className="h-4 w-4" />
        </Button>
      )}
    </form>
  );
}
