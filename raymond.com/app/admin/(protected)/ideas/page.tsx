import { prisma } from "@/lib/db";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { IdeasList } from "./ideas-list";

export default async function IdeasPage() {
  const ideas = await prisma.idea.findMany({
    orderBy: { createdAt: "desc" },
  });

  const unreadCount = ideas.filter((i) => !i.isRead).length;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-glow-cyan">
            Ideas & Feedback
          </h1>
          <p className="text-muted-foreground">
            Visitor suggestions and questions
          </p>
        </div>
        <div className="flex gap-2">
          <Badge variant="outline" className="font-data">
            {ideas.length} total
          </Badge>
          {unreadCount > 0 && (
            <Badge className="bg-primary/20 text-primary font-data">
              {unreadCount} new
            </Badge>
          )}
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>All Ideas</CardTitle>
          <CardDescription>Click to mark as read</CardDescription>
        </CardHeader>
        <CardContent>
          <IdeasList ideas={ideas} />
        </CardContent>
      </Card>
    </div>
  );
}
