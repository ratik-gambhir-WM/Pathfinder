import { Icon } from "../../ui/Icon";

type SidebarStaticItemProps = {
  active?: boolean;
  icon: "dashboard" | "folderOpen" | "graph" | "grid" | "person" | "timeline";
  label: string;
  onClick?: () => void;
};

export function SidebarStaticItem({ active = false, icon, label, onClick }: SidebarStaticItemProps) {
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
