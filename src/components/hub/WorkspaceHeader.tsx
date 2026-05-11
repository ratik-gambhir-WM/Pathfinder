import { Button } from "../ui/Button";
import { Icon } from "../ui/Icon";

export function WorkspaceHeader() {
  return (
    <header className="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
      <div className="space-y-2">
        <h1 className="type-display text-text-main">The Hub</h1>
        <p className="type-subtle text-muted">Portfolio Performance &amp; Strategic Initiatives</p>
      </div>

      <div className="flex flex-wrap items-center gap-3">
        <button className="inline-flex h-14 items-center gap-3 rounded-full border border-white/80 bg-white/70 px-7 text-[14px] font-semibold text-text-main shadow-[0_8px_20px_rgba(28,40,38,0.05)] transition hover:bg-white">
          <Icon className="h-5 w-5 text-muted" name="grid" />
          Grid View
        </button>
        <Button className="h-14 w-auto px-7" icon={<Icon className="h-5 w-5" name="plus" />}>
          New Initiative
        </Button>
      </div>
    </header>
  );
}
