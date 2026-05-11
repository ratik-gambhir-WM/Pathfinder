import { Navigate, useParams } from "react-router-dom";
import { ActivityTimelineCard } from "../components/deal-room/ActivityTimelineCard";
import { DealRoomHeader } from "../components/deal-room/DealRoomHeader";
import { DealSummaryCard } from "../components/deal-room/DealSummaryCard";
import { PendingTasksTimelineCard } from "../components/deal-room/PendingTasksTimelineCard";
import { WorkspaceLayout } from "../components/hub/WorkspaceLayout";
import { WorkspaceSidebar } from "../components/hub/WorkspaceSidebar";
import { Button } from "../components/ui/Button";
import { Icon } from "../components/ui/Icon";
import { getDealById, workspaceDeals } from "../data/workspace";
import { useWorkspaceSession } from "../hooks/useWorkspaceSession";

export function DealRoomPage() {
  const { dealId } = useParams();
  const deal = dealId ? getDealById(dealId) : undefined;
  const { email, navigationState } = useWorkspaceSession();

  if (!deal) {
    return <Navigate replace to="/hub" />;
  }

  return (
    <WorkspaceLayout
      sidebar={
        <WorkspaceSidebar
          activeDealId={deal.room.id}
          activeSection="deal-room"
          deals={workspaceDeals}
          email={email}
          mode="deal-room"
          navigationState={navigationState}
        />
      }
    >
      <div className="mx-auto flex w-full max-w-[1440px] flex-col gap-6 pb-10">
        <DealRoomHeader subtitle={deal.room.overviewSubtitle} />

        <div className="grid grid-cols-12 gap-6">
          <DealSummaryCard deal={deal.room} />
          <div className="relative col-span-12 xl:col-span-4">
            <div className="absolute inset-x-0 bottom-full z-10 mb-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-2">
              <button className="inline-flex h-12 items-center justify-center gap-3 rounded-full border border-white/80 bg-white/72 px-6 text-[14px] font-semibold text-text-main shadow-[0_8px_20px_rgba(28,40,38,0.05)] transition hover:bg-white">
                <Icon className="h-5 w-5 text-muted" name="refresh" />
                Sync Data Room
              </button>

              <Button className="h-12 px-6" icon={<Icon className="h-5 w-5" name="plus" />}>
                New Note
              </Button>
            </div>

            <PendingTasksTimelineCard key={deal.room.id} tasks={deal.room.pendingTasks} />
          </div>
          <ActivityTimelineCard className="col-span-12 flex min-h-[540px] flex-col rounded-[28px] p-6" items={deal.room.timeline} />
        </div>
      </div>
    </WorkspaceLayout>
  );
}
