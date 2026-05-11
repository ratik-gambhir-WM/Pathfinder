import { ReactNode } from "react";
import { NavLink } from "react-router-dom";
import {
  getDealRoomPath,
  getTeamLabel,
  WorkspaceDeal,
  WorkspaceLocationState,
  WorkspaceSidebarTool,
} from "../../data/workspace";
import { Icon } from "../ui/Icon";

type WorkspaceSidebarProps = {
  activeDealId?: string;
  deals: WorkspaceDeal[];
  email?: string;
  initiatives?: WorkspaceSidebarTool[];
  mode?: "deal-room" | "home";
  navigationState?: WorkspaceLocationState;
  tools?: WorkspaceSidebarTool[];
};

const dealRoomSidebarLinks = [
  { icon: "dashboard" as const, label: "Deal Room" },
  { icon: "timeline" as const, label: "Meeting Timeline" },
  { icon: "folderOpen" as const, label: "Data Room Vault" },
  { icon: "grid" as const, label: "Synthesis Canvas" },
];

export function WorkspaceSidebar({
  activeDealId,
  deals,
  email,
  initiatives = [],
  mode = "home",
  navigationState,
  tools = [],
}: WorkspaceSidebarProps) {
  const teamLabel = getTeamLabel(email);
  const activeDeal = deals.find((deal) => deal.room.id === activeDealId) ?? deals[0];

  return (
    <aside className="hidden h-full w-72 shrink-0 border-r border-white/80 bg-white/40 backdrop-blur-md lg:flex">
      <div className="flex h-full min-h-0 w-full flex-col p-4">
        <div className="space-y-6">
          <div className="flex items-center gap-4 px-2">
            <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-primary/8 text-primary">
              <Icon className="h-7 w-7" name="dashboard" />
            </div>
            <div>
              <h1 className="text-[1.1rem] font-bold leading-tight text-text-main [font-family:var(--font-heading)]">
                Pathfinder
              </h1>
              <p className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted">Workspace</p>
            </div>
          </div>

          {mode === "home" ? (
            <>
              <nav className="space-y-1">
                <NavLink
                  className={({ isActive }) =>
                    [
                      "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                      isActive
                        ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(74,124,88,0.06)]"
                        : "text-text-main hover:bg-white/40",
                    ].join(" ")
                  }
                  end
                  state={navigationState}
                  to="/hub"
                >
                  <Icon className="h-7 w-7" name="dashboard" />
                  <span className="text-[12px] font-semibold">The Hub</span>
                </NavLink>
              </nav>

              <SidebarSection title="Active Deals">
                {deals.map((deal) => (
                  <NavLink
                    className={({ isActive }) =>
                      [
                        "flex items-center justify-between rounded-2xl px-5 py-3 transition",
                        isActive ? "bg-white/58 shadow-[0_12px_30px_rgba(28,40,38,0.06)]" : "hover:bg-white/40",
                      ].join(" ")
                    }
                    key={deal.room.id}
                    state={navigationState}
                    to={getDealRoomPath(deal.room.id)}
                  >
                    <div className="flex items-center gap-4">
                      <span className={`h-3.5 w-3.5 rounded-full ${deal.colorClassName}`} />
                      <span className="text-[12px] font-medium text-text-main">{deal.room.name}</span>
                    </div>
                    {deal.complete ? <Icon className="h-5 w-5 text-primary/85" name="checkCircle" /> : null}
                  </NavLink>
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
            </>
          ) : (
            <nav className="space-y-2">
              {dealRoomSidebarLinks.map((link, index) => {
                const isPrimaryLink = index === 0;

                if (isPrimaryLink && activeDeal) {
                  return (
                    <NavLink
                      className={({ isActive }) =>
                        [
                          "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                          isActive
                            ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(74,124,88,0.06)]"
                            : "text-text-main hover:bg-white/40",
                        ].join(" ")
                      }
                      end
                      key={link.label}
                      state={navigationState}
                      to={getDealRoomPath(activeDeal.room.id)}
                    >
                      <Icon className="h-6 w-6" name={link.icon} />
                      <span className="text-[12px] font-semibold">{link.label}</span>
                    </NavLink>
                  );
                }

                return <SidebarStaticItem icon={link.icon} key={link.label} label={link.label} />;
              })}
            </nav>
          )}
        </div>

        <div className="mt-auto pt-6">
          <div className="flex items-center gap-4 rounded-2xl px-4 py-3 transition hover:bg-white/40">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-secondary-fixed-dim text-white">
              <span className="text-lg font-semibold">{teamLabel.slice(0, 1)}</span>
            </div>
            <div className="min-w-0">
              <p className="truncate text-[17px] font-medium text-text-main">{teamLabel}</p>
              <p className="truncate text-[12px] font-semibold text-secondary">
                {mode === "deal-room" && activeDeal ? activeDeal.room.name : "Focus: Project Alpha"}
              </p>
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
    <button
      className="flex w-full items-center gap-4 rounded-2xl px-5 py-3 text-left text-text-main transition hover:bg-white/40"
      type="button"
    >
      <Icon className="h-6 w-6 text-muted" name={icon} />
      <span className="text-[12px] font-medium">{label}</span>
    </button>
  );
}

type SidebarStaticItemProps = {
  icon: "dashboard" | "folderOpen" | "grid" | "timeline";
  label: string;
};

function SidebarStaticItem({ icon, label }: SidebarStaticItemProps) {
  return (
    <button
      className="flex w-full items-center gap-3 rounded-[22px] px-5 py-4 text-left text-text-main transition hover:bg-white/40"
      type="button"
    >
      <Icon className="h-6 w-6 text-muted" name={icon} />
      <span className="text-[12px] font-medium">{label}</span>
    </button>
  );
}
