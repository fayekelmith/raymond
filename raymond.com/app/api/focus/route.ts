import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/db";
import { isAuthenticated } from "@/lib/auth";

// GET current active focus
export async function GET() {
  const focus = await prisma.focusTask.findFirst({
    where: { isActive: true },
    orderBy: { createdAt: "desc" },
  });
  return NextResponse.json(focus);
}

// POST new focus (clears previous)
export async function POST(request: NextRequest) {
  const authenticated = await isAuthenticated();
  if (!authenticated) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  try {
    const { content } = await request.json();

    // Deactivate all existing focus tasks
    await prisma.focusTask.updateMany({
      where: { isActive: true },
      data: { isActive: false },
    });

    if (!content || content.trim() === "") {
      // Just clearing focus
      return NextResponse.json({ cleared: true });
    }

    // Create new focus
    const focus = await prisma.focusTask.create({
      data: { content, isActive: true },
    });

    return NextResponse.json(focus, { status: 201 });
  } catch (error) {
    console.error("Error setting focus:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
