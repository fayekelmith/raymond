import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/db";
import { isAuthenticated } from "@/lib/auth";

export async function GET() {
  const milestones = await prisma.milestone.findMany({
    orderBy: { order: "asc" },
  });
  return NextResponse.json(milestones);
}

export async function POST(request: NextRequest) {
  const authenticated = await isAuthenticated();
  if (!authenticated) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  try {
    const { title, description, progress, status, targetDate } =
      await request.json();

    if (!title) {
      return NextResponse.json({ error: "Title is required" }, { status: 400 });
    }

    // Get the highest order number
    const lastMilestone = await prisma.milestone.findFirst({
      orderBy: { order: "desc" },
    });
    const newOrder = (lastMilestone?.order ?? 0) + 1;

    const milestone = await prisma.milestone.create({
      data: {
        title,
        description: description || null,
        progress: progress ?? 0,
        status: status ?? "UPCOMING",
        targetDate: targetDate ? new Date(targetDate) : null,
        order: newOrder,
      },
    });

    return NextResponse.json(milestone, { status: 201 });
  } catch (error) {
    console.error("Error creating milestone:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
