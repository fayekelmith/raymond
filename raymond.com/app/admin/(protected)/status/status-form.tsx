"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function StatusForm() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);

    const form = e.currentTarget;
    const formData = new FormData(form);
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
        form.reset();
        router.refresh();
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="content">What did you work on?</Label>
        <Textarea
          id="content"
          name="content"
          placeholder="Today I worked on..."
          required
          className="min-h-[120px] bg-background"
        />
      </div>
      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="blockers">Blockers (optional)</Label>
          <Input
            id="blockers"
            name="blockers"
            placeholder="Stuck on..."
            className="bg-background"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="tomorrow">Next steps (optional)</Label>
          <Input
            id="tomorrow"
            name="tomorrow"
            placeholder="Tomorrow I'll..."
            className="bg-background"
          />
        </div>
      </div>
      <Button type="submit" disabled={loading}>
        {loading ? "Saving..." : "Save Status"}
      </Button>
    </form>
  );
}
