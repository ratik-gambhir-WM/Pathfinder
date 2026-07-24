import { ReactNode, useEffect, useRef, useState } from "react";
import { NavLink } from "react-router-dom";
import {
  getDataRoomPath,
  getDealRoomPath,
  getTeamLabel,
  WorkspaceDeal,
  WorkspaceLocationState,
  WorkspaceSidebarTool,
} from "../../data/workspace";
import { useThemeMode } from "../../hooks/useThemeMode";
import { WestMonroeMark } from "../brand/WestMonroeMark";
import { Icon } from "../ui/Icon";

type WorkspaceSidebarProps = {
  activeDealId?: string;
  activeHomeSection?: "hub" | "summarize" | "vault";
  activeSection?: "data-room" | "deal-room" | "timeline";
  deals: WorkspaceDeal[];
  email?: string;
  initiatives?: WorkspaceSidebarTool[];
  mode?: "deal-room" | "home";
  navigationState?: WorkspaceLocationState;
  onDealRoomSectionChange?: (section: "deal-room" | "timeline") => void;
  tools?: WorkspaceSidebarTool[];
};

const dealRoomSidebarLinks = [
  { icon: "dashboard" as const, key: "deal-room" as const, label: "Deal Room" },
  { icon: "timeline" as const, key: "timeline" as const, label: "Meeting Timeline" },
  { icon: "folderOpen" as const, key: "data-room" as const, label: "Data Room Vault" },
  { icon: "grid" as const, label: "Synthesis Canvas" },
];

export function WorkspaceSidebar({
  activeDealId,
  activeHomeSection = "hub",
  activeSection = "deal-room",
  deals,
  email,
  initiatives = [],
  mode = "home",
  navigationState,
  onDealRoomSectionChange,
  tools = [],
}: WorkspaceSidebarProps) {
  const [profileMenuOpen, setProfileMenuOpen] = useState(false);
  const [dealMenuOpen, setDealMenuOpen] = useState(false);
  const dealMenuRef = useRef<HTMLDivElement>(null);
  const teamLabel = getTeamLabel(email);
  const activeDeal = deals.find((deal) => deal.room.id === activeDealId) ?? deals[0];

  useEffect(() => {
    function handlePointerDown(event: PointerEvent) {
      if (!dealMenuRef.current?.contains(event.target as Node)) {
        setDealMenuOpen(false);
      }
    }

    if (dealMenuOpen) {
      document.addEventListener("pointerdown", handlePointerDown);
    }

    return () => document.removeEventListener("pointerdown", handlePointerDown);
  }, [dealMenuOpen]);

  return (
    <aside className="workspace-sidebar hidden h-full w-72 shrink-0 border-r border-white/80 bg-white/40 backdrop-blur-md lg:flex">
      <div className="flex h-full min-h-0 w-full flex-col p-4">
        <div className="space-y-6">
          <NavLink
            aria-label="Back to home page"
            className="flex items-center gap-4 rounded-[22px] px-2 py-2 transition hover:bg-white/40"
            state={navigationState}
            to="/hub"
          >
            <WestMonroeMark framed />
            <div>
              <h1 className="text-[1.1rem] font-bold leading-tight text-text-main [font-family:var(--font-heading)]">
                West Monroe
              </h1>
              <p className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted">Diligence</p>
            </div>
          </NavLink>

          {mode === "home" ? (
            <>
              <nav className="space-y-1">
                <NavLink
                  className={({ isActive }) =>
                    [
                      "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                      isActive && activeHomeSection === "hub"
                        ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
                        : "text-text-main hover:bg-white/40",
                    ].join(" ")
                  }
                  end
                  state={navigationState}
                  to="/hub"
                >
                  <WestMonroeMark className="h-7 w-7" />
                  <span className="text-[12px] font-semibold">Pathfinder</span>
                </NavLink>
              </nav>

              <SidebarSection
                action={
                  <DealSectionMenu
                    containerRef={dealMenuRef}
                    menuOpen={dealMenuOpen}
                    onAddDeal={() => setDealMenuOpen(false)}
                    onToggleMenu={() => setDealMenuOpen((isOpen) => !isOpen)}
                  />
                }
                title="Active Deals"
              >
                {deals.map((deal) => (
                  <NavLink
                    className={({ isActive }) =>
                      [
                        "flex items-center justify-between rounded-2xl px-5 py-3 transition",
                        isActive ? "bg-white/58 shadow-[0_12px_30px_rgba(7,1,84,0.06)]" : "hover:bg-white/40",
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

              <SidebarSection title="Research">
                <SidebarLink icon="search" label="Topics" />
                <SidebarLink
                  homeSection={activeHomeSection}
                  href="/hub/summarize"
                  icon="sparkles"
                  label="Quick Chat"
                  navigationState={navigationState}
                />
              </SidebarSection>

              <div className="border-t border-white/40 pt-6">
                <nav className="space-y-1">
                  {tools.map((item) => (
                    <SidebarLink
                      homeSection={activeHomeSection}
                      href={item.href}
                      icon={item.icon}
                      key={item.name}
                      label={item.name}
                      navigationState={navigationState}
                    />
                  ))}
                </nav>
              </div>
            </>
          ) : (
            <nav className="space-y-2">
              {dealRoomSidebarLinks.map((link) => {
                if (activeDeal && "key" in link && link.key !== "timeline") {
                  const destination = link.key === "deal-room" ? getDealRoomPath(activeDeal.room.id) : getDataRoomPath(activeDeal.room.id);

                  return (
                    <NavLink
                      className={() =>
                        [
                          "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                          activeSection === link.key
                            ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
                            : "text-text-main hover:bg-white/40",
                        ].join(" ")
                      }
                      key={link.label}
                      onClick={() => {
                        if (link.key === "deal-room") {
                          onDealRoomSectionChange?.("deal-room");
                        }
                      }}
                      state={navigationState}
                      to={destination}
                    >
                      <Icon className="h-6 w-6" name={link.icon} />
                      <span className="text-[12px] font-semibold">{link.label}</span>
                    </NavLink>
                  );
                }

                if ("key" in link && link.key === "timeline") {
                  return (
                    <SidebarStaticItem
                      active={activeSection === "timeline"}
                      icon={link.icon}
                      key={link.label}
                      label={link.label}
                      onClick={() => onDealRoomSectionChange?.("timeline")}
                    />
                  );
                }

                return <SidebarStaticItem icon={link.icon} key={link.label} label={link.label} />;
              })}
            </nav>
          )}
        </div>

        <div className="mt-auto pt-6">
          <div className="relative">
            {profileMenuOpen ? <ProfilePreferences /> : null}
            <button
              aria-expanded={profileMenuOpen}
              aria-haspopup="menu"
              className="flex w-full items-center gap-4 rounded-2xl px-4 py-3 text-left transition hover:bg-white/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
              onClick={() => setProfileMenuOpen((isOpen) => !isOpen)}
              type="button"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-secondary-fixed-dim text-white">
                <span className="text-lg font-semibold">{teamLabel.slice(0, 1)}</span>
              </div>
              <div className="min-w-0">
                <p className="truncate text-[17px] font-medium text-text-main">{teamLabel}</p>
                <p className="truncate text-[12px] font-semibold text-secondary">
                  {mode === "deal-room" && activeDeal ? activeDeal.room.name : "Focus: Project Alpha"}
                </p>
              </div>
            </button>
          </div>
        </div>
      </div>
    </aside>
  );
}

function ProfilePreferences() {
  const { setThemeMode, themeMode } = useThemeMode();

  return (
    <div
      className="absolute bottom-full left-0 z-20 mb-3 w-full rounded-2xl border border-outline-variant bg-white p-3 shadow-[0_18px_44px_rgba(7,1,84,0.12)]"
      role="menu"
    >
      <div className="space-y-3">
        <p className="px-2 text-[10px] font-bold uppercase tracking-[0.18em] text-muted">Theme</p>
        <div className="grid grid-cols-2 gap-1 rounded-full border border-outline-variant bg-surface-container-high p-1">
          <ThemeModeButton active={themeMode === "slate-frost"} label="Slate" onClick={() => setThemeMode("slate-frost")} />
          <ThemeModeButton active={themeMode === "dark"} label="Dark" onClick={() => setThemeMode("dark")} />
        </div>
      </div>
    </div>
  );
}

type ThemeModeButtonProps = {
  active: boolean;
  disabled?: boolean;
  label: string;
  onClick: () => void;
};

function ThemeModeButton({ active, disabled = false, label, onClick }: ThemeModeButtonProps) {
  return (
    <button
      aria-pressed={active}
      className={[
        "h-9 rounded-full text-[12px] font-semibold transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed disabled:cursor-not-allowed disabled:opacity-45",
        active ? "bg-primary-container text-on-primary-container shadow-[0_6px_16px_rgba(7,1,84,0.16)]" : "text-primary hover:bg-white",
      ].join(" ")}
      disabled={disabled}
      onClick={onClick}
      type="button"
    >
      {label}
    </button>
  );
}

type SidebarSectionProps = {
  action?: ReactNode;
  children: ReactNode;
  title: string;
};

function SidebarSection({ action, children, title }: SidebarSectionProps) {
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

type DealSectionMenuProps = {
  containerRef: React.RefObject<HTMLDivElement | null>;
  menuOpen: boolean;
  onAddDeal: () => void;
  onToggleMenu: () => void;
};

function DealSectionMenu({ containerRef, menuOpen, onAddDeal, onToggleMenu }: DealSectionMenuProps) {
  return (
    <div className="relative -mr-1" ref={containerRef}>
      <button
        aria-expanded={menuOpen}
        aria-haspopup="menu"
        aria-label="Active deals actions"
        className="flex h-7 w-7 items-center justify-center rounded-full text-muted transition hover:bg-white/60 hover:text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
        onClick={onToggleMenu}
        type="button"
      >
        <Icon className="h-4 w-4" name="plus" />
      </button>

      {menuOpen ? (
        <div
          className="absolute right-0 top-full z-20 mt-2 w-36 rounded-2xl border border-outline-variant bg-white p-1.5 shadow-[0_18px_44px_rgba(7,1,84,0.12)]"
          role="menu"
        >
          <button
            className="flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-[12px] font-semibold text-text-main transition hover:bg-surface-container-high focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
            onClick={onAddDeal}
            role="menuitem"
            type="button"
          >
            <Icon className="h-4 w-4 text-muted" name="plus" />
            <span>Add deal</span>
          </button>
        </div>
      ) : null}
    </div>
  );
}

type SidebarLinkProps = {
  homeSection?: "hub" | "summarize" | "vault";
  href?: string;
  icon: "personSearch" | "terminal" | "timeline" | "folderOpen" | "sparkles" | "search";
  label: string;
  navigationState?: WorkspaceLocationState;
};

function SidebarLink({ homeSection, href, icon, label, navigationState }: SidebarLinkProps) {
  if (href) {
    const isVaultLink = label === "Global Vault";

    return (
      <NavLink
        className={({ isActive }) =>
          [
            "flex w-full items-center gap-4 rounded-2xl px-5 py-3 text-left transition",
            isActive && (!isVaultLink || homeSection === "vault")
              ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
              : "text-text-main hover:bg-white/40",
          ].join(" ")
        }
        end
        state={navigationState}
        to={href}
      >
        <Icon className="h-6 w-6 text-current" name={icon} />
        <span className="text-[12px] font-medium">{label}</span>
      </NavLink>
    );
  }

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
  active?: boolean;
  icon: "dashboard" | "folderOpen" | "grid" | "timeline";
  label: string;
  onClick?: () => void;
};

function SidebarStaticItem({ active = false, icon, label, onClick }: SidebarStaticItemProps) {
  return (
    <button
      className={[
        "flex w-full items-center gap-3 rounded-[22px] px-5 py-4 text-left transition",
        active
          ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
          : "text-text-main hover:bg-white/40",
      ].join(" ")}
      onClick={onClick}
      type="button"
    >
      <Icon className={`h-6 w-6 ${active ? "text-current" : "text-muted"}`} name={icon} />
      <span className={`text-[12px] ${active ? "font-semibold" : "font-medium"}`}>{label}</span>
    </button>
  );
}
