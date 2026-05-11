import { useState } from "react";
import { DataRoomTreeNode } from "../../data/dataRoom";
import { Button } from "../ui/Button";
import { Icon } from "../ui/Icon";

type DataRoomExplorerProps = {
  dealName: string;
  nodes: DataRoomTreeNode[];
};

export function DataRoomExplorer({ dealName, nodes }: DataRoomExplorerProps) {
  return (
    <aside className="flex w-72 shrink-0 flex-col gap-3 border-r border-white/80 bg-white/40 p-4 backdrop-blur-md">
      <div className="mb-3 flex items-center gap-4 px-3 py-3">
        <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
          <Icon className="h-7 w-7" name="dataset" />
        </div>
        <div>
          <h2 className="text-[1.65rem] font-bold leading-none text-text-main [font-family:var(--font-heading)]">
            {dealName}
          </h2>
          <p className="mt-1 text-[10px] font-bold uppercase tracking-[0.22em] text-muted">Due Diligence</p>
        </div>
      </div>

      <Button className="mb-5 h-14 px-6" icon={<Icon className="h-5 w-5" name="plus" />}>
        New Analysis
      </Button>

      <div className="workspace-scrollbar-hidden min-h-0 flex-1 overflow-y-auto pr-1">
        <div className="space-y-1">
          {nodes.map((node) => (
            <ExplorerNodeItem depth={0} key={node.id} node={node} />
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
            <span className="text-[10px] font-bold uppercase tracking-[0.18em] text-muted">{dealName}</span>
          </div>
        </div>
      </div>
    </aside>
  );
}

type ExplorerNodeItemProps = {
  depth: number;
  node: DataRoomTreeNode;
};

function ExplorerNodeItem({ depth, node }: ExplorerNodeItemProps) {
  const [expanded, setExpanded] = useState(Boolean(node.defaultExpanded));
  const hasChildren = Boolean(node.children?.length);

  return (
    <div>
      <button
        className="flex w-full items-start gap-2 rounded-lg px-2 py-1.5 text-left transition hover:bg-white/40"
        onClick={() => {
          if (hasChildren) {
            setExpanded((value) => !value);
          }
        }}
        style={{ paddingLeft: `${depth * 18 + 8}px` }}
        type="button"
      >
        <span className="mt-0.5 flex h-5 w-5 items-center justify-center text-muted">
          {hasChildren ? <Icon className="h-4 w-4" name={expanded ? "chevronDown" : "chevronRight"} /> : null}
        </span>
        <span className="mt-0.5 text-primary">
          <Icon className="h-[18px] w-[18px]" name={iconNameForNode(node.kind)} />
        </span>
        <span className={`truncate ${hasChildren ? "text-sm font-medium text-text-main" : "text-[13px] text-text-main/80"}`}>
          {node.name}
        </span>
      </button>

      {hasChildren && expanded ? (
        <div className="space-y-0.5">
          {node.children?.map((child) => (
            <ExplorerNodeItem depth={depth + 1} key={child.id} node={child} />
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
