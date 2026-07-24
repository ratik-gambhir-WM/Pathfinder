import { NavLink } from "react-router-dom";
import { getDataRoomPath, getDealRoomPath } from "../../../data/workspace";
import { Icon } from "../../ui/Icon";
import { SidebarFrame } from "./SidebarFrame";
import { SidebarStaticItem } from "./SidebarStaticItem";
import type { DealRoomSidebarProps, DealRoomTabSection } from "./sidebarTypes";

const dealRoomSidebarLinks = [
  { icon: "dashboard" as const, key: "deal-room" as const, label: "Deal Room" },
  { icon: "timeline" as const, key: "timeline" as const, label: "Deal Activity" },
  { icon: "person" as const, key: "site-visits" as const, label: "Site Visits" },
  { icon: "graph" as const, key: "diligence-graph" as const, label: "Diligence Graph" },
  { icon: "folderOpen" as const, key: "data-room" as const, label: "Data Room Vault" },
  { icon: "grid" as const, label: "Synthesis Canvas" },
];

export function DealRoomWorkspaceSidebar({
  activeDealId,
  activeSection,
  deals,
  email,
  navigationState,
  onDealRoomSectionChange,
}: DealRoomSidebarProps) {
  const activeDeal = deals.find((deal) => deal.room.id === activeDealId) ?? deals[0];

  return (
    <SidebarFrame email={email} navigationState={navigationState} profileDeal={activeDeal}>
      <nav className="space-y-2">
        {dealRoomSidebarLinks.map((link) => {
          if (activeDeal && "key" in link && (link.key === "deal-room" || link.key === "data-room")) {
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

          if ("key" in link && (link.key === "timeline" || link.key === "diligence-graph" || link.key === "site-visits")) {
            return (
              <SidebarStaticItem
                active={activeSection === link.key}
                icon={link.icon}
                key={link.label}
                label={link.label}
                onClick={() => onDealRoomSectionChange?.(link.key as DealRoomTabSection)}
              />
            );
          }

          return <SidebarStaticItem icon={link.icon} key={link.label} label={link.label} />;
        })}
      </nav>
    </SidebarFrame>
  );
}
