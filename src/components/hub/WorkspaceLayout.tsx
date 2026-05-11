import { ReactNode } from "react";

type WorkspaceLayoutProps = {
  children: ReactNode;
  sidebar: ReactNode;
};

export function WorkspaceLayout({ children, sidebar }: WorkspaceLayoutProps) {
  return (
    <div className="relative h-screen overflow-hidden bg-background text-on-surface">
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-[14%] top-[18%] h-[22rem] w-[22rem] rounded-full bg-tertiary-fixed/20 blur-3xl" />
        <div className="absolute right-[10%] top-[10%] h-[24rem] w-[24rem] rounded-full bg-primary-fixed/20 blur-3xl" />
        <div className="absolute bottom-[8%] left-[34%] h-[28rem] w-[28rem] rounded-full bg-surface-container-high/80 blur-3xl" />
      </div>

      <div className="relative z-10 flex h-full min-h-0">
        {sidebar}

        <main className="workspace-scrollbar-hidden flex-1 overflow-y-auto px-8 py-8">{children}</main>
      </div>
    </div>
  );
}
