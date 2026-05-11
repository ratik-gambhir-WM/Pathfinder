import { AiSearchCard } from "../components/hub/cards/AiSearchCard";
import { CriticalTasksCard } from "../components/hub/cards/CriticalTasksCard";
import { RecentOpenedCard } from "../components/hub/cards/RecentOpenedCard";
import { InsightsStrip } from "../components/hub/InsightsStrip";
import { WorkspaceHeader } from "../components/hub/WorkspaceHeader";
import { WorkspaceLayout } from "../components/hub/WorkspaceLayout";
import { WorkspaceSidebar } from "../components/hub/WorkspaceSidebar";
import { workspaceDeals, workspaceInitiatives, workspaceTools } from "../data/workspace";
import { useWorkspaceSession } from "../hooks/useWorkspaceSession";

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
  const { email, navigationState } = useWorkspaceSession();

  return (
    <WorkspaceLayout
      sidebar={
        <WorkspaceSidebar
          deals={workspaceDeals}
          email={email}
          initiatives={workspaceInitiatives}
          navigationState={navigationState}
          tools={workspaceTools}
        />
      }
    >
      <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
        <WorkspaceHeader />

        <div className="grid grid-cols-12 gap-6">
          <CriticalTasksCard assignees={["AT", "JD"]} tasks={tasks} />
          <RecentOpenedCard items={recentFiles} />
          <AiSearchCard suggestions={aiSuggestions} />
          <InsightsStrip items={recentInsights} />
        </div>
      </div>
    </WorkspaceLayout>
  );
}
