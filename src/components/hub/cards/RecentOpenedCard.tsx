import { Icon } from "../../ui/Icon";
import { WorkspaceCard } from "../WorkspaceCard";

type RecentFile = {
  deal: string;
  icon: "doc" | "pdf" | "sheet";
  time: string;
  title: string;
  tone: "accent" | "error" | "primary";
};

type RecentOpenedCardProps = {
  items: RecentFile[];
};

export function RecentOpenedCard({ items }: RecentOpenedCardProps) {
  return (
    <WorkspaceCard className="col-span-12 flex min-h-[640px] flex-col p-6 lg:col-span-4 lg:row-span-2">
      <div className="mb-6 flex items-center justify-between">
        <h2 className="type-h3 text-text-main">Recently Opened</h2>
        <button className="text-muted transition hover:text-text-main" type="button">
          <Icon className="h-5 w-5" name="filter" />
        </button>
      </div>

      <div className="workspace-scrollbar-hidden flex-1 overflow-y-auto pr-2">
        <div className="space-y-4">
          {items.map((item) => (
            <RecentFileRow item={item} key={item.title} />
          ))}
        </div>
      </div>

      <button className="mt-5 rounded-xl border border-white/80 bg-white/55 py-3 text-sm font-semibold text-text-main shadow-sm transition hover:bg-white">
        View All Updates
      </button>
    </WorkspaceCard>
  );
}

function RecentFileRow({ item }: { item: RecentFile }) {
  const toneClassName =
    item.tone === "error"
      ? "bg-error/10 text-error"
      : item.tone === "accent"
        ? "bg-accent/10 text-accent"
        : "bg-primary/10 text-primary";

  return (
    <div className="group flex cursor-pointer items-center gap-4 rounded-xl p-2 transition hover:bg-white/40">
      <div className={`flex h-16 w-16 shrink-0 items-center justify-center rounded-2xl ${toneClassName}`}>
        <Icon className="h-8 w-8" name={item.icon} />
      </div>
      <div className="min-w-0 flex-1">
        <p className="truncate text-[17px] font-semibold text-text-main">{item.title}</p>
        <p className="truncate text-[11px] font-medium uppercase tracking-[0.12em] text-muted">
          {item.deal} • {item.time}
        </p>
      </div>
    </div>
  );
}
