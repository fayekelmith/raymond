import { prisma } from "@/lib/db";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { MilestoneList } from "./milestone-list";
import { MilestoneForm } from "./milestone-form";

export default async function MilestonesPage() {
  const milestones = await prisma.milestone.findMany({
    orderBy: { order: "asc" },
  });

  const stats = {
    total: milestones.length,
    upcoming: milestones.filter((m) => m.status === "UPCOMING").length,
    inProgress: milestones.filter((m) => m.status === "IN_PROGRESS").length,
    completed: milestones.filter((m) => m.status === "COMPLETED").length,
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-glow-cyan">Milestones</h1>
        <p className="text-muted-foreground">
          Define and track your project goals
        </p>
      </div>

      {/* Stats */}
      <div className="flex gap-2 flex-wrap">
        <Badge variant="outline" className="font-data">
          {stats.total} total
        </Badge>
        <Badge className="bg-muted text-muted-foreground">
          {stats.upcoming} upcoming
        </Badge>
        <Badge className="bg-primary/20 text-primary">
          {stats.inProgress} in progress
        </Badge>
        <Badge className="bg-green-500/20 text-green-400">
          {stats.completed} completed
        </Badge>
      </div>

      <Card className="hover-glow">
        <CardHeader>
          <CardTitle>New Milestone</CardTitle>
          <CardDescription>Add a new goal to track</CardDescription>
        </CardHeader>
        <CardContent>
          <MilestoneForm />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>All Milestones</CardTitle>
        </CardHeader>
        <CardContent>
          <MilestoneList milestones={milestones} />
        </CardContent>
      </Card>
    </div>
  );
}
