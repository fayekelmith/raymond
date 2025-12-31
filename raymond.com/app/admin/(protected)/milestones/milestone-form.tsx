"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";

export function MilestoneForm() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [status, setStatus] = useState<
    "UPCOMING" | "IN_PROGRESS" | "COMPLETED"
  >("UPCOMING");

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);

    const formData = new FormData(e.currentTarget);
    const data = {
      title: formData.get("title") as string,
      description: (formData.get("description") as string) || null,
      progress,
      status,
      targetDate: (formData.get("targetDate") as string) || null,
    };

    try {
      const res = await fetch("/api/milestones", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });

      if (res.ok) {
        e.currentTarget.reset();
        setProgress(0);
        setStatus("UPCOMING");
        router.refresh();
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="title">Title</Label>
          <Input
            id="title"
            name="title"
            placeholder="Milestone name"
            required
            className="bg-background"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="targetDate">Target Date (optional)</Label>
          <Input
            id="targetDate"
            name="targetDate"
            type="date"
            className="bg-background"
          />
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="description">Description (optional)</Label>
        <Textarea
          id="description"
          name="description"
          placeholder="What does this milestone involve?"
          className="bg-background"
        />
      </div>

      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label>Progress: {progress}%</Label>
          <Slider
            value={[progress]}
            onValueChange={(v) => setProgress(v[0])}
            max={100}
            step={5}
            className="py-2"
          />
        </div>
        <div className="space-y-2">
          <Label>Status</Label>
          <div className="flex gap-2">
            {(["UPCOMING", "IN_PROGRESS", "COMPLETED"] as const).map((s) => (
              <Button
                key={s}
                type="button"
                variant={status === s ? "default" : "outline"}
                size="sm"
                onClick={() => setStatus(s)}
                className={status === s ? "" : "text-muted-foreground"}
              >
                {s.replace("_", " ")}
              </Button>
            ))}
          </div>
        </div>
      </div>

      <Button type="submit" disabled={loading}>
        {loading ? "Creating..." : "Create Milestone"}
      </Button>
    </form>
  );
}
