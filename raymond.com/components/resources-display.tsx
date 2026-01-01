"use client";

import { useState } from "react";
import { motion } from "framer-motion";
import { Badge } from "@/components/ui/badge";
import { ExternalLink, BookOpen, Video, Wrench, FileText } from "lucide-react";
import type { Resource } from "@prisma/client";

interface ResourcesDisplayProps {
  resources: Resource[];
}

const categories = ["All", "Docs", "Videos", "Tools", "Articles"];

const categoryIcons: Record<string, typeof BookOpen> = {
  Docs: BookOpen,
  Videos: Video,
  Tools: Wrench,
  Articles: FileText,
};

const categoryColors: Record<string, string> = {
  Docs: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  Videos: "bg-red-500/20 text-red-400 border-red-500/30",
  Tools: "bg-green-500/20 text-green-400 border-green-500/30",
  Articles: "bg-purple-500/20 text-purple-400 border-purple-500/30",
};

const difficultyColors: Record<string, string> = {
  Beginner: "bg-green-500/20 text-green-400",
  Intermediate: "bg-amber-500/20 text-amber-400",
  Advanced: "bg-red-500/20 text-red-400",
};

export function ResourcesDisplay({ resources }: ResourcesDisplayProps) {
  const [activeCategory, setActiveCategory] = useState("All");

  const filtered =
    activeCategory === "All"
      ? resources
      : resources.filter((r) => r.category === activeCategory);

  if (resources.length === 0) {
    return (
      <div className="text-center py-12">
        <BookOpen className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
        <p className="text-muted-foreground">
          Resources will appear here as the journey unfolds.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {/* Category Tabs */}
      <div className="flex gap-2 justify-center flex-wrap">
        {categories.map((cat) => (
          <button
            key={cat}
            onClick={() => setActiveCategory(cat)}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
              activeCategory === cat
                ? "bg-primary text-primary-foreground glow-cyan-sm"
                : "bg-card border border-border hover:border-primary/50"
            }`}
          >
            {cat}
          </button>
        ))}
      </div>

      {/* Resource Grid */}
      <div className="grid md:grid-cols-2 gap-4">
        {filtered.map((resource, index) => {
          const Icon = categoryIcons[resource.category] || BookOpen;
          return (
            <motion.a
              key={resource.id}
              href={resource.url}
              target="_blank"
              rel="noopener noreferrer"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: index * 0.05 }}
              className="block p-4 rounded-lg border border-border bg-card hover-glow group"
            >
              <div className="flex items-start gap-3">
                <div
                  className={`p-2 rounded-lg ${
                    categoryColors[resource.category] || "bg-muted"
                  }`}
                >
                  <Icon className="h-5 w-5" />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="font-semibold truncate group-hover:text-primary transition-colors">
                      {resource.title}
                    </h3>
                    <ExternalLink className="h-3 w-3 text-muted-foreground shrink-0" />
                  </div>
                  {resource.notes && (
                    <p className="text-sm text-muted-foreground line-clamp-2">
                      {resource.notes}
                    </p>
                  )}
                  {resource.difficulty && (
                    <Badge
                      className={`mt-2 text-xs ${
                        difficultyColors[resource.difficulty]
                      }`}
                    >
                      {resource.difficulty}
                    </Badge>
                  )}
                </div>
              </div>
            </motion.a>
          );
        })}
      </div>

      {filtered.length === 0 && (
        <p className="text-center text-muted-foreground py-8">
          No resources in this category yet.
        </p>
      )}
    </div>
  );
}
