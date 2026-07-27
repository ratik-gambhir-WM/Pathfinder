import { ReactNode, useState } from "react";
import { NavLink } from "react-router-dom";
import { getTeamLabel, WorkspaceDeal, WorkspaceLocationState } from "../../../data/workspace";
import { WestMonroeMark } from "../../brand/WestMonroeMark";
import { ProfilePreferences } from "./ProfilePreferences";

type SidebarFrameProps = {
  children: ReactNode;
  email?: string;
  navigationState?: WorkspaceLocationState;
  profileDeal?: WorkspaceDeal;
  profileSubtitle?: string;
};

export function SidebarFrame({ children, email, navigationState, profileDeal, profileSubtitle }: SidebarFrameProps) {
  const [profileMenuOpen, setProfileMenuOpen] = useState(false);
  const teamLabel = getTeamLabel(email);
  const subtitle = profileDeal ? profileDeal.room.name : profileSubtitle ?? "Focus: Project Alpha";

  return (
    <aside className="hidden h-full w-72 shrink-0 border-r border-white/80 bg-white/40 backdrop-blur-md lg:flex [html[data-theme=dark]_&]:bg-[#070a1b]">
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

          {children}
        </div>

        <div className="mt-auto pt-6">
          <div className="relative">
            {profileMenuOpen ? <ProfilePreferences email={email} navigationState={navigationState} /> : null}
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
                <p className="truncate text-[12px] font-semibold text-secondary">{subtitle}</p>
              </div>
            </button>
          </div>
        </div>
      </div>
    </aside>
  );
}
