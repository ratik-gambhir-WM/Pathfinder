import { DealRoomData } from "../../data/workspace";
import { WorkspaceCard } from "../hub/WorkspaceCard";
import { Icon } from "../ui/Icon";

type DealSummaryCardProps = {
  deal: DealRoomData;
};

export function DealSummaryCard({ deal }: DealSummaryCardProps) {
  return (
    <WorkspaceCard className="relative col-span-12 overflow-hidden rounded-[28px] bg-[rgba(248,250,245,0.9)] p-8 lg:p-10 xl:col-span-8">
      <div className="pointer-events-none absolute -right-16 -top-16 h-52 w-52 rounded-full bg-primary/6 blur-3xl" />

      <div className="relative z-10 flex h-full flex-col gap-10">
        <div className="flex flex-col gap-8 lg:flex-row lg:justify-between">
          <div className="max-w-3xl space-y-5">
            <h2 className="text-[1.75rem] font-bold leading-[1.02] tracking-[-0.04em] text-text-main [font-family:var(--font-heading)] sm:text-[2.2rem]">
              {deal.name}
              <br />
              Due Diligence
            </h2>

            <div className="flex flex-wrap gap-2">
              <StatusBadge className="bg-primary/10 text-primary" label={deal.stageLabel} />
              <StatusBadge className="bg-tertiary-fixed/35 text-tertiary" label={deal.phaseLabel} />
              <StatusBadge className="border border-white/80 bg-white/80 text-muted" label={deal.sectorLabel} />
            </div>

            <p className="max-w-2xl text-[1.05rem] leading-8 text-text-main/90">{deal.summary}</p>
          </div>

          <div className="flex h-16 w-16 shrink-0 items-center justify-center rounded-full border border-white/90 bg-white/80 text-primary shadow-[0_10px_24px_rgba(28,40,38,0.06)]">
            <Icon className="h-8 w-8" name="dataset" />
          </div>
        </div>

        <div className="grid gap-5 border-t border-white/60 pt-8 sm:grid-cols-3">
          {deal.metrics.map((metric) => (
            <div className="space-y-2" key={metric.label}>
              <p
                className={`text-[2.1rem] font-bold leading-none tracking-[-0.04em] [font-family:var(--font-heading)] ${
                  metric.tone === "error" ? "text-error" : "text-text-main"
                }`}
              >
                {metric.value}
              </p>
              <p className="text-[9px] font-semibold uppercase tracking-[0.18em] text-muted">{metric.label}</p>
            </div>
          ))}
        </div>

        <div className="grid gap-8 border-t border-white/50 pt-8 xl:grid-cols-2">
          <section className="space-y-4">
            <div className="flex items-center gap-3">
              <Icon className="h-6 w-6 text-primary" name="help" />
              <h3 className="type-h2 text-text-main">Key Questions</h3>
            </div>

            <div className="space-y-3">
              {deal.keyQuestions.map((question) => (
                <div
                  className="flex items-start gap-3 rounded-2xl border border-white/70 bg-white/42 px-4 py-4"
                  key={question}
                >
                  <span className="mt-1 h-2.5 w-2.5 rounded-full bg-primary" />
                  <p className="text-[0.96rem] leading-6 text-text-main">{question}</p>
                </div>
              ))}
            </div>
          </section>

          <section className="space-y-4">
            <div className="flex items-center gap-3">
              <Icon className="h-6 w-6 text-accent" name="sparkles" />
              <h3 className="type-h2 text-text-main">Investment Thesis</h3>
            </div>

            <div className="rounded-[24px] border border-white/70 bg-white/45 p-5">
              <p className="text-[0.96rem] leading-7 text-text-main/86 italic">"{deal.thesis}"</p>
            </div>
          </section>
        </div>
      </div>
    </WorkspaceCard>
  );
}

type StatusBadgeProps = {
  className: string;
  label: string;
};

function StatusBadge({ className, label }: StatusBadgeProps) {
  return (
    <span className={`inline-flex rounded-full px-4 py-2 text-[12px] font-semibold uppercase tracking-[0.16em] ${className}`}>
      {label}
    </span>
  );
}
