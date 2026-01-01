"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";

const categories = ["Docs", "Videos", "Tools", "Articles"];
const difficulties = ["Beginner", "Intermediate", "Advanced"];

export function ResourceForm() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [category, setCategory] = useState("Docs");
  const [difficulty, setDifficulty] = useState<string | null>(null);

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);

    const form = e.currentTarget;
    const formData = new FormData(form);
    const data = {
      title: formData.get("title") as string,
      url: formData.get("url") as string,
      category,
      notes: (formData.get("notes") as string) || null,
      difficulty,
    };

    try {
      const res = await fetch("/api/resources", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });

      if (res.ok) {
        form.reset();
        setCategory("Docs");
        setDifficulty(null);
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
            placeholder="Resource name"
            required
            className="bg-background"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="url">URL</Label>
          <Input
            id="url"
            name="url"
            type="url"
            placeholder="https://..."
            required
            className="bg-background"
          />
        </div>
      </div>

      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label>Category</Label>
          <div className="flex gap-2 flex-wrap">
            {categories.map((cat) => (
              <Button
                key={cat}
                type="button"
                variant={category === cat ? "default" : "outline"}
                size="sm"
                onClick={() => setCategory(cat)}
              >
                {cat}
              </Button>
            ))}
          </div>
        </div>
        <div className="space-y-2">
          <Label>Difficulty (optional)</Label>
          <div className="flex gap-2 flex-wrap">
            {difficulties.map((diff) => (
              <Button
                key={diff}
                type="button"
                variant={difficulty === diff ? "default" : "outline"}
                size="sm"
                onClick={() => setDifficulty(difficulty === diff ? null : diff)}
              >
                {diff}
              </Button>
            ))}
          </div>
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="notes">Notes (optional)</Label>
        <Textarea
          id="notes"
          name="notes"
          placeholder="Why is this resource useful?"
          className="bg-background"
        />
      </div>

      <Button type="submit" disabled={loading}>
        {loading ? "Adding..." : "Add Resource"}
      </Button>
    </form>
  );
}
