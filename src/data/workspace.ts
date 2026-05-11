export type WorkspaceLocationState = {
  email?: string;
};

export type WorkspaceSidebarIcon = "personSearch" | "terminal" | "timeline" | "folderOpen";

export type WorkspaceSidebarTool = {
  icon: WorkspaceSidebarIcon;
  name: string;
};

export type DealTimelineTone = "accent" | "error" | "muted" | "primary";

export type DealTimelineItem = {
  detail: string;
  id: string;
  timestamp: string;
  title: string;
  tone: DealTimelineTone;
};

export type DealMetric = {
  label: string;
  tone?: "default" | "error";
  value: string;
};

export type DealTask = {
  done?: boolean;
  id: string;
  label: string;
  priority?: boolean;
};

export type DealRoomData = {
  id: string;
  keyQuestions: string[];
  metrics: DealMetric[];
  name: string;
  overviewSubtitle: string;
  phaseLabel: string;
  pendingTasks: DealTask[];
  sectorLabel: string;
  stageLabel: string;
  summary: string;
  thesis: string;
  timeline: DealTimelineItem[];
};

export type WorkspaceDeal = {
  colorClassName: string;
  complete?: boolean;
  room: DealRoomData;
};

export const workspaceDeals: WorkspaceDeal[] = [
  {
    colorClassName: "bg-primary",
    complete: true,
    room: {
      id: "project-alpha",
      keyQuestions: [
        "What are the primary drivers of the 15% CapEx reduction?",
        "Are there undisclosed environmental liabilities at the secondary site?",
        "How do retention rates compare with industry benchmarks for Tier 1 staff?",
      ],
      metrics: [
        { label: "Files Analyzed", value: "42" },
        { label: "Insights Extracted", value: "18" },
        { label: "Critical Risks", tone: "error", value: "3" },
      ],
      name: "Project Alpha",
      overviewSubtitle: "Project Alpha Due Diligence Overview",
      pendingTasks: [
        { id: "alpha-turnover", label: "Review Q3 Employee Turnover Data" },
        { id: "alpha-cfo", label: "Schedule Follow-up with CFO" },
        { id: "alpha-litigation", label: "Draft initial memo on EPA litigation", priority: true },
      ],
      phaseLabel: "Phase 1",
      sectorLabel: "Manufacturing Sector",
      stageLabel: "In Progress",
      summary:
        "Evaluating the potential acquisition of Alpha Corp. Current focus is on identifying operational synergies, verifying the reported 15% CapEx reduction, and assessing environmental liabilities at the primary manufacturing site.",
      thesis:
        "Synthesis suggests a strong acquisition target based on operational synergies. The automated packing line represents a significant competitive moat, provided the transition risks are mitigated through a phased integration. Alpha Corp's market positioning in the manufacturing sector offers a strategic foothold for expansion into vertical markets.",
      timeline: [
        {
          detail: '"EBITDA margins improved by 200bps driven by supply chain optimizations..."',
          id: "alpha-note",
          timestamp: "Today, 10:30 AM",
          title: 'Extracted Note from "Q3 Financials"',
          tone: "accent",
        },
        {
          detail: "Pending EPA litigation in Sector 7 facility.",
          id: "alpha-risk",
          timestamp: "Yesterday, 4:15 PM",
          title: "Risk Flagged",
          tone: "error",
        },
        {
          detail: "Uploaded and auto-transcribed. 4 highlights added.",
          id: "alpha-transcript",
          timestamp: "Oct 24, 2:00 PM",
          title: "CEO Site Visit Transcript",
          tone: "primary",
        },
        {
          detail: "Project kickoff.",
          id: "alpha-kickoff",
          timestamp: "Oct 22, 9:00 AM",
          title: "Data Room Initialized",
          tone: "muted",
        },
      ],
    },
  },
  {
    colorClassName: "bg-[#6b87c8]",
    room: {
      id: "project-beta",
      keyQuestions: [
        "How durable are the distributor agreements across the top three enterprise accounts?",
        "Is margin expansion being driven by one-off pricing actions or structural improvements?",
        "What implementation risk exists if the ERP migration slips beyond Q1?",
      ],
      metrics: [
        { label: "Files Analyzed", value: "29" },
        { label: "Insights Extracted", value: "11" },
        { label: "Critical Risks", tone: "error", value: "2" },
      ],
      name: "Project Beta",
      overviewSubtitle: "Project Beta Commercial Review",
      pendingTasks: [
        { done: true, id: "beta-pricing", label: "Validate pricing bridge with finance team" },
        { id: "beta-contracts", label: "Review distributor penalty clauses" },
        { id: "beta-erp", label: "Confirm ERP cutover contingency plan", priority: true },
      ],
      phaseLabel: "Phase 2",
      sectorLabel: "Industrial Distribution",
      stageLabel: "Under Review",
      summary:
        "Assessing Beta Holdings as a platform acquisition with particular emphasis on channel concentration, pricing resilience, and execution risk tied to the in-flight ERP modernization program.",
      thesis:
        "Beta appears compelling if channel dependence can be reshaped through contract renegotiation and a staged digital integration plan. The upside is concentrated in operating discipline rather than pure topline acceleration, which makes execution readiness the central investment variable.",
      timeline: [
        {
          detail: "Contract renewal draft shows tighter minimum-volume thresholds for the largest distributor.",
          id: "beta-legal",
          timestamp: "Today, 9:10 AM",
          title: "Legal Review Updated",
          tone: "accent",
        },
        {
          detail: "ERP testing backlog grew by 14 unresolved items this week.",
          id: "beta-tech",
          timestamp: "Yesterday, 2:45 PM",
          title: "Implementation Risk Raised",
          tone: "error",
        },
        {
          detail: "Commercial diligence memo uploaded with notes from the revenue operations lead.",
          id: "beta-memo",
          timestamp: "Oct 25, 1:15 PM",
          title: "Memo Added",
          tone: "primary",
        },
        {
          detail: "External data room permissions approved for counsel.",
          id: "beta-room",
          timestamp: "Oct 23, 11:20 AM",
          title: "Room Access Expanded",
          tone: "muted",
        },
      ],
    },
  },
  {
    colorClassName: "bg-muted",
    room: {
      id: "logistics-merger",
      keyQuestions: [
        "What network overlaps create the largest immediate synergy opportunities?",
        "Which unionized depots create the biggest integration constraints?",
        "How quickly can cross-dock utilization improve without service degradation?",
      ],
      metrics: [
        { label: "Files Analyzed", value: "34" },
        { label: "Insights Extracted", value: "14" },
        { label: "Critical Risks", tone: "error", value: "4" },
      ],
      name: "Logistics Merger",
      overviewSubtitle: "Logistics Merger Integration Hub",
      pendingTasks: [
        { id: "logistics-union", label: "Summarize union negotiations by depot" },
        { id: "logistics-fleet", label: "Reconcile fleet maintenance assumptions" },
        { id: "logistics-routing", label: "Draft day-one routing risk memo", priority: true },
      ],
      phaseLabel: "Integration Planning",
      sectorLabel: "Transportation & Logistics",
      stageLabel: "Risk Watch",
      summary:
        "Preparing the integration case for a regional logistics merger. The current workstream is focused on route overlap, depot labor complexity, and validating the modeled service-level improvements against historical throughput.",
      thesis:
        "The merger can unlock strong density economics if the organization avoids forcing integration ahead of depot-level readiness. The highest value is in rationalizing overlapping routes and maintenance spend, but only if labor relations and customer service continuity are tightly managed.",
      timeline: [
        {
          detail: "Network model now identifies 12 overlapping lanes with same-day consolidation potential.",
          id: "logistics-network",
          timestamp: "Today, 8:25 AM",
          title: "Synergy Model Refreshed",
          tone: "accent",
        },
        {
          detail: "Two depots require separate labor consultations before schedule redesign.",
          id: "logistics-labor",
          timestamp: "Yesterday, 5:05 PM",
          title: "Labor Constraint Flagged",
          tone: "error",
        },
        {
          detail: "Operations leads added comments on cross-dock throughput assumptions.",
          id: "logistics-ops",
          timestamp: "Oct 24, 3:40 PM",
          title: "Operations Notes Added",
          tone: "primary",
        },
        {
          detail: "Merger workspace created and baseline files imported.",
          id: "logistics-start",
          timestamp: "Oct 21, 10:00 AM",
          title: "Workspace Initialized",
          tone: "muted",
        },
      ],
    },
  },
];

export const workspaceInitiatives: WorkspaceSidebarTool[] = [
  { icon: "personSearch", name: "Q4 Recruiting" },
  { icon: "terminal", name: "Software Migration" },
];

export const workspaceTools: WorkspaceSidebarTool[] = [
  { icon: "timeline", name: "Meeting Timeline" },
  { icon: "folderOpen", name: "Global Vault" },
];

export function getDealById(dealId: string) {
  return workspaceDeals.find((deal) => deal.room.id === dealId);
}

export function getDealRoomPath(dealId: string) {
  return `/hub/deals/${dealId}`;
}

export function getTeamLabel(email?: string) {
  const teamName = email?.split("@")[0]?.replace(/[._-]/g, " ") ?? "Analyst Team";

  return teamName.replace(/\b\w/g, (character) => character.toUpperCase());
}
