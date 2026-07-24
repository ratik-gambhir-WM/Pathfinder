import { ReactNode } from "react";

type SidebarSectionProps = {
  action?: ReactNode;
  children: ReactNode;
  title: string;
};

export function SidebarSection({ action, children, title }: SidebarSectionProps) {
  return (
    <section className="space-y-3">
      <div className="flex items-center justify-between px-5">
        <h2 className="text-[11px] font-bold uppercase tracking-[0.2em] text-muted">{title}</h2>
        {action}
      </div>
      <nav className="space-y-1">{children}</nav>
    </section>
  );
}
