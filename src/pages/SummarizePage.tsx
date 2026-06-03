import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { FormEvent, useState } from "react";
import ReactMarkdown from "react-markdown";
import { WorkspaceHomeShell } from "../components/hub/WorkspaceHomeShell";
import { ChatPanel } from "../components/summarize/ChatPanel";
import { PanelTab } from "../components/summarize/PanelTab";
import { Icon } from "../components/ui/Icon";

type ActivePanel = "chat" | "summary";

export function SummarizePage() {
  const [activePanel, setActivePanel] = useState<ActivePanel>("summary");
  const [error, setError] = useState("");
  const [isSummarizing, setIsSummarizing] = useState(false);
  const [summary, setSummary] = useState("");
  const [selectedPath, setSelectedPath] = useState("");

  async function handleBrowse(directory: boolean) {
    const selection = await open({
      directory,
      fileAccessMode: "scoped",
      multiple: false,
      title: directory ? "Choose a folder to summarize" : "Choose a file to summarize",
    });

    if (typeof selection === "string") {
      setSelectedPath(selection);
    }
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const path = selectedPath.trim();
    if (!path) {
      return;
    }

    setError("");
    setSummary("");
    setIsSummarizing(true);

    try {
      const result = await invoke<string>("summarize", { payload: { path } });
      setSummary(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsSummarizing(false);
    }
  }

  async function handleSaveSummary() {
    if (!summary) {
      return;
    }

    const path = await save({
      defaultPath: "summary.md",
      filters: [{ extensions: ["md", "markdown"], name: "Markdown" }],
      title: "Save markdown summary",
    });

    if (!path) {
      return;
    }

    try {
      await invoke("save_markdown_summary", { payload: { path, summary } });
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }

  return (
    <WorkspaceHomeShell activeHomeSection="summarize">
      <div className="mx-auto flex w-full max-w-[1120px] flex-col pb-10">
        <form
          className="flex min-h-16 w-full items-center gap-3 rounded-full border border-white/85 bg-white/82 px-6 py-3 text-text-main shadow-[0_12px_34px_rgba(28,40,38,0.07)] backdrop-blur-md"
          onSubmit={handleSubmit}
        >
          <Icon className="h-6 w-6 shrink-0 text-primary" name="search" />
          <input
            className="min-w-0 flex-1 bg-transparent text-[15px] text-text-main outline-none placeholder:text-muted"
            onChange={(event) => setSelectedPath(event.target.value)}
            placeholder="Search or browse files in Finder..."
            value={selectedPath}
          />
          <button
            className="shrink-0 rounded-full border border-primary/18 bg-primary/8 px-5 py-2 text-[12px] font-semibold text-primary transition hover:bg-primary/12"
            onClick={() => void handleBrowse(false)}
            type="button"
          >
            Browse File
          </button>
          <button
            className="shrink-0 rounded-full border border-primary/18 bg-primary/8 px-5 py-2 text-[12px] font-semibold text-primary transition hover:bg-primary/12"
            onClick={() => void handleBrowse(true)}
            type="button"
          >
            Browse Folder
          </button>
          <button
            className="shrink-0 rounded-full bg-primary px-5 py-2 text-[12px] font-semibold text-white transition hover:bg-primary-container disabled:cursor-not-allowed disabled:bg-primary/35"
            disabled={!selectedPath.trim() || isSummarizing}
            type="submit"
          >
            {isSummarizing ? "Summarizing" : "Submit"}
          </button>
        </form>

        <div className="mt-5 flex justify-center">
          <div className="inline-flex rounded-full border border-white/85 bg-white/68 p-1 shadow-[0_10px_28px_rgba(28,40,38,0.05)]">
            <PanelTab active={activePanel === "summary"} icon="sparkles" label="Summary" onClick={() => setActivePanel("summary")} />
            <PanelTab active={activePanel === "chat"} icon="send" label="Chat" onClick={() => setActivePanel("chat")} />
          </div>
        </div>

        {activePanel === "summary" ? (
          <div aria-live="polite" className="mt-6 min-h-[calc(100vh-210px)]">
            {error ? <p className="px-6 text-[13px] font-semibold text-error">{error}</p> : null}
            {isSummarizing ? <SummaryLoadingState /> : null}
            {summary ? (
              <div className="vault-markdown relative rounded-[28px] border border-white/85 bg-white/76 p-8 pr-16 text-[15px] leading-7 text-text-main shadow-[0_12px_34px_rgba(28,40,38,0.05)]">
                <button
                  aria-label="Save markdown summary"
                  className="absolute right-5 top-5 flex h-10 w-10 items-center justify-center rounded-full text-primary transition hover:bg-primary/8"
                  onClick={() => void handleSaveSummary()}
                  title="Save markdown summary"
                  type="button"
                >
                  <Icon className="h-5 w-5" name="bookmark" />
                </button>
                <ReactMarkdown>{summary}</ReactMarkdown>
              </div>
            ) : null}
          </div>
        ) : (
          <ChatPanel />
        )}
      </div>
    </WorkspaceHomeShell>
  );
}

function SummaryLoadingState() {
  return (
    <div className="flex min-h-[220px] items-center justify-center rounded-[28px] border border-white/85 bg-white/64 shadow-[0_12px_34px_rgba(28,40,38,0.05)] backdrop-blur-md">
      <div className="flex items-center gap-4 text-primary">
        <span className="h-7 w-7 animate-spin rounded-full border-2 border-primary/18 border-t-primary" />
        <span className="text-[14px] font-semibold">Summarizing documents...</span>
      </div>
    </div>
  );
}
