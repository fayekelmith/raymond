import { getAllPosts } from "@/lib/blog";
import { BlogList } from "@/components/blog-list";
import Link from "next/link";
import { Home } from "lucide-react";

export default function BlogPage() {
  const posts = getAllPosts();

  return (
    <main className="min-h-screen py-16 px-6 md:px-8">
      <div className="max-w-3xl mx-auto">
        {/* Navigation */}
        <nav className="flex items-center gap-2 mb-10">
          <Link
            href="/"
            className="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-primary transition-colors"
          >
            <Home className="h-4 w-4" />
            Home
          </Link>
        </nav>

        {/* Header */}
        <header className="text-center mb-12">
          <h1 className="text-4xl font-bold text-glow-cyan mb-3">
            Mission Log
          </h1>
          <p className="text-muted-foreground">
            Documenting the journey of building Raymond
          </p>
        </header>

        <BlogList posts={posts} />
      </div>
    </main>
  );
}
