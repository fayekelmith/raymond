import { prisma } from "@/lib/db";
import { ResourcesDisplay } from "@/components/resources-display";
import Link from "next/link";
import { Home } from "lucide-react";

export default async function ResourcesPage() {
  const resources = await prisma.resource.findMany({
    orderBy: { createdAt: "desc" },
  });

  return (
    <main className="min-h-screen py-16 px-6 md:px-8">
      <div className="max-w-4xl mx-auto">
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
            Resource Library
          </h1>
          <p className="text-muted-foreground">
            Curated materials for learning embedded Rust
          </p>
        </header>

        <ResourcesDisplay resources={resources} />
      </div>
    </main>
  );
}
