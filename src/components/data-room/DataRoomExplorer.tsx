import { useState } from "react";
import { Link } from "react-router-dom";
import { DataRoomTreeNode } from "../../data/dataRoom";
import type { DealExtractionLocationState } from "../../data/dealExtraction";
import { Button } from "../ui/Button";
import { Icon } from "../ui/Icon";
import { DataRoomSidebarTabs } from "./DataRoomSidebarTabs";

type DataRoomExplorerProps = {
  collapsed: boolean;
  dealName: string;
  dealRoomPath: string;
  navigationState?: DealExtractionLocationState;
  nodes: DataRoomTreeNode[];
  onCollapse: () => void;
};

export function DataRoomExplorer({
  collapsed,
  dealName,
  dealRoomPath,
  navigationState,
  nodes,
  onCollapse,
}: DataRoomExplorerProps) {
  const [expandedNodeIds, setExpandedNodeIds] = useState<Set<string>>(() => new Set());

  function toggleNode(nodeId: string) {
    setExpandedNodeIds((current) => {
      const next = new Set(current);

      if (next.has(nodeId)) {
        next.delete(nodeId);
      } else {
        next.add(nodeId);
      }

      return next;
    });
  }

  return (
    <aside
      aria-hidden={collapsed}
      className={`flex shrink-0 overflow-hidden bg-white/40 backdrop-blur-md transition-[width,border-color] duration-300 ${
        collapsed ? "w-0 border-r-0" : "w-72 border-r border-white/80"
      }`}
    >
      <div
        className={`flex h-full w-72 shrink-0 flex-col gap-3 p-4 ${
          collapsed ? "pointer-events-none invisible" : "visible"
        }`}
      >
        <div className="flex items-center justify-between">
          <Link
            aria-label={`Back to ${dealName} deal room`}
            className="flex h-10 w-10 items-center justify-center rounded-full text-primary transition hover:bg-white/70"
            state={navigationState}
            to={dealRoomPath}
          >
            <Icon className="h-5 w-5" name="home" />
          </Link>
          <button
            aria-label="Collapse data room sidebar"
            className="flex h-10 w-10 items-center justify-center rounded-full text-muted transition hover:bg-white/70 hover:text-text-main"
            onClick={onCollapse}
            title="Collapse data room sidebar"
            type="button"
          >
            <Icon className="h-5 w-5" name="chevronLeft" />
          </button>
        </div>

        <div className="mb-3 flex items-center gap-4 px-3 py-3">
          <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
            <Icon className="h-7 w-7" name="dataset" />
          </div>
          <div>
            <h2 className="text-[1.65rem] font-bold leading-none text-text-main [font-family:var(--font-heading)]">
              {dealName}
            </h2>
            <p className="mt-1 text-[11px] font-bold uppercase tracking-[0.18em] text-muted">Due Diligence</p>
          </div>
        </div>

        <Button className="mb-4 h-14 px-6" icon={<Icon className="h-5 w-5" name="plus" />}>
          New Analysis
        </Button>

        <div className="mb-4">
          <DataRoomSidebarTabs activeTab="data-room" />
        </div>

        <div className="workspace-scrollbar-hidden min-h-0 flex-1 overflow-y-auto pr-1">
          <div className="space-y-1">
            {nodes.map((node) => (
              <ExplorerNodeItem
                depth={0}
                expandedNodeIds={expandedNodeIds}
                key={node.id}
                node={node}
                onToggle={toggleNode}
              />
            ))}
          </div>
        </div>

        <div className="mt-auto border-t border-white/50 pt-4">
          <div className="flex items-center gap-3 rounded-xl px-3 py-2 transition hover:bg-white/50">
            <div className="flex h-9 w-9 items-center justify-center rounded-full bg-secondary-fixed-dim text-sm font-semibold text-white">
              A
            </div>
            <div className="flex flex-col">
              <span className="text-sm font-medium text-text-main">Analyst Team</span>
              <span className="text-[11px] font-bold uppercase tracking-[0.14em] text-muted">{dealName}</span>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}

type ExplorerNodeItemProps = {
  depth: number;
  expandedNodeIds: Set<string>;
  node: DataRoomTreeNode;
  onToggle: (nodeId: string) => void;
};

function ExplorerNodeItem({ depth, expandedNodeIds, node, onToggle }: ExplorerNodeItemProps) {
  const hasChildren = Boolean(node.children?.length);
  const expanded = expandedNodeIds.has(node.id);

  return (
    <div>
      <button
        className="flex w-full items-start gap-2 rounded-lg px-2 py-1.5 text-left transition hover:bg-white/40"
        onClick={() => {
          if (hasChildren) {
            onToggle(node.id);
          }
        }}
        style={{ paddingLeft: `${depth * 18 + 8}px` }}
        title={node.name}
        type="button"
      >
        <span className="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center text-muted">
          {hasChildren ? <Icon className="h-4 w-4" name={expanded ? "chevronDown" : "chevronRight"} /> : null}
        </span>
        <span className="mt-0.5 shrink-0 text-primary">
          <Icon className="h-[18px] w-[18px]" name={iconNameForNode(node.kind)} />
        </span>
        <span
          className={`min-w-0 whitespace-normal break-words leading-snug [overflow-wrap:anywhere] ${
            hasChildren ? "text-[14px] font-medium text-text-main" : "text-[14px] text-text-main/80"
          }`}
        >
          {node.name}
        </span>
      </button>

      {hasChildren && expanded ? (
        <div className="space-y-0.5">
          {node.children?.map((child) => (
            <ExplorerNodeItem
              depth={depth + 1}
              expandedNodeIds={expandedNodeIds}
              key={child.id}
              node={child}
              onToggle={onToggle}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
}

function iconNameForNode(kind: DataRoomTreeNode["kind"]) {
  switch (kind) {
    case "folder":
      return "folderOpen";
    case "pdf":
      return "pdf";
    case "sheet":
      return "sheet";
    default:
      return "doc";
  }
}
