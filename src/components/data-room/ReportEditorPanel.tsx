import { EditorBlock } from "../../data/dataRoom";
import { Icon } from "../ui/Icon";

type ReportEditorPanelProps = {
  blocks: EditorBlock[];
  reportTitle: string;
  versionLabel: string;
};

export function ReportEditorPanel({ blocks, reportTitle, versionLabel }: ReportEditorPanelProps) {
  return (
    <section className="glass-panel flex min-w-0 flex-1 flex-col overflow-hidden rounded-[28px]">
      <div className="workspace-scrollbar-hidden flex-1 overflow-y-auto p-6">
        <div className="mx-auto max-w-4xl py-8">
          <div className="mb-10">
            <div className="mb-4 flex items-center gap-2 text-muted">
              <Icon className="h-5 w-5" name="doc" />
              <span className="text-[10px] font-bold uppercase tracking-[0.2em]">{versionLabel}</span>
            </div>
            <h1 className="mb-6 text-[4rem] font-bold leading-[1.05] tracking-[-0.05em] text-text-main [font-family:var(--font-heading)]">
              {reportTitle}
            </h1>
            <div className="flex items-center gap-4 border-b border-white/50 pb-6">
              <div className="flex -space-x-2">
                <AvatarBadge label="SC" />
                <AvatarBadge label="AT" />
              </div>
              <span className="text-sm text-muted/80">
                Edited 4 minutes ago by <strong className="text-text-main">Sarah Chen</strong>
              </span>
            </div>
          </div>

          <div className="space-y-8 text-[1.02rem] leading-8 text-text-main/90">
            {blocks.map((block) => {
              if (block.type === "paragraph") {
                return <p key={block.id}>{block.text}</p>;
              }

              if (block.type === "heading") {
                return (
                  <h2
                    className="pt-6 text-[3rem] font-bold leading-tight tracking-[-0.04em] text-text-main [font-family:var(--font-heading)]"
                    key={block.id}
                  >
                    {block.text}
                  </h2>
                );
              }

              if (block.type === "quote") {
                return (
                  <blockquote className="rounded-[24px] border border-primary/10 bg-primary/5 p-8 shadow-sm" key={block.id}>
                    <p className="text-[1.35rem] italic leading-9 text-text-main/80 [font-family:var(--font-heading)]">
                      "{block.text}"
                    </p>
                  </blockquote>
                );
              }

              if (block.type === "dropzone") {
                return <DropZone key={block.id} />;
              }

              return <EditorCallouts columns={block.columns} key={block.id} />;
            })}
          </div>
        </div>
      </div>

      <div className="sticky bottom-0 flex justify-center bg-gradient-to-t from-white/80 to-transparent px-6 pb-6 pt-2">
        <div className="flex items-center gap-2 rounded-full border border-white bg-white/90 px-6 py-2 shadow-2xl backdrop-blur-xl">
          <ToolbarButton label="B" />
          <ToolbarButton className="italic" label="I" />
          <ToolbarButton label="•" />
          <div className="mx-2 h-5 w-px bg-muted/20" />
          <button className="flex items-center gap-2 rounded-full px-3 py-1.5 transition hover:bg-primary/5" type="button">
            <Icon className="h-5 w-5 text-primary" name="sparkles" />
            <span className="text-[11px] font-bold uppercase tracking-[0.16em] text-primary">AI Polish</span>
          </button>
          <div className="mx-2 h-5 w-px bg-muted/20" />
          <ToolbarButton label="☰" />
        </div>
      </div>
    </section>
  );
}

type ToolbarButtonProps = {
  className?: string;
  label: string;
};

function ToolbarButton({ className = "", label }: ToolbarButtonProps) {
  return (
    <button
      className={`flex h-10 w-10 items-center justify-center rounded-lg text-[1.1rem] font-semibold text-text-main transition hover:bg-background ${className}`}
      type="button"
    >
      {label}
    </button>
  );
}

type EditorCalloutsProps = {
  columns: Array<{ items: string[]; title: string; tone: "error" | "primary" }>;
};

function EditorCallouts({ columns }: EditorCalloutsProps) {
  return (
    <div className="my-10 grid gap-6 md:grid-cols-2">
      {columns.map((column) => (
        <div className="rounded-[24px] border border-white/80 bg-white/40 p-6 shadow-sm backdrop-blur-sm" key={column.title}>
          <h4
            className={`mb-4 flex items-center gap-2 text-sm font-bold uppercase tracking-[0.16em] ${
              column.tone === "error" ? "text-[#d9534f]" : "text-primary"
            }`}
          >
            <span className="text-lg">{column.tone === "error" ? "!" : "+"}</span>
            {column.title}
          </h4>
          <ul className="space-y-2 text-sm">
            {column.items.map((item) => (
              <li className="flex items-start gap-2" key={item}>
                <span className={column.tone === "error" ? "text-[#d9534f]" : "text-primary"}>•</span>
                <span>{item}</span>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </div>
  );
}

function DropZone() {
  return (
    <div className="group relative py-4">
      <div className="absolute -left-12 top-1/2 flex -translate-y-1/2 gap-2 opacity-0 transition group-hover:opacity-100">
        <button className="text-muted transition hover:text-text-main" type="button">
          ≡
        </button>
        <button className="text-muted transition hover:text-text-main" type="button">
          +
        </button>
      </div>
      <div className="relative h-0.5 w-full rounded-full bg-accent/40">
        <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full bg-accent px-4 py-1 text-[9px] font-bold uppercase tracking-[0.2em] text-white shadow-sm">
          Drop Insight Here
        </div>
      </div>
    </div>
  );
}

type AvatarBadgeProps = {
  label: string;
};

function AvatarBadge({ label }: AvatarBadgeProps) {
  return (
    <div className="flex h-8 w-8 items-center justify-center rounded-full border-2 border-white bg-secondary-fixed-dim text-[11px] font-semibold text-white shadow-sm">
      {label}
    </div>
  );
}
