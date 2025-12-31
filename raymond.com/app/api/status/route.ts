import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/db";
import { isAuthenticated } from "@/lib/auth";

export async function GET() {
  const statuses = await prisma.statusUpdate.findMany({
    orderBy: { createdAt: "desc" },
  });
  return NextResponse.json(statuses);
}

export async function POST(request: NextRequest) {
  const authenticated = await isAuthenticated();
  if (!authenticated) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  try {
    const { content, blockers, tomorrow } = await request.json();

    if (!content) {
      return NextResponse.json(
        { error: "Content is required" },
        { status: 400 }
      );
    }

    const status = await prisma.statusUpdate.create({
      data: {
        content,
        blockers: blockers || null,
        tomorrow: tomorrow || null,
      },
    });

    return NextResponse.json(status, { status: 201 });
  } catch (error) {
    console.error("Error creating status:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
