"use client";

import { motion } from "framer-motion";
import { MessageSquare, AlertCircle, ArrowRight } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import type { StatusUpdate } from "@prisma/client";

interface StatusSectionProps {
  status: StatusUpdate | null;
}

export function StatusSection({ status }: StatusSectionProps) {
  if (!status) {
    return (
      <section className="py-20 px-8 border-t border-border/50">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">Where I Am</h2>
          <p className="text-muted-foreground">
            No status updates yet. Check back soon!
          </p>
        </div>
      </section>
    );
  }

  return (
    <section className="py-20 px-8 border-t border-border/50">
      <div className="max-w-4xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
        >
          <h2 className="text-3xl font-bold mb-8 text-center">Where I Am</h2>

          <Card className="hover-glow">
            <CardHeader>
              <div className="flex items-center justify-between flex-wrap gap-2">
                <CardTitle className="flex items-center gap-2">
                  <MessageSquare className="h-5 w-5 text-primary" />
                  Latest Update
                </CardTitle>
                <Badge variant="outline" className="font-data">
                  {new Date(status.createdAt).toLocaleDateString("en-US", {
                    weekday: "short",
                    month: "short",
                    day: "numeric",
                  })}
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-lg">{status.content}</p>

              {status.blockers && (
                <div className="flex items-start gap-2 p-3 rounded-lg bg-amber-500/10 border border-amber-500/20">
                  <AlertCircle className="h-5 w-5 text-amber-400 shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm font-medium text-amber-400">
                      Blockers
                    </p>
                    <p className="text-sm text-amber-200/80">
                      {status.blockers}
                    </p>
                  </div>
                </div>
              )}

              {status.tomorrow && (
                <div className="flex items-start gap-2 p-3 rounded-lg bg-primary/10 border border-primary/20">
                  <ArrowRight className="h-5 w-5 text-primary shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm font-medium text-primary">Next Up</p>
                    <p className="text-sm text-primary/80">{status.tomorrow}</p>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </motion.div>
      </div>
    </section>
  );
}
