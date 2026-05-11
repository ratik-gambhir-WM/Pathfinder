import { Navigate, useParams } from "react-router-dom";
import { ChipBankPanel } from "../components/data-room/ChipBankPanel";
import { DataRoomExplorer } from "../components/data-room/DataRoomExplorer";
import { DataRoomTopBar } from "../components/data-room/DataRoomTopBar";
import { ReportEditorPanel } from "../components/data-room/ReportEditorPanel";
import { getDealDataRoomView } from "../data/dataRoom";
import { getDealById, getTeamLabel } from "../data/workspace";
import { useWorkspaceSession } from "../hooks/useWorkspaceSession";

export function DataRoomPage() {
  const { dealId } = useParams();
  const deal = dealId ? getDealById(dealId) : undefined;
  const { email } = useWorkspaceSession();

  if (!deal) {
    return <Navigate replace to="/hub" />;
  }

  const dataRoomView = getDealDataRoomView(deal.room);
  const teamInitial = getTeamLabel(email).slice(0, 1);

  return (
    <div className="relative min-h-screen overflow-hidden bg-background text-on-surface">
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-[15%] top-[22%] h-[22rem] w-[22rem] rounded-full bg-tertiary-fixed/16 blur-3xl" />
        <div className="absolute right-[10%] top-[12%] h-[26rem] w-[26rem] rounded-full bg-primary-fixed/18 blur-3xl" />
        <div className="absolute bottom-[8%] left-[30%] h-[30rem] w-[30rem] rounded-full bg-surface-container-high/70 blur-3xl" />
      </div>

      <div className="relative z-10">
        <DataRoomTopBar teamInitial={teamInitial} />

        <div className="flex h-[calc(100vh-72px)]">
          <DataRoomExplorer dealName={deal.room.name} nodes={dataRoomView.tree} />

          <main className="flex min-w-0 flex-1 gap-6 overflow-hidden p-6">
            <ChipBankPanel chips={dataRoomView.chips} />
            <ReportEditorPanel
              blocks={dataRoomView.editorBlocks}
              reportTitle={dataRoomView.reportTitle}
              versionLabel={dataRoomView.versionLabel}
            />
          </main>
        </div>
      </div>
    </div>
  );
}
