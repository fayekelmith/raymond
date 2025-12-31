import { prisma } from "@/lib/db";
import { HeroSection } from "@/components/hero-section";
import { ProgressSection } from "@/components/progress-section";
import { StatusSection } from "@/components/status-section";
import { MilestonesSection } from "@/components/milestones-section";

export default async function HomePage() {
  const [latestStatus, milestones] = await Promise.all([
    prisma.statusUpdate.findFirst({
      orderBy: { createdAt: "desc" },
    }),
    prisma.milestone.findMany({
      orderBy: { order: "asc" },
    }),
  ]);

  const totalProgress =
    milestones.length > 0
      ? Math.round(
          milestones.reduce((acc, m) => acc + m.progress, 0) / milestones.length
        )
      : 0;

  return (
    <main className="min-h-screen">
      <HeroSection />
      <ProgressSection
        progress={totalProgress}
        milestoneCount={milestones.length}
      />
      <StatusSection status={latestStatus} />
      <MilestonesSection milestones={milestones} />
    </main>
  );
}
