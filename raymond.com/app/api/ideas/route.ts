import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/db";
import { isAuthenticated } from "@/lib/auth";

// GET all ideas
export async function GET() {
  const authenticated = await isAuthenticated();
  if (!authenticated) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  const ideas = await prisma.idea.findMany({
    orderBy: { createdAt: "desc" },
  });
  return NextResponse.json(ideas);
}

// POST new idea (public)
export async function POST(request: NextRequest) {
  try {
    const { content, author } = await request.json();

    if (!content || content.trim() === "") {
      return NextResponse.json(
        { error: "Content is required" },
        { status: 400 }
      );
    }

    const idea = await prisma.idea.create({
      data: {
        content: content.trim(),
        author: author?.trim() || null,
      },
    });

    return NextResponse.json(idea, { status: 201 });
  } catch (error) {
    console.error("Error creating idea:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
