import { prisma } from "@/lib/db";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { QuickStatusForm } from "./quick-status-form";
import { FocusInput } from "@/components/focus-input";
import { GlitchText } from "@/components/glitch-text";

export default async function AdminDashboard() {
  const [latestStatus, milestones, currentFocus] = await Promise.all([
    prisma.statusUpdate.findFirst({
      orderBy: { createdAt: "desc" },
    }),
    prisma.milestone.findMany({
      orderBy: { order: "asc" },
    }),
    prisma.focusTask.findFirst({
      where: { isActive: true },
      orderBy: { createdAt: "desc" },
    }),
  ]);

  const totalProgress =
    milestones.length > 0
      ? Math.round(
          milestones.reduce((acc, m) => acc + m.progress, 0) / milestones.length
        )
      : 0;

  const inProgressCount = milestones.filter(
    (m) => m.status === "IN_PROGRESS"
  ).length;
  const completedCount = milestones.filter(
    (m) => m.status === "COMPLETED"
  ).length;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-glow-cyan">
          <GlitchText text="Mission Overview" />
        </h1>
        <p className="text-muted-foreground">Welcome to Mission Control</p>
      </div>

      {/* Quick Stats */}
      <div className="grid gap-4 md:grid-cols-3">
        <Card className="hover-glow">
          <CardHeader className="pb-2">
            <CardDescription>Overall Progress</CardDescription>
            <CardTitle className="text-4xl font-data">
              {totalProgress}%
            </CardTitle>
          </CardHeader>
          <CardContent>
            <Progress value={totalProgress} className="h-2" />
          </CardContent>
        </Card>

        <Card className="hover-glow">
          <CardHeader className="pb-2">
            <CardDescription>In Progress</CardDescription>
            <CardTitle className="text-4xl font-data">
              {inProgressCount}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">Active milestones</p>
          </CardContent>
        </Card>

        <Card className="hover-glow">
          <CardHeader className="pb-2">
            <CardDescription>Completed</CardDescription>
            <CardTitle className="text-4xl font-data">
              {completedCount}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">Milestones achieved</p>
          </CardContent>
        </Card>
      </div>

      {/* Current Focus */}
      <Card className="hover-glow border-amber-500/30">
        <CardHeader className="pb-2">
          <CardTitle className="flex items-center gap-2 text-lg">
            <span className="text-amber-400">⚡</span> Current Focus
          </CardTitle>
          <CardDescription>What are you working on right now?</CardDescription>
        </CardHeader>
        <CardContent>
          <FocusInput currentFocus={currentFocus?.content} />
        </CardContent>
      </Card>

      {/* Quick Status Update */}
      <Card className="hover-glow">
        <CardHeader>
          <CardTitle>Quick Status Update</CardTitle>
          <CardDescription>Log what you worked on today</CardDescription>
        </CardHeader>
        <CardContent>
          <QuickStatusForm />
        </CardContent>
      </Card>

      {/* Latest Status */}
      {latestStatus && (
        <Card className="hover-glow">
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Latest Status</CardTitle>
              <Badge variant="outline" className="font-data">
                {new Date(latestStatus.createdAt).toLocaleDateString()}
              </Badge>
            </div>
          </CardHeader>
          <CardContent className="space-y-2">
            <p>{latestStatus.content}</p>
            {latestStatus.blockers && (
              <p className="text-sm text-amber-400">
                <span className="font-medium">Blockers:</span>{" "}
                {latestStatus.blockers}
              </p>
            )}
            {latestStatus.tomorrow && (
              <p className="text-sm text-muted-foreground">
                <span className="font-medium">Next:</span>{" "}
                {latestStatus.tomorrow}
              </p>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
