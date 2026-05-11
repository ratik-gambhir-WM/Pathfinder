import { WorkspaceHeader } from "../components/hub/WorkspaceHeader";
import { WorkspaceSidebar } from "../components/hub/WorkspaceSidebar";
import { AiSearchCard } from "../components/hub/cards/AiSearchCard";
import { CriticalTasksCard } from "../components/hub/cards/CriticalTasksCard";
import { RecentOpenedCard } from "../components/hub/cards/RecentOpenedCard";
import { InsightsStrip } from "../components/hub/InsightsStrip";
import { useLocation } from "react-router-dom";

type HubLocationState = {
  email?: string;
};

const deals = [
  { colorClassName: "bg-primary", complete: true, name: "Project Alpha" },
  { colorClassName: "bg-[#6b87c8]", name: "Project Beta" },
  { colorClassName: "bg-muted", name: "Logistics Merger" },
];

const initiatives = [
  { icon: "personSearch" as const, name: "Q4 Recruiting" },
  { icon: "terminal" as const, name: "Software Migration" },
];

const tools = [
  { icon: "timeline" as const, name: "Meeting Timeline" },
  { icon: "folderOpen" as const, name: "Global Vault" },
];

const tasks = [
  {
    checked: true,
    label: "Finalize Q3 Financial Extract for Project Alpha",
    tag: { tone: "error" as const, value: "High Priority" },
  },
  {
    checked: false,
    label: "Review legal disclosures for Logistics Merger",
    tag: { tone: "success" as const, value: "Due Today" },
  },
  {
    checked: false,
    label: "Approve analyst transcript summary: CEO Site Visit",
    tag: { tone: "muted" as const, value: "Alpha" },
  },
  {
    checked: false,
    label: "Initialize Data Room for Project Gamma",
    tag: { tone: "icon" as const, value: "more" },
  },
] as const;

const recentFiles = [
  {
    deal: "Project Alpha",
    icon: "pdf" as const,
    time: "2m ago",
    title: "Q3 Financial Report.pdf",
    tone: "error" as const,
  },
  {
    deal: "Project Beta",
    icon: "doc" as const,
    time: "1h ago",
    title: "Meeting Minutes - Legal Review.doc",
    tone: "accent" as const,
  },
  {
    deal: "Logistics Merger",
    icon: "sheet" as const,
    time: "3h ago",
    title: "Logistics_Due_Diligence.xlsx",
    tone: "primary" as const,
  },
  {
    deal: "Project Alpha",
    icon: "pdf" as const,
    time: "Yesterday",
    title: "Environmental Impact Study.pdf",
    tone: "error" as const,
  },
];

const recentInsights = [
  {
    category: "Financials",
    deal: "Project Alpha",
    fileIcon: "doc",
    fileName: "Transcript: CEO Site Visit",
    quote: "CapEx will decrease by 15% next year due to completion of the automated packing line.",
    toneClassName: "bg-primary",
    toneTextClassName: "text-primary",
  },
  {
    category: "Legal Risk",
    deal: "Project Beta",
    fileIcon: "pdf",
    fileName: "Contract_Dist_v2.pdf",
    quote: "Clause 4.2 indicates severe penalties if delivery quotas to main distributor fall below 95%.",
    toneClassName: "bg-accent",
    toneTextClassName: "text-accent",
  },
  {
    category: "Market Data",
    deal: "Project Alpha",
    fileIcon: "image" as const,
    fileName: "Q3_Deck_Slide14.png",
    image: true,
    quote: "Distribution expansion is strongest across the Southeast region, with demand pockets clustering around new warehouse nodes.",
    toneClassName: "bg-primary",
    toneTextClassName: "text-primary",
  },
] as const;

const aiSuggestions = ['"Compare Q3 EBITDA across Alpha and Beta"', '"Summarize recent legal risks"'];

export function HubPage() {
  const location = useLocation();
  const state = (location.state ?? {}) as HubLocationState;

  return (
    <div className="relative h-screen overflow-hidden bg-background text-on-surface">
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-[14%] top-[18%] h-[22rem] w-[22rem] rounded-full bg-tertiary-fixed/20 blur-3xl" />
        <div className="absolute right-[10%] top-[10%] h-[24rem] w-[24rem] rounded-full bg-primary-fixed/20 blur-3xl" />
        <div className="absolute bottom-[8%] left-[34%] h-[28rem] w-[28rem] rounded-full bg-surface-container-high/80 blur-3xl" />
      </div>

      <div className="relative z-10 flex h-full min-h-0">
        <WorkspaceSidebar
          deals={deals}
          email={state.email}
          initiatives={initiatives}
          tools={tools}
        />

        <main className="workspace-scrollbar-hidden flex-1 overflow-y-auto px-8 py-8">
          <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
            <WorkspaceHeader />

            <div className="grid grid-cols-12 gap-6">
              <CriticalTasksCard assignees={["AT", "JD"]} tasks={tasks} />
              <RecentOpenedCard items={recentFiles} />
              <AiSearchCard suggestions={aiSuggestions} />
              <InsightsStrip items={recentInsights} />
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
