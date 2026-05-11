import { Icon } from "../ui/Icon";
import { WorkspaceCard } from "./WorkspaceCard";

type InsightItem = {
  category: string;
  deal: string;
  fileIcon: "doc" | "pdf" | "image";
  fileName: string;
  image?: boolean;
  quote: string;
  toneClassName: string;
  toneTextClassName: string;
};

type InsightsStripProps = {
  items: readonly InsightItem[];
};

export function InsightsStrip({ items }: InsightsStripProps) {
  return (
    <section className="col-span-12 mt-2">
      <div className="mb-4 flex items-center justify-between px-2">
        <h2 className="type-h3 flex items-center gap-3 text-text-main">
          <Icon className="h-6 w-6 text-accent" name="bookmark" />
          Recent Insights
        </h2>
        <span className="type-label text-muted">Across Projects</span>
      </div>

      <div className="workspace-scrollbar-hidden flex gap-4 overflow-x-auto px-2 pb-6 pt-2">
        {items.map((item) => (
          <InsightChip item={item} key={`${item.deal}-${item.category}`} />
        ))}

        <LoadingChip />

        <div className="flex w-12 shrink-0 items-center justify-center rounded-2xl border-2 border-dashed border-muted/25 text-muted/55 transition hover:bg-white/20 hover:text-muted">
          <Icon className="h-5 w-5" name="arrowRight" />
        </div>
      </div>
    </section>
  );
}

function InsightChip({ item }: { item: InsightItem }) {
  return (
    <article className="chip-card group relative flex w-72 shrink-0 flex-col gap-3 overflow-hidden p-4">
      <div className={`absolute left-0 top-0 h-full w-1 ${item.toneClassName}`} />
      <div className="flex items-start justify-between pl-2">
        <div className="space-y-1">
          <p className={`text-[9px] font-bold uppercase tracking-[0.18em] ${item.toneTextClassName}`}>{item.deal}</p>
          <span className="inline-flex rounded-md bg-surface-container-low px-2 py-1 text-[10px] font-bold uppercase tracking-[0.12em] text-muted">
            {item.category}
          </span>
        </div>
        <Icon className="h-4 w-4 text-muted opacity-0 transition group-hover:opacity-100" name="openInNew" />
      </div>

      {item.image ? <div className="ml-2 h-20 rounded-xl border border-white/60 bg-[linear-gradient(120deg,rgba(92,128,188,0.22),rgba(74,124,88,0.16),rgba(255,255,255,0.4))]" /> : null}

      <p className="line-clamp-3 pl-2 font-heading text-[0.98rem] leading-relaxed text-text-main">{item.quote}</p>

      <div className="mt-auto flex items-center gap-2 pl-2">
        <Icon className="h-4 w-4 text-muted" name={item.fileIcon} />
        <span className="truncate text-xs text-muted">{item.fileName}</span>
      </div>
    </article>
  );
}

function LoadingChip() {
  return (
    <WorkspaceCard className="relative flex w-72 shrink-0 flex-col gap-4 overflow-hidden p-4 opacity-55">
      <div className="absolute left-0 top-0 h-full w-1 bg-muted/25" />
      <div className="space-y-2 pl-2 pt-1">
        <div className="shimmer h-2 w-24 rounded-full bg-muted/10" />
        <div className="shimmer h-2 w-16 rounded-full bg-muted/10" />
      </div>
      <div className="shimmer ml-2 mt-4 h-3 w-full rounded-full bg-muted/10" />
      <div className="shimmer ml-2 h-3 w-5/6 rounded-full bg-muted/10" />
      <div className="mt-auto flex items-center gap-2 pl-2">
        <div className="shimmer h-4 w-4 rounded-full bg-muted/10" />
        <div className="shimmer h-2 w-32 rounded-full bg-muted/10" />
      </div>
    </WorkspaceCard>
  );
}
