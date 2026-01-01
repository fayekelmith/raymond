import { prisma } from "@/lib/db";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ResourceList } from "./resource-list";
import { ResourceForm } from "./resource-form";

export default async function ResourcesPage() {
  const resources = await prisma.resource.findMany({
    orderBy: { createdAt: "desc" },
  });

  const categories = ["Docs", "Videos", "Tools", "Articles"];
  const categoryCounts = categories.reduce((acc, cat) => {
    acc[cat] = resources.filter((r) => r.category === cat).length;
    return acc;
  }, {} as Record<string, number>);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-glow-cyan">Resources</h1>
        <p className="text-muted-foreground">
          Curate useful learning materials
        </p>
      </div>

      {/* Category Stats */}
      <div className="flex gap-2 flex-wrap">
        <Badge variant="outline" className="font-data">
          {resources.length} total
        </Badge>
        {categories.map((cat) => (
          <Badge key={cat} variant="secondary">
            {cat}: {categoryCounts[cat]}
          </Badge>
        ))}
      </div>

      <Card className="hover-glow">
        <CardHeader>
          <CardTitle>Add Resource</CardTitle>
          <CardDescription>Add a new link to your library</CardDescription>
        </CardHeader>
        <CardContent>
          <ResourceForm />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>All Resources</CardTitle>
        </CardHeader>
        <CardContent>
          <ResourceList resources={resources} />
        </CardContent>
      </Card>
    </div>
  );
}
