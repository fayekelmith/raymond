"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function QuickStatusForm() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);

    const formData = new FormData(e.currentTarget);
    const data = {
      content: formData.get("content") as string,
      blockers: (formData.get("blockers") as string) || null,
      tomorrow: (formData.get("tomorrow") as string) || null,
    };

    try {
      const res = await fetch("/api/status", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });

      if (res.ok) {
        e.currentTarget.reset();
        router.refresh();
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="content">What did you work on today?</Label>
        <Textarea
          id="content"
          name="content"
          placeholder="Today I worked on..."
          required
          className="min-h-[100px] bg-background"
        />
      </div>
      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="blockers">Any blockers? (optional)</Label>
          <Input
            id="blockers"
            name="blockers"
            placeholder="Stuck on..."
            className="bg-background"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="tomorrow">What&apos;s next? (optional)</Label>
          <Input
            id="tomorrow"
            name="tomorrow"
            placeholder="Tomorrow I'll..."
            className="bg-background"
          />
        </div>
      </div>
      <Button type="submit" disabled={loading}>
        {loading ? "Saving..." : "Log Status"}
      </Button>
    </form>
  );
}
