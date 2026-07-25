import { useEffect, useState } from "react";
import { Navigate, useLocation, useParams } from "react-router-dom";
import { ActivityTimelineCard } from "../components/deal-room/ActivityTimelineCard";
import { DealRoomHeader } from "../components/deal-room/DealRoomHeader";
import { DealSummaryCard } from "../components/deal-room/DealSummaryCard";
import { DealTimelineView } from "../components/deal-room/DealTimelineView";
import { UnderConstructionView } from "../components/deal-room/UnderConstructionView";
import { InsightsStrip } from "../components/hub/InsightsStrip";
import { WorkspaceLayout } from "../components/hub/WorkspaceLayout";
import { WorkspaceSidebar } from "../components/hub/WorkspaceSidebar";
import type { DealExtractionLocationState } from "../data/dealExtraction";
import { buildWorkspaceDealFromExtractionResult } from "../data/dealExtraction";
import { getDealById, workspaceDeals, workspaceInsights } from "../data/workspace";
import type { DealTimelineItem } from "../data/workspace";
import { useWorkspaceSession } from "../hooks/useWorkspaceSession";

type ActiveDealView = "deal-room" | "diligence-graph" | "site-visits" | "synthesis-canvas" | "timeline";

export function DealRoomPage() {
  const { dealId } = useParams();
  const location = useLocation();
  const extractionResult = (location.state as DealExtractionLocationState | null)?.result;
  const extractedDeal =
    extractionResult && String(extractionResult.deal.id) === dealId
      ? buildWorkspaceDealFromExtractionResult(extractionResult)
      : undefined;
  const deal = extractedDeal ?? (dealId ? getDealById(dealId) : undefined);
  const { email, navigationState } = useWorkspaceSession();
  const [activeDealView, setActiveDealView] = useState<ActiveDealView>("deal-room");
  const [timelineItems, setTimelineItems] = useState<DealTimelineItem[]>([]);
  const dealInsights = workspaceInsights.filter((insight) => insight.deal === deal?.room.name);
  const deals = extractedDeal
    ? [extractedDeal, ...workspaceDeals.filter((workspaceDeal) => workspaceDeal.room.id !== extractedDeal.room.id)]
    : workspaceDeals;
  const dealNavigationState = extractionResult
    ? ({
        ...navigationState,
        result: extractionResult,
      } satisfies DealExtractionLocationState)
    : navigationState;

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
          deals={deals}
          email={email}
          mode="deal-room"
          navigationState={dealNavigationState}
          onDealRoomSectionChange={setActiveDealView}
        />
      }
    >
      <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
        {activeDealView === "timeline" ? (
          <DealTimelineView deal={deal.room} events={timelineItems} onEventsChange={setTimelineItems} />
        ) : activeDealView === "diligence-graph" ? (
          <UnderConstructionView
            description="Evidence relationships and dependency mapping for this deal."
            icon="graph"
            title="Diligence Graph"
          />
        ) : activeDealView === "site-visits" ? (
          <UnderConstructionView
            description="Planning templates and visit notes for diligence fieldwork."
            icon="person"
            title="Site Visits"
          />
        ) : activeDealView === "synthesis-canvas" ? (
          <UnderConstructionView
            description="A working canvas for combining findings, risks, and recommendations."
            icon="grid"
            title="Synthesis Canvas"
          />
        ) : (
          <>
            <DealRoomHeader subtitle={deal.room.overviewSubtitle} />

            <div className="grid grid-cols-12 gap-6">
              <DealSummaryCard deal={deal.room} />
              <InsightsStrip
                className="col-span-12 mt-2"
                contextLabel={deal.room.name}
                items={dealInsights}
              />
              <ActivityTimelineCard className="col-span-12 flex min-h-[540px] flex-col p-6" items={timelineItems} />
            </div>
          </>
        )}
      </div>
    </WorkspaceLayout>
  );
}
