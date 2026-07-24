import { useEffect, useRef, useState } from "react";
import { NavLink } from "react-router-dom";
import { getDealRoomPath } from "../../../data/workspace";
import { Icon } from "../../ui/Icon";
import { AddDealModal } from "./AddDealModal";
import { DealSectionMenu } from "./DealSectionMenu";
import { SidebarFrame } from "./SidebarFrame";
import { SidebarLink } from "./SidebarLink";
import { SidebarSection } from "./SidebarSection";
import type { HomeSidebarProps } from "./sidebarTypes";

export function HomeWorkspaceSidebar({
  activeHomeSection,
  deals,
  email,
  initiatives,
  navigationState,
  tools,
}: HomeSidebarProps) {
  const [dealMenuOpen, setDealMenuOpen] = useState(false);
  const [addDealModalOpen, setAddDealModalOpen] = useState(false);
  const dealMenuRef = useRef<HTMLDivElement>(null);

  function handleAddDeal() {
    setDealMenuOpen(false);
    setAddDealModalOpen(true);
  }

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
    <>
      <SidebarFrame email={email} navigationState={navigationState}>
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
            <Icon className="h-7 w-7" name="home" />
            <span className="text-[12px] font-semibold">Deal Hub</span>
          </NavLink>
        </nav>

        <SidebarSection
          action={
            <DealSectionMenu
              containerRef={dealMenuRef}
              menuOpen={dealMenuOpen}
              onAddDeal={handleAddDeal}
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
      </SidebarFrame>

      {addDealModalOpen ? <AddDealModal onClose={() => setAddDealModalOpen(false)} /> : null}
    </>
  );
}
