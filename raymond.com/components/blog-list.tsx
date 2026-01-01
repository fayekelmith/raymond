"use client";

import { motion } from "framer-motion";
import Link from "next/link";
import { FileText, Calendar } from "lucide-react";

interface Post {
  title: string;
  date: string;
  description: string;
  slug: string;
}

interface BlogListProps {
  posts: Post[];
}

export function BlogList({ posts }: BlogListProps) {
  if (posts.length === 0) {
    return (
      <div className="text-center py-12">
        <FileText className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
        <p className="text-muted-foreground">No posts yet. Check back soon!</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {posts.map((post, index) => (
        <motion.div
          key={post.slug}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.4, delay: index * 0.1 }}
        >
          <Link
            href={`/blog/${post.slug}`}
            className="block p-6 rounded-lg border border-border bg-card hover-glow group"
          >
            <div className="flex items-center gap-2 text-xs text-muted-foreground mb-2 font-data">
              <Calendar className="h-3 w-3" />
              {new Date(post.date).toLocaleDateString("en-US", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </div>
            <h2 className="text-xl font-semibold mb-2 group-hover:text-primary transition-colors">
              {post.title}
            </h2>
            {post.description && (
              <p className="text-muted-foreground line-clamp-2">
                {post.description}
              </p>
            )}
          </Link>
        </motion.div>
      ))}
    </div>
  );
}
