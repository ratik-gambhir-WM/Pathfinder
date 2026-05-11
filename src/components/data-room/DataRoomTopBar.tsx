import { ReactNode } from "react";
import { Button } from "../ui/Button";
import { Icon } from "../ui/Icon";

type DataRoomTopBarProps = {
  teamInitial: string;
};

export function DataRoomTopBar({ teamInitial }: DataRoomTopBarProps) {
  return (
    <header className="sticky top-0 z-20 flex h-[72px] items-center gap-6 border-b border-white/80 bg-white/40 px-8 backdrop-blur-md">
      <div className="shrink-0">
        <h1 className="text-[2rem] font-bold tracking-[-0.03em] text-text-main [font-family:var(--font-heading)]">
          Pathfinder
        </h1>
      </div>

      <div className="flex flex-1 items-center">
        <label className="flex w-full items-center gap-3 rounded-full border border-white/80 bg-white/60 px-5 py-3 shadow-sm">
          <Icon className="h-5 w-5 text-muted" name="search" />
          <input
            className="w-full border-none bg-transparent text-[14px] text-text-main outline-none placeholder:text-muted/70"
            placeholder="Search insights..."
            type="text"
          />
        </label>
      </div>

      <div className="flex shrink-0 items-center gap-3">
        <TopActionButton icon={<Icon className="h-5 w-5" name="notification" />} />
        <TopActionButton icon={<Icon className="h-5 w-5" name="settings" />} />
        <Button className="h-12 w-auto px-7">Export Report</Button>
        <div className="flex h-10 w-10 items-center justify-center rounded-full border-2 border-white bg-secondary-fixed-dim text-sm font-semibold text-white shadow-sm">
          {teamInitial}
        </div>
      </div>
    </header>
  );
}

type TopActionButtonProps = {
  icon: ReactNode;
};

function TopActionButton({ icon }: TopActionButtonProps) {
  return (
    <button
      className="inline-flex h-10 w-10 items-center justify-center rounded-full text-muted transition hover:bg-white/80 hover:text-text-main"
      type="button"
    >
      {icon}
    </button>
  );
}
