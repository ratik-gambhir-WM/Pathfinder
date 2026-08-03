import { NavLink } from "react-router-dom";
import { WorkspaceLocationState } from "../../../data/workspace";
import { Icon } from "../../ui/Icon";

type SidebarLinkProps = {
  homeSection?: "account" | "hub" | "summarize" | "tauri-playground" | "vault";
  href?: string;
  icon: "personSearch" | "terminal" | "timeline" | "folderOpen" | "sparkles" | "search";
  label: string;
  navigationState?: WorkspaceLocationState;
};

export function SidebarLink({ homeSection, href, icon, label, navigationState }: SidebarLinkProps) {
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
        <span className="text-[13px] font-medium">{label}</span>
      </NavLink>
    );
  }

  return (
    <button
      className="flex w-full items-center gap-4 rounded-2xl px-5 py-3 text-left text-text-main transition hover:bg-white/40"
      type="button"
    >
      <Icon className="h-6 w-6 text-muted" name={icon} />
      <span className="text-[13px] font-medium">{label}</span>
    </button>
  );
}
