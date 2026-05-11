import { useMemo, useState } from "react";
import { DataRoomChip } from "../../data/dataRoom";
import { Icon } from "../ui/Icon";

type ChipBankPanelProps = {
  chips: DataRoomChip[];
};

export function ChipBankPanel({ chips }: ChipBankPanelProps) {
  const [query, setQuery] = useState("");

  const filteredChips = useMemo(() => {
    const normalized = query.trim().toLowerCase();

    if (!normalized) {
      return chips;
    }

    return chips.filter((chip) => {
      const searchable =
        chip.type === "text"
          ? `${chip.category} ${chip.body}`
          : chip.type === "chart"
            ? `${chip.category} ${chip.footer}`
            : `${chip.category} ${chip.rows.map((row) => `${row.label} ${row.value}`).join(" ")}`;

      return searchable.toLowerCase().includes(normalized);
    });
  }, [chips, query]);

  return (
    <section className="glass-panel flex min-w-[320px] max-w-[380px] flex-[0_0_30%] flex-col rounded-[28px] p-6">
      <div className="mb-6 flex items-center justify-between">
        <h3 className="flex items-center gap-3 font-bold text-text-main [font-family:var(--font-heading)] text-[2rem] tracking-[-0.03em]">
          <Icon className="h-7 w-7 text-accent" name="bookmark" />
          Chip Bank
        </h3>
        <button className="rounded-full p-2 text-muted transition hover:bg-white/60 hover:text-text-main" type="button">
          <Icon className="h-5 w-5" name="filter" />
        </button>
      </div>

      <label className="relative mb-6">
        <Icon className="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted" name="search" />
        <input
          className="w-full rounded-2xl border border-white/80 bg-white/60 py-3 pl-11 pr-4 text-sm outline-none ring-0 transition placeholder:text-muted/70 focus:border-primary/25"
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Filter insights..."
          type="text"
          value={query}
        />
      </label>

      <div className="workspace-scrollbar-hidden flex-1 space-y-4 overflow-y-auto pr-2">
        {filteredChips.map((chip) => (
          <InsightChipCard chip={chip} key={chip.id} />
        ))}
      </div>
    </section>
  );
}

type InsightChipCardProps = {
  chip: DataRoomChip;
};

function InsightChipCard({ chip }: InsightChipCardProps) {
  const toneClasses = {
    accent: "bg-accent/10 text-accent border-accent/20",
    muted: "bg-muted/10 text-muted border-muted/20",
    primary: "bg-primary/10 text-primary border-primary/20",
  }[chip.tone];

  return (
    <article className="chip-card relative overflow-hidden rounded-[22px] p-4">
      <div className={`absolute left-0 top-0 h-full w-1 ${chip.tone === "accent" ? "bg-accent" : chip.tone === "primary" ? "bg-primary" : "bg-muted"}`} />

      {chip.type === "text" ? (
        <>
          <div className="mb-3">
            <span className={`rounded-md border px-2 py-1 text-[10px] font-bold uppercase tracking-[0.16em] ${toneClasses}`}>
              {chip.category}
            </span>
          </div>
          <p className="text-[1.05rem] leading-8 text-text-main [font-family:var(--font-heading)]">{chip.body}</p>
        </>
      ) : null}

      {chip.type === "chart" ? (
        <>
          <div className="mb-4">
            <span className={`rounded-md border px-2 py-1 text-[10px] font-bold uppercase tracking-[0.16em] ${toneClasses}`}>
              {chip.category}
            </span>
          </div>
          <div className="mt-2 flex h-28 items-end gap-2 rounded-2xl border border-white/40 bg-background/40 px-4 pb-4">
            {chip.bars.map((bar, index) => (
              <div
                className={`w-full rounded-t-md ${chip.tone === "primary" ? "bg-primary/70" : "bg-primary/35"} ${index === 2 ? "bg-primary" : ""}`}
                key={`${chip.id}-${bar}`}
                style={{ height: `${bar}%` }}
              />
            ))}
          </div>
          <p className="mt-3 text-center text-[10px] font-bold uppercase tracking-[0.16em] text-muted">{chip.footer}</p>
        </>
      ) : null}

      {chip.type === "metrics" ? (
        <>
          <div className="mb-4">
            <span className={`rounded-md border px-2 py-1 text-[10px] font-bold uppercase tracking-[0.16em] ${toneClasses}`}>
              {chip.category}
            </span>
          </div>
          <div className="space-y-3">
            {chip.rows.map((row) => (
              <div className="flex items-center justify-between text-sm" key={row.label}>
                <span className="font-medium text-muted">{row.label}</span>
                <span className="font-bold text-text-main">{row.value}</span>
              </div>
            ))}
          </div>
        </>
      ) : null}
    </article>
  );
}
