import { getPostBySlug, getAllPosts } from "@/lib/blog";
import { MDXRemote } from "next-mdx-remote/rsc";
import { notFound } from "next/navigation";
import Link from "next/link";
import { ArrowLeft, Calendar, Home } from "lucide-react";
import rehypePrettyCode from "rehype-pretty-code";

export function generateStaticParams() {
  const posts = getAllPosts();
  return posts.map((post) => ({ slug: post.slug }));
}

const options = {
  theme: "github-dark",
  keepBackground: true,
};

export default async function BlogPost({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const post = getPostBySlug(slug);

  if (!post) {
    notFound();
  }

  return (
    <main className="min-h-screen py-16 px-6 md:px-8">
      <article className="max-w-3xl mx-auto">
        {/* Navigation */}
        <nav className="flex items-center gap-4 mb-10">
          <Link
            href="/"
            className="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-primary transition-colors"
          >
            <Home className="h-4 w-4" />
            Home
          </Link>
          <span className="text-muted-foreground/50">/</span>
          <Link
            href="/blog"
            className="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-primary transition-colors"
          >
            <ArrowLeft className="h-4 w-4" />
            Mission Log
          </Link>
        </nav>

        {/* Header */}
        <header className="mb-10">
          <div className="flex items-center gap-2 text-sm text-muted-foreground mb-3 font-data">
            <Calendar className="h-4 w-4" />
            {new Date(post.date).toLocaleDateString("en-US", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </div>
          <h1 className="text-3xl md:text-4xl font-bold text-glow-cyan mb-4 leading-tight">
            {post.title}
          </h1>
          {post.description && (
            <p className="text-lg text-muted-foreground leading-relaxed">
              {post.description}
            </p>
          )}
        </header>

        {/* Content */}
        <div
          className="prose prose-invert prose-cyan max-w-none 
          prose-headings:mt-8 prose-headings:mb-4 prose-headings:font-bold
          prose-h1:text-2xl prose-h2:text-xl prose-h3:text-lg
          prose-p:mb-4 prose-p:leading-relaxed
          prose-ul:my-4 prose-ol:my-4 prose-li:my-1
          prose-strong:text-foreground
          [&_pre]:rounded-lg [&_pre]:p-4 [&_pre]:my-6 [&_pre]:overflow-x-auto [&_pre]:bg-card [&_pre]:border [&_pre]:border-border
          [&_code]:font-data [&_code]:text-sm
          [&_:not(pre)>code]:bg-card [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:rounded"
        >
          <MDXRemote
            source={post.content}
            options={{
              mdxOptions: {
                rehypePlugins: [[rehypePrettyCode, options]],
              },
            }}
          />
        </div>
      </article>
    </main>
  );
}
