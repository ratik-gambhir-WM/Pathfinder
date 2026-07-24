import { useEffect, useState } from "react";
import { Navigate, useParams } from "react-router-dom";
import { ActivityTimelineCard } from "../components/deal-room/ActivityTimelineCard";
import { DealRoomHeader } from "../components/deal-room/DealRoomHeader";
import { DealSummaryCard } from "../components/deal-room/DealSummaryCard";
import { DealTimelineView } from "../components/deal-room/DealTimelineView";
import { PendingTasksTimelineCard } from "../components/deal-room/PendingTasksTimelineCard";
import { InsightsStrip } from "../components/hub/InsightsStrip";
import { WorkspaceLayout } from "../components/hub/WorkspaceLayout";
import { WorkspaceSidebar } from "../components/hub/WorkspaceSidebar";
import { Button } from "../components/ui/Button";
import { Icon } from "../components/ui/Icon";
import { getDealById, workspaceDeals, workspaceInsights } from "../data/workspace";
import type { DealRoomData, DealTimelineItem } from "../data/workspace";
import { useWorkspaceSession } from "../hooks/useWorkspaceSession";

type ActiveDealView = "deal-room" | "diligence-graph" | "site-visits" | "timeline";

export function DealRoomPage() {
  const { dealId } = useParams();
  const deal = dealId ? getDealById(dealId) : undefined;
  const { email, navigationState } = useWorkspaceSession();
  const [activeDealView, setActiveDealView] = useState<ActiveDealView>("deal-room");
  const [timelineItems, setTimelineItems] = useState<DealTimelineItem[]>([]);
  const dealInsights = workspaceInsights.filter((insight) => insight.deal === deal?.room.name);

  useEffect(() => {
    setTimelineItems(deal?.room.timeline ?? []);
  }, [deal?.room.id, deal?.room.timeline]);

  if (!deal) {
    return <Navigate replace to="/hub" />;
  }

  return (
    <WorkspaceLayout
      sidebar={
        <WorkspaceSidebar
          activeDealId={deal.room.id}
          activeSection={activeDealView}
          deals={workspaceDeals}
          email={email}
          mode="deal-room"
          navigationState={navigationState}
          onDealRoomSectionChange={setActiveDealView}
        />
      }
    >
      <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
        {activeDealView === "timeline" ? (
          <DealTimelineView deal={deal.room} events={timelineItems} onEventsChange={setTimelineItems} />
        ) : activeDealView === "diligence-graph" ? (
          <DiligenceGraphView deal={deal.room} />
        ) : activeDealView === "site-visits" ? (
          <SiteVisitsView deal={deal.room} />
        ) : (
          <>
            <DealRoomHeader subtitle={deal.room.overviewSubtitle} />

            <div className="grid grid-cols-12 gap-6">
              <DealSummaryCard deal={deal.room} />
              <div className="relative col-span-12 xl:col-span-4">
                <div className="absolute inset-x-0 bottom-full z-10 mb-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-2">
                  <button className="inline-flex h-12 items-center justify-center gap-3 rounded-full border border-white/80 bg-white/72 px-6 text-[14px] font-semibold text-text-main shadow-[0_8px_20px_rgba(7,1,84,0.05)] transition hover:bg-white">
                    <Icon className="h-5 w-5 text-muted" name="refresh" />
                    Sync Data Room
                  </button>

                  <Button className="h-12 px-6" icon={<Icon className="h-5 w-5" name="plus" />}>
                    New Note
                  </Button>
                </div>

                <PendingTasksTimelineCard key={deal.room.id} tasks={deal.room.pendingTasks} />
              </div>
              <InsightsStrip
                className="col-span-12 mt-2"
                contextLabel={deal.room.name}
                items={dealInsights}
              />
              <ActivityTimelineCard className="col-span-12 flex min-h-[540px] flex-col rounded-[28px] p-6" items={timelineItems} />
            </div>
          </>
        )}
      </div>
    </WorkspaceLayout>
  );
}

type SiteVisitsViewProps = {
  deal: DealRoomData;
};

function SiteVisitsView({ deal }: SiteVisitsViewProps) {
  const siteVisits = [
    {
      date: "Oct 8",
      location: "Primary Manufacturing Site",
      owner: "Operations diligence lead",
      status: "Scheduled",
      summary: "Walk production floor, validate automated packing line throughput, and inspect CapEx project completion.",
    },
    {
      date: "Oct 11",
      location: "Secondary Environmental Site",
      owner: "Risk workstream",
      status: "Needs prep",
      summary: "Review environmental controls, open remediation questions, and local compliance documentation.",
    },
    {
      date: "Oct 15",
      location: "Corporate HQ",
      owner: "Management interview team",
      status: "Draft agenda",
      summary: "Meet finance, HR, and sales leaders to confirm synergy assumptions and retention risks.",
    },
  ];

  return (
    <>
      <DealRoomHeader subtitle={`${deal.name} field diligence`} />

      <section className="workspace-card rounded-[28px] p-8">
        <div className="flex flex-col gap-5 md:flex-row md:items-start md:justify-between">
          <div>
            <p className="text-[11px] font-bold uppercase tracking-[0.2em] text-muted">Site Visits</p>
            <h1 className="mt-3 text-[2.6rem] font-bold leading-none text-text-main [font-family:var(--font-heading)]">
              Field Diligence Plan
            </h1>
            <p className="mt-4 max-w-2xl text-[15px] leading-7 text-text-main/78">
              Coordinate in-person reviews, owners, and open prep work for {deal.name}.
            </p>
          </div>
          <button className="inline-flex h-11 items-center justify-center gap-2 rounded-full bg-primary px-5 text-[13px] font-bold text-white shadow-[0_10px_26px_rgba(80,101,142,0.24)] transition hover:bg-primary-container">
            <Icon className="h-4 w-4" name="plus" />
            New Visit
          </button>
        </div>

        <div className="mt-8 grid gap-4">
          {siteVisits.map((visit) => (
            <article
              className="grid gap-5 rounded-[24px] border border-outline-variant bg-white/62 p-5 shadow-[0_8px_20px_rgba(7,1,84,0.04)] md:grid-cols-[7rem_1fr_auto]"
              key={visit.location}
            >
              <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/10 text-primary">
                <Icon className="h-7 w-7" name="person" />
              </div>
              <div className="min-w-0">
                <p className="text-[11px] font-bold uppercase tracking-[0.16em] text-muted">{visit.date}</p>
                <h2 className="mt-1 text-[1.35rem] font-bold text-text-main [font-family:var(--font-heading)]">{visit.location}</h2>
                <p className="mt-2 text-[14px] leading-6 text-text-main/78">{visit.summary}</p>
                <p className="mt-3 text-[12px] font-semibold text-muted">Owner: {visit.owner}</p>
              </div>
              <div className="flex items-start md:justify-end">
                <span className="rounded-full border border-outline-variant bg-white/72 px-3 py-1.5 text-[11px] font-bold uppercase tracking-[0.12em] text-primary">
                  {visit.status}
                </span>
              </div>
            </article>
          ))}
        </div>
      </section>
    </>
  );
}

type DiligenceGraphViewProps = {
  deal: DealRoomData;
};

function DiligenceGraphView({ deal }: DiligenceGraphViewProps) {
  const graphNodes = [
    { label: deal.name, tone: "primary", x: "50%", y: "18%" },
    { label: "Financial Performance", tone: "accent", x: "20%", y: "48%" },
    { label: "Legal & Compliance", tone: "error", x: "50%", y: "54%" },
    { label: "Operations", tone: "muted", x: "80%", y: "48%" },
    { label: "Investment Thesis", tone: "primary", x: "50%", y: "82%" },
  ];

  return (
    <>
      <DealRoomHeader subtitle={`${deal.name} relationship map`} />

      <section className="workspace-card relative min-h-[680px] overflow-hidden rounded-[28px] p-8">
        <div className="flex items-start justify-between gap-6">
          <div>
            <p className="text-[11px] font-bold uppercase tracking-[0.2em] text-muted">Diligence Graph</p>
            <h1 className="mt-3 text-[2.6rem] font-bold leading-none text-text-main [font-family:var(--font-heading)]">
              {deal.name} Knowledge Map
            </h1>
            <p className="mt-4 max-w-2xl text-[15px] leading-7 text-text-main/78">
              A working graph for connecting documents, findings, risks, and thesis support across the diligence process.
            </p>
          </div>
          <div className="hidden rounded-full border border-white/80 bg-white/72 px-4 py-2 text-[12px] font-semibold uppercase tracking-[0.14em] text-muted md:block">
            Prototype View
          </div>
        </div>

        <div className="relative mt-10 h-[460px] rounded-[28px] border border-outline-variant/70 bg-white/52">
          <svg aria-hidden="true" className="absolute inset-0 h-full w-full text-outline-variant" viewBox="0 0 100 100" preserveAspectRatio="none">
            <path className="fill-none stroke-current stroke-[0.35]" d="M50 18 L20 48 L50 82 L80 48 L50 18" />
            <path className="fill-none stroke-current stroke-[0.3]" d="M20 48 L50 54 L80 48 M50 18 L50 54 L50 82" />
          </svg>

          {graphNodes.map((node) => (
            <div
              className="absolute flex -translate-x-1/2 -translate-y-1/2 flex-col items-center gap-2"
              key={node.label}
              style={{ left: node.x, top: node.y }}
            >
              <div
                className={`flex h-16 w-16 items-center justify-center rounded-full border-2 border-white text-white shadow-[0_14px_34px_rgba(7,1,84,0.16)] ${
                  node.tone === "error" ? "bg-error" : node.tone === "accent" ? "bg-accent" : node.tone === "muted" ? "bg-muted" : "bg-primary-container"
                }`}
              >
                <Icon className="h-7 w-7" name={node.label === deal.name ? "dataset" : "grid"} />
              </div>
              <span className="rounded-full border border-outline-variant bg-white/90 px-3 py-1 text-[12px] font-semibold text-text-main shadow-sm">
                {node.label}
              </span>
            </div>
          ))}
        </div>
      </section>
    </>
  );
}
