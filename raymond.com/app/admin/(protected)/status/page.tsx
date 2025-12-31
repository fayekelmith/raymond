import { prisma } from "@/lib/db";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { StatusList } from "./status-list";
import { StatusForm } from "./status-form";

export default async function StatusPage() {
  const statuses = await prisma.statusUpdate.findMany({
    orderBy: { createdAt: "desc" },
  });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-glow-cyan">Status Updates</h1>
        <p className="text-muted-foreground">Track your daily progress</p>
      </div>

      <Card className="hover-glow">
        <CardHeader>
          <CardTitle>New Status Update</CardTitle>
          <CardDescription>Log what you&apos;re working on</CardDescription>
        </CardHeader>
        <CardContent>
          <StatusForm />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>History</CardTitle>
            <Badge variant="outline" className="font-data">
              {statuses.length} entries
            </Badge>
          </div>
        </CardHeader>
        <CardContent>
          <StatusList statuses={statuses} />
        </CardContent>
      </Card>
    </div>
  );
}
