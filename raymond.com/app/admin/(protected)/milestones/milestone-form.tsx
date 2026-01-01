"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { format } from "date-fns";
import { Calendar as CalendarIcon } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

export function MilestoneForm() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [date, setDate] = useState<Date>();
  const [status, setStatus] = useState<
    "UPCOMING" | "IN_PROGRESS" | "COMPLETED"
  >("UPCOMING");

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);

    const form = e.currentTarget; // Capture form reference
    const formData = new FormData(form);

    // Format date specifically for API: YYYY-MM-DD
    const targetDate = date ? format(date, "yyyy-MM-dd") : null;

    const data = {
      title: formData.get("title") as string,
      description: (formData.get("description") as string) || null,
      progress,
      status,
      targetDate,
    };

    try {
      const res = await fetch("/api/milestones", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });

      if (res.ok) {
        form.reset(); // Use captured reference
        setProgress(0);
        setStatus("UPCOMING");
        setDate(undefined);
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
          <Label>Target Date (optional)</Label>
          <Popover>
            <PopoverTrigger asChild>
              <Button
                variant={"outline"}
                className={cn(
                  "w-full justify-start text-left font-normal bg-background",
                  !date && "text-muted-foreground"
                )}
              >
                <CalendarIcon className="mr-2 h-4 w-4" />
                {date ? format(date, "PPP") : <span>Pick a date</span>}
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-auto p-0" align="start">
              <Calendar
                mode="single"
                selected={date}
                onSelect={setDate}
                initialFocus
                className="pointer-events-auto"
              />
            </PopoverContent>
          </Popover>
          {/* Hidden input to ensure logic flow remains similar if needed, 
              but we are handling date via state now */}
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
