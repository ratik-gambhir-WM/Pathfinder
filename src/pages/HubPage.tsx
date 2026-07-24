import { AiSearchCard } from "../components/hub/cards/AiSearchCard";
import { CriticalTasksCard } from "../components/hub/cards/CriticalTasksCard";
import { RecentOpenedCard } from "../components/hub/cards/RecentOpenedCard";
import { InsightsStrip } from "../components/hub/InsightsStrip";
import { WorkspaceHeader } from "../components/hub/WorkspaceHeader";
import { WorkspaceHomeShell } from "../components/hub/WorkspaceHomeShell";
import { workspaceInsights } from "../data/workspace";

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

const aiSuggestions = ['"Compare Q3 EBITDA across Alpha and Beta"', '"Summarize recent legal risks"'];

export function HubPage() {
  return (
    <WorkspaceHomeShell>
      <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
        <WorkspaceHeader />

        <div className="grid grid-cols-12 gap-6">
          <AiSearchCard suggestions={aiSuggestions} />
          <RecentOpenedCard items={recentFiles} />
          <CriticalTasksCard assignees={["AT", "JD"]} tasks={tasks} />
          <InsightsStrip items={workspaceInsights} />
        </div>
      </div>
    </WorkspaceHomeShell>
  );
}
