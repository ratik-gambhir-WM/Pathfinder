import { useState } from "react";
import { Navigate, useLocation, useParams } from "react-router-dom";
import { ChipBankPanel } from "../components/data-room/ChipBankPanel";
import { DataRoomExplorer } from "../components/data-room/DataRoomExplorer";
import { EdgePanelOpenButton } from "../components/data-room/EdgePanelOpenButton";
import { ReportEditorPanel } from "../components/data-room/ReportEditorPanel";
import { getDealDataRoomView } from "../data/dataRoom";
import type { DealExtractionLocationState } from "../data/dealExtraction";
import { buildWorkspaceDealFromExtractionResult } from "../data/dealExtraction";
import { getDealById, getDealRoomPath } from "../data/workspace";

export function DataRoomPage() {
  const { dealId } = useParams();
  const location = useLocation();
  const [isChipBankOpen, setIsChipBankOpen] = useState(true);
  const [isExplorerOpen, setIsExplorerOpen] = useState(true);
  const extractionResult = (location.state as DealExtractionLocationState | null)?.result;
  const extractedDeal =
    extractionResult && String(extractionResult.deal.id) === dealId
      ? buildWorkspaceDealFromExtractionResult(extractionResult)
      : undefined;
  const deal = extractedDeal ?? (dealId ? getDealById(dealId) : undefined);

  if (!deal) {
    return <Navigate replace to="/hub" />;
  }

  const dataRoomView = getDealDataRoomView(deal.room);

  return (
    <div className="relative min-h-screen overflow-hidden bg-background text-on-surface">
      <div className="workspace-ambient pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-[15%] top-[22%] h-[22rem] w-[22rem] rounded-full bg-tertiary-fixed/16 blur-3xl" />
        <div className="absolute right-[10%] top-[12%] h-[26rem] w-[26rem] rounded-full bg-primary-fixed/18 blur-3xl" />
        <div className="absolute bottom-[8%] left-[30%] h-[30rem] w-[30rem] rounded-full bg-surface-container-high/70 blur-3xl" />
      </div>

      <div className="relative z-10">
        <div className="relative flex h-screen">
          <DataRoomExplorer
            collapsed={!isExplorerOpen}
            dealName={deal.room.name}
            dealRoomPath={getDealRoomPath(deal.room.id)}
            navigationState={location.state as DealExtractionLocationState | undefined}
            nodes={dataRoomView.tree}
            onCollapse={() => setIsExplorerOpen(false)}
          />
          {!isExplorerOpen ? (
            <EdgePanelOpenButton
              label="Open data room sidebar"
              onClick={() => setIsExplorerOpen(true)}
              side="left"
            />
          ) : null}

          <main className="relative flex min-w-0 flex-1 gap-0 overflow-hidden p-0">
            <ReportEditorPanel
              blocks={dataRoomView.editorBlocks}
              reportTitle={dataRoomView.reportTitle}
              versionLabel={dataRoomView.versionLabel}
            />
            {isChipBankOpen ? <ChipBankPanel chips={dataRoomView.chips} onCollapse={() => setIsChipBankOpen(false)} /> : null}
            {!isChipBankOpen ? (
              <EdgePanelOpenButton
                label="Open document search"
                onClick={() => setIsChipBankOpen(true)}
                side="right"
              />
            ) : null}
          </main>
        </div>
      </div>
    </div>
  );
}
