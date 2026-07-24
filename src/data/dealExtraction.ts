import type { WorkspaceDeal, WorkspaceLocationState } from "./workspace";

export type SaveDealAndExtractInput = {
  buyerOrPlatformCompany: string | null;
  carveOutBusiness: string | null;
  dealName: string;
  dealType: string;
  mainDataRoomFolder: string;
  parentOrSellerCompany: string | null;
  peFirm: string;
  targetCompany: string | null;
};

export type DealExtractionSourceFile = {
  filename: string;
  matchedOn: string[];
  path: string;
  relativePath: string;
  sizeBytes: number;
  textExtracted: boolean;
  textTruncated: boolean;
};

export type SavedDeal = {
  buyerOrPlatformCompany: string | null;
  carveOutBusiness: string | null;
  createdAt: string;
  dealName: string;
  dealType: string;
  id: number;
  mainDataRoomFolder: string;
  parentOrSellerCompany: string | null;
  peFirm: string;
  status: string;
  targetCompany: string | null;
  updatedAt: string;
};

export type DealExtractionResult = {
  investmentThesis: string;
  keyQuestions: string[];
};

export type SavedDealMetadata = {
  createdAt: string;
  dataRoomSizeBytes: number;
  dealId: number;
  documentCount: number;
  id: number;
  investmentThesis: string;
  keyQuestionsJson: string;
  updatedAt: string;
};

export type SaveDealAndExtractResponse = {
  deal: SavedDeal;
  extraction: DealExtractionResult;
  files: DealExtractionSourceFile[];
  metadata: SavedDealMetadata;
};

export type SaveDealAndFindFilesResponse = {
  deal: SavedDeal;
  files: DealExtractionSourceFile[];
};

export type ExtractDealQuestionsAndThesisInput = {
  dealId: number;
  projectTimelineFilePath: string | null;
  sowFilePath: string | null;
};

export type DealExtractionLocationState = WorkspaceLocationState & {
  result?: SaveDealAndExtractResponse;
};

export function buildWorkspaceDealFromExtractionResult(result: SaveDealAndExtractResponse): WorkspaceDeal {
  const keyQuestions = result.extraction.keyQuestions;
  const investmentThesis = result.extraction.investmentThesis.trim();
  const insightCount = keyQuestions.length + (investmentThesis ? 1 : 0);

  return {
    colorClassName: "bg-primary",
    complete: true,
    room: {
      dealType: result.deal.dealType,
      id: String(result.deal.id),
      keyQuestions,
      metrics: [
        { label: "Files Analyzed", value: String(result.metadata.documentCount) },
        { label: "Insights Extracted", value: String(insightCount) },
        { label: "Data Room Size", value: formatCompactFileSize(result.metadata.dataRoomSizeBytes) },
      ],
      name: result.deal.dealName,
      overviewSubtitle: `${result.deal.dealName} Due Diligence Overview`,
      pendingTasks: [],
      phaseLabel: "Phase 1",
      sectorLabel: result.deal.dealType,
      stageLabel: "In Progress",
      summary: buildDealSummary(result.deal),
      thesis: investmentThesis || "No investment thesis has been extracted yet.",
      timeline: [],
    },
  };
}

function buildDealSummary(deal: SavedDeal) {
  const company = deal.targetCompany ?? deal.carveOutBusiness ?? deal.buyerOrPlatformCompany ?? deal.dealName;

  return `Evaluating ${company} for ${deal.peFirm}. Current focus is on reviewing the selected diligence materials, extracting key questions, and building the initial investment narrative.`;
}

function formatCompactFileSize(sizeBytes: number) {
  if (sizeBytes < 1024) {
    return `${sizeBytes} B`;
  }

  if (sizeBytes < 1024 * 1024) {
    return `${(sizeBytes / 1024).toFixed(1)} KB`;
  }

  if (sizeBytes < 1024 * 1024 * 1024) {
    return `${(sizeBytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  return `${(sizeBytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}
