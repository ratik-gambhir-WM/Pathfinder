import { ReactNode } from "react";
import { Icon } from "../ui/Icon";

type DealItem = {
  colorClassName: string;
  complete?: boolean;
  name: string;
};

type SidebarTool = {
  icon: "personSearch" | "terminal" | "timeline" | "folderOpen";
  name: string;
};

type WorkspaceSidebarProps = {
  deals: DealItem[];
  email?: string;
  initiatives: SidebarTool[];
  tools: SidebarTool[];
};

export function WorkspaceSidebar({ deals, email, initiatives, tools }: WorkspaceSidebarProps) {
  const teamName = email?.split("@")[0]?.replace(/[._-]/g, " ") ?? "Analyst Team";
  const teamLabel = teamName.replace(/\b\w/g, (character) => character.toUpperCase());

  return (
    <aside className="hidden h-full w-72 shrink-0 border-r border-white/80 bg-white/40 backdrop-blur-md lg:flex">
      <div className="flex h-full min-h-0 w-full flex-col p-4">
        <div className="space-y-6">
          <div className="flex items-center gap-4 px-2">
            <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/8 text-primary">
              <Icon className="h-8 w-8" name="dataset" />
            </div>
            <div>
              <h1 className="text-[1.1rem] font-bold leading-tight text-text-main [font-family:var(--font-heading)]">
                Synthesis Canvas
              </h1>
              <p className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted">Enterprise</p>
            </div>
          </div>

          <nav className="space-y-1">
            <a
              className="flex items-center gap-3 rounded-[22px] border border-primary/20 bg-primary/8 px-5 py-4 text-primary shadow-[inset_0_0_0_1px_rgba(74,124,88,0.06)]"
              href="#"
            >
              <Icon className="h-7 w-7" name="dashboard" />
              <span className="text-[17px] font-semibold">The Hub</span>
            </a>
          </nav>

          <SidebarSection title="Active Deals">
            {deals.map((deal) => (
              <div className="flex items-center justify-between px-5 py-2" key={deal.name}>
                <div className="flex items-center gap-4">
                  <span className={`h-3.5 w-3.5 rounded-full ${deal.colorClassName}`} />
                  <span className="text-[17px] font-medium text-text-main">{deal.name}</span>
                </div>
                {deal.complete ? <Icon className="h-5 w-5 text-primary/85" name="checkCircle" /> : null}
              </div>
            ))}
          </SidebarSection>

          <SidebarSection title="Internal Initiatives">
            {initiatives.map((item) => (
              <SidebarLink icon={item.icon} key={item.name} label={item.name} />
            ))}
          </SidebarSection>

          <div className="border-t border-white/40 pt-6">
            <nav className="space-y-1">
              {tools.map((item) => (
                <SidebarLink icon={item.icon} key={item.name} label={item.name} />
              ))}
            </nav>
          </div>
        </div>

        <div className="mt-auto pt-6">
          <div className="flex items-center gap-4 rounded-2xl px-4 py-3 transition hover:bg-white/40">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-secondary-fixed-dim text-white">
              <span className="text-lg font-semibold">{teamLabel.slice(0, 1)}</span>
            </div>
            <div className="min-w-0">
              <p className="truncate text-[17px] font-medium text-text-main">{teamLabel}</p>
              <p className="truncate text-[12px] font-semibold text-secondary">Focus: Project Alpha</p>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}

type SidebarSectionProps = {
  children: ReactNode;
  title: string;
};

function SidebarSection({ children, title }: SidebarSectionProps) {
  return (
    <section className="space-y-3">
      <h2 className="px-5 text-[11px] font-bold uppercase tracking-[0.2em] text-muted">{title}</h2>
      <nav className="space-y-1">{children}</nav>
    </section>
  );
}

type SidebarLinkProps = {
  icon: "personSearch" | "terminal" | "timeline" | "folderOpen";
  label: string;
};

function SidebarLink({ icon, label }: SidebarLinkProps) {
  return (
    <a className="flex items-center gap-4 rounded-2xl px-5 py-3 text-text-main transition hover:bg-white/40" href="#">
      <Icon className="h-6 w-6 text-muted" name={icon} />
      <span className="text-[17px] font-medium">{label}</span>
    </a>
  );
}
