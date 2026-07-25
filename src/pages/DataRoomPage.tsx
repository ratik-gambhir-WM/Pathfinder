import { useState } from "react";
import { Navigate, useLocation, useParams } from "react-router-dom";
import { ChipBankPanel } from "../components/data-room/ChipBankPanel";
import { DataRoomExplorer } from "../components/data-room/DataRoomExplorer";
import { ReportEditorPanel } from "../components/data-room/ReportEditorPanel";
import { Icon } from "../components/ui/Icon";
import { getDealDataRoomView } from "../data/dataRoom";
import type { DealExtractionLocationState } from "../data/dealExtraction";
import { buildWorkspaceDealFromExtractionResult } from "../data/dealExtraction";
import { getDealById, getDealRoomPath } from "../data/workspace";

export function DataRoomPage() {
  const { dealId } = useParams();
  const location = useLocation();
  const [isChipBankOpen, setIsChipBankOpen] = useState(true);
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
        <div className="flex h-screen">
          <DataRoomExplorer
            dealName={deal.room.name}
            dealRoomPath={getDealRoomPath(deal.room.id)}
            navigationState={location.state as DealExtractionLocationState | undefined}
            nodes={dataRoomView.tree}
          />

          <main className="relative flex min-w-0 flex-1 gap-0 overflow-hidden p-0">
            <ReportEditorPanel
              blocks={dataRoomView.editorBlocks}
              reportTitle={dataRoomView.reportTitle}
              versionLabel={dataRoomView.versionLabel}
            />
            {isChipBankOpen ? <ChipBankPanel chips={dataRoomView.chips} onCollapse={() => setIsChipBankOpen(false)} /> : null}
            {!isChipBankOpen ? (
              <button
                aria-label="Open document search"
                className="absolute right-0 top-1/2 z-30 flex h-14 w-11 -translate-y-1/2 items-center justify-center border border-r-0 border-outline-variant bg-white/90 text-primary shadow-sm transition hover:bg-white hover:text-text-main"
                onClick={() => setIsChipBankOpen(true)}
                title="Open document search"
                type="button"
              >
                <Icon className="h-6 w-6" name="chevronLeft" />
              </button>
            ) : null}
          </main>
        </div>
      </div>
    </div>
  );
}
