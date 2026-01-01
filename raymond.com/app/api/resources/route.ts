import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/db";
import { isAuthenticated } from "@/lib/auth";

export async function GET() {
  const resources = await prisma.resource.findMany({
    orderBy: { createdAt: "desc" },
  });
  return NextResponse.json(resources);
}

export async function POST(request: NextRequest) {
  const authenticated = await isAuthenticated();
  if (!authenticated) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  try {
    const { title, url, category, notes, difficulty } = await request.json();

    if (!title || !url || !category) {
      return NextResponse.json(
        { error: "Title, URL, and category are required" },
        { status: 400 }
      );
    }

    const resource = await prisma.resource.create({
      data: {
        title,
        url,
        category,
        notes: notes || null,
        difficulty: difficulty || null,
      },
    });

    return NextResponse.json(resource, { status: 201 });
  } catch (error) {
    console.error("Error creating resource:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
