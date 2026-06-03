import { FormEvent, useMemo, useState } from "react";
import { DealRoomData, DealTask, DealTimelineItem, DealTimelineTone } from "../../data/workspace";
import { WorkspaceCard } from "../hub/WorkspaceCard";
import { Icon } from "../ui/Icon";

type DealTimelineViewProps = {
  deal: DealRoomData;
  events: DealTimelineItem[];
  onEventsChange: (events: DealTimelineItem[]) => void;
};

type TimelineFormState = {
  category: string;
  date: string;
  detail: string;
  title: string;
};

const initialFormState: TimelineFormState = {
  category: "Meeting",
  date: "",
  detail: "",
  title: "",
};

const categoryOptions = ["Meeting", "Site Visit", "Task", "Milestone", "Risk", "Note"];

const toneByCategory: Record<string, DealTimelineTone> = {
  Meeting: "accent",
  Milestone: "primary",
  Note: "accent",
  Risk: "error",
  "Site Visit": "primary",
  Task: "muted",
};

const timelineToneClasses: Record<DealTimelineTone, { detail: string; dot: string; label: string }> = {
  accent: {
    detail: "border-white/60 bg-white/54 text-text-main/82",
    dot: "bg-tertiary",
    label: "text-muted",
  },
  error: {
    detail: "border-error/20 bg-error-container/18 text-on-error-container italic",
    dot: "bg-error",
    label: "text-error/65",
  },
  muted: {
    detail: "border-white/55 bg-white/42 text-text-main/76",
    dot: "bg-muted",
    label: "text-muted",
  },
  primary: {
    detail: "border-white/60 bg-white/54 text-text-main/82",
    dot: "bg-primary",
    label: "text-muted",
  },
};

export function DealTimelineView({ deal, events, onEventsChange }: DealTimelineViewProps) {
  const [formState, setFormState] = useState<TimelineFormState>(initialFormState);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [toastMessage, setToastMessage] = useState("");

  const sortedEvents = useMemo(
    () =>
      [...events].sort((first, second) => {
        const dateComparison = first.date.localeCompare(second.date);

        if (dateComparison !== 0) {
          return dateComparison;
        }

        return first.title.localeCompare(second.title);
      }),
    [events],
  );

  function openModal() {
    setFormState(initialFormState);
    setIsModalOpen(true);
  }

  function closeModal() {
    setIsModalOpen(false);
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const title = formState.title.trim();
    const detail = formState.detail.trim();

    if (!title || !formState.date) {
      return;
    }

    const timelineItem: DealTimelineItem = {
      category: formState.category,
      date: formState.date,
      detail: detail || "No notes added yet.",
      id: `timeline-${Date.now()}`,
      timestamp: formatTimelineDate(formState.date),
      title,
      tone: toneByCategory[formState.category] ?? "primary",
    };

    onEventsChange([...events, timelineItem]);
    setIsModalOpen(false);
    setToastMessage("Timeline updated successfully");
    window.setTimeout(() => setToastMessage(""), 3200);
  }

  return (
    <>
      <div className="flex flex-col gap-6">
        <header className="flex flex-col gap-2">
          <h1 className="type-display text-text-main">Meeting Timeline</h1>
          <p className="type-subtle text-muted">{deal.overviewSubtitle}</p>
        </header>

        <div className="grid grid-cols-12 gap-6">
          <WorkspaceCard className="col-span-12 min-h-[640px] rounded-[28px] p-8 lg:p-10 xl:col-span-8">
            <div className="mb-10 flex flex-col gap-5 sm:flex-row sm:items-center sm:justify-between">
              <h2 className="type-h1 text-text-main">Activity Timeline</h2>

              <div className="flex items-center gap-3">
                <button
                  className="inline-flex h-12 items-center justify-center gap-2 rounded-full bg-primary/10 px-5 text-[13px] font-bold text-primary transition hover:bg-primary/16 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
                  onClick={openModal}
                  type="button"
                >
                  <Icon className="h-4 w-4" name="plus" />
                  Log Activity
                </button>
                <button
                  aria-label="Timeline actions"
                  className="inline-flex h-11 w-11 items-center justify-center rounded-full text-muted transition hover:bg-white/58 hover:text-text-main"
                  type="button"
                >
                  <Icon className="h-5 w-5" name="more" />
                </button>
              </div>
            </div>

            <div className="workspace-scrollbar-hidden relative max-h-[640px] overflow-y-auto pl-8 pr-1">
              <div className="absolute bottom-4 left-[7px] top-2 w-px bg-outline-variant/60" />

              <div className="relative z-10 space-y-10 pb-6">
                {sortedEvents.map((item) => (
                  <TimelineEvent item={item} key={item.id} />
                ))}
              </div>
            </div>
          </WorkspaceCard>

          <aside className="col-span-12 space-y-6 xl:col-span-4">
            <ProjectSummaryPanel deal={deal} />
            <TimelineTasksPanel tasks={deal.pendingTasks} />
          </aside>
        </div>
      </div>

      {isModalOpen ? (
        <NewActivityModal
          formState={formState}
          onChange={setFormState}
          onClose={closeModal}
          onSubmit={handleSubmit}
        />
      ) : null}

      <div
        aria-live="polite"
        className={`fixed bottom-8 right-8 z-50 flex items-center gap-3 rounded-[24px] border border-white/80 bg-white/76 px-5 py-4 text-sm font-semibold text-text-main shadow-[0_18px_50px_rgba(28,40,38,0.16)] backdrop-blur-md transition ${
          toastMessage ? "translate-y-0 opacity-100" : "pointer-events-none translate-y-8 opacity-0"
        }`}
      >
        <Icon className="h-5 w-5 text-primary" name="checkCircle" />
        {toastMessage}
      </div>
    </>
  );
}

type TimelineEventProps = {
  item: DealTimelineItem;
};

function TimelineEvent({ item }: TimelineEventProps) {
  const tone = timelineToneClasses[item.tone];
  const isSiteVisit = item.category.toLowerCase().includes("site");

  return (
    <article className="relative">
      <span className={`absolute -left-[31px] top-1.5 h-4 w-4 rounded-full border-4 border-surface ${tone.dot}`} />

      <div className="grid gap-3 sm:grid-cols-[1fr_auto]">
        <div className="min-w-0">
          <p className={`mb-2 text-[10px] font-bold uppercase tracking-[0.18em] ${tone.label}`}>{item.category}</p>
          <h3 className="text-[1.22rem] font-bold leading-tight text-text-main [font-family:var(--font-heading)]">{item.title}</h3>
        </div>

        <time className="text-[13px] font-semibold text-text-main/78" dateTime={item.date}>
          {formatTimelineDate(item.date)}
        </time>
      </div>

      <div className={`mt-4 rounded-[20px] border px-5 py-4 text-[15px] leading-7 ${tone.detail}`}>
        <p>{item.detail}</p>

        {isSiteVisit ? (
          <div className="mt-4 flex flex-wrap gap-3">
            <TimelineThumbnail tone="primary" />
            <TimelineThumbnail tone="accent" />
          </div>
        ) : null}
      </div>
    </article>
  );
}

type TimelineThumbnailProps = {
  tone: "accent" | "primary";
};

function TimelineThumbnail({ tone }: TimelineThumbnailProps) {
  const className =
    tone === "primary"
      ? "from-primary/85 via-secondary-fixed-dim to-surface-container-highest"
      : "from-tertiary/85 via-tertiary-fixed-dim to-surface-container-highest";

  return (
    <div className={`h-16 w-24 overflow-hidden rounded-xl bg-gradient-to-br ${className} shadow-[0_10px_20px_rgba(28,40,38,0.12)]`}>
      <div className="grid h-full grid-cols-5 gap-1 p-2 opacity-70">
        {Array.from({ length: 15 }, (_, index) => (
          <span className="rounded-sm bg-white/42" key={index} />
        ))}
      </div>
    </div>
  );
}

type ProjectSummaryPanelProps = {
  deal: DealRoomData;
};

function ProjectSummaryPanel({ deal }: ProjectSummaryPanelProps) {
  return (
    <WorkspaceCard className="rounded-[28px] p-7">
      <h2 className="type-h2 text-text-main">{deal.name} Due Diligence</h2>
      <div className="mt-4 flex flex-wrap gap-2">
        <StatusBadge className="bg-secondary-container/45 text-on-secondary-container" label={deal.stageLabel} />
        <StatusBadge className="bg-primary/10 text-primary" label={deal.phaseLabel} />
      </div>
      <p className="mt-5 text-[15px] leading-7 text-text-main/82">{deal.summary}</p>
    </WorkspaceCard>
  );
}

type TimelineTasksPanelProps = {
  tasks: DealTask[];
};

function TimelineTasksPanel({ tasks }: TimelineTasksPanelProps) {
  const [checkedState, setCheckedState] = useState(() => tasks.map((task) => Boolean(task.done)));

  return (
    <WorkspaceCard className="rounded-[28px] p-7">
      <div className="mb-7 flex items-center justify-between">
        <h2 className="type-h2 text-text-main">Pending Tasks</h2>
        <button
          aria-label="Task actions"
          className="inline-flex h-9 w-9 items-center justify-center rounded-full text-muted transition hover:bg-white/58 hover:text-text-main"
          type="button"
        >
          <Icon className="h-5 w-5" name="more" />
        </button>
      </div>

      <div className="space-y-7">
        {tasks.map((task, index) => {
          const isChecked = checkedState[index] ?? false;
          const statusLabel = isChecked ? "Completed" : task.priority ? "High Priority" : "Open Task";

          return (
            <article className="flex gap-4" key={task.id}>
              <button
                aria-label={`${isChecked ? "Mark incomplete" : "Mark complete"}: ${task.label}`}
                aria-pressed={isChecked}
                className="mt-1 flex h-5 w-5 shrink-0 items-center justify-center rounded-full border-2 border-primary/20 bg-white/70 transition hover:scale-105"
                onClick={() => setCheckedState((current) => current.map((value, valueIndex) => (valueIndex === index ? !value : value)))}
                type="button"
              >
                <span className={`h-2.5 w-2.5 rounded-full ${task.priority ? "bg-error" : "bg-primary"} ${isChecked ? "opacity-100" : ""}`} />
              </button>

              <div className="min-w-0 flex-1">
                <p className={`mb-2 text-[10px] font-bold uppercase tracking-[0.16em] ${task.priority ? "text-error/65" : "text-muted/72"}`}>
                  {statusLabel}
                </p>
                <h3 className={`text-[15px] font-bold text-text-main ${isChecked ? "line-through opacity-55" : ""}`}>{task.label}</h3>
                <div
                  className={`mt-4 rounded-[18px] border px-4 py-3 text-[13px] leading-6 ${
                    task.priority ? "border-error/20 bg-error-container/18 text-on-error-container italic" : "border-white/55 bg-white/48 text-text-main/78"
                  }`}
                >
                  {isChecked
                    ? "Task completed."
                    : task.priority
                      ? "Needs immediate follow-up and should be addressed before the next review cycle."
                      : "Click the circle to mark this diligence task complete."}
                </div>
              </div>
            </article>
          );
        })}
      </div>

      <button className="mt-8 w-full rounded-2xl border border-outline-variant bg-white/55 px-4 py-3 text-[15px] font-semibold text-text-main transition hover:bg-white/78">
        View Full List
      </button>
    </WorkspaceCard>
  );
}

type NewActivityModalProps = {
  formState: TimelineFormState;
  onChange: (formState: TimelineFormState) => void;
  onClose: () => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};

function NewActivityModal({ formState, onChange, onClose, onSubmit }: NewActivityModalProps) {
  const canSubmit = Boolean(formState.title.trim() && formState.date);

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/10 p-4 backdrop-blur-sm" role="presentation">
      <form
        className="shrink-0 rounded-[28px] border border-white/80 bg-white/78 p-8 shadow-[0_28px_80px_rgba(28,40,38,0.18)] backdrop-blur-xl"
        onSubmit={onSubmit}
        style={{ width: "min(calc(100vw - 32px), 28rem)" }}
      >
        <div className="mb-7 flex items-center justify-between">
          <h2 className="type-h1 text-text-main">New Activity</h2>
          <button
            aria-label="Close new activity"
            className="inline-flex h-10 w-10 items-center justify-center rounded-full text-text-main transition hover:bg-surface-container"
            onClick={onClose}
            type="button"
          >
            <span className="relative h-5 w-5 before:absolute before:left-1/2 before:top-0 before:h-full before:w-0.5 before:-translate-x-1/2 before:rotate-45 before:rounded-full before:bg-current after:absolute after:left-1/2 after:top-0 after:h-full after:w-0.5 after:-translate-x-1/2 after:-rotate-45 after:rounded-full after:bg-current" />
          </button>
        </div>

        <div className="space-y-5">
          <label className="block min-w-0">
            <span className="mb-2 block text-[12px] font-bold uppercase tracking-[0.12em] text-text-main/75">Activity Title</span>
            <input
              className="h-11 w-full rounded-xl border border-outline-variant bg-white/55 px-4 text-[15px] text-text-main outline-none transition placeholder:text-muted focus:border-primary focus:ring-2 focus:ring-primary/16"
              onChange={(event) => onChange({ ...formState, title: event.target.value })}
              placeholder="e.g. Stakeholder Interview"
              required
              type="text"
              value={formState.title}
            />
          </label>

          <div className="grid min-w-0 gap-4 sm:grid-cols-2">
            <label className="block min-w-0">
              <span className="mb-2 block text-[12px] font-bold uppercase tracking-[0.12em] text-text-main/75">Category</span>
              <select
                className="h-11 w-full rounded-xl border border-outline-variant bg-white/55 px-4 text-[15px] text-text-main outline-none transition focus:border-primary focus:ring-2 focus:ring-primary/16"
                onChange={(event) => onChange({ ...formState, category: event.target.value })}
                value={formState.category}
              >
                {categoryOptions.map((category) => (
                  <option key={category}>{category}</option>
                ))}
              </select>
            </label>

            <label className="block min-w-0">
              <span className="mb-2 block text-[12px] font-bold uppercase tracking-[0.12em] text-text-main/75">Date</span>
              <input
                className="h-11 w-full rounded-xl border border-outline-variant bg-white/55 px-4 text-[15px] text-text-main outline-none transition focus:border-primary focus:ring-2 focus:ring-primary/16"
                onChange={(event) => onChange({ ...formState, date: event.target.value })}
                required
                type="date"
                value={formState.date}
              />
            </label>
          </div>

          <label className="block min-w-0">
            <span className="mb-2 block text-[12px] font-bold uppercase tracking-[0.12em] text-text-main/75">Notes</span>
            <textarea
              className="min-h-24 w-full resize-y rounded-xl border border-outline-variant bg-white/55 px-4 py-3 text-[15px] leading-6 text-text-main outline-none transition placeholder:text-muted focus:border-primary focus:ring-2 focus:ring-primary/16"
              onChange={(event) => onChange({ ...formState, detail: event.target.value })}
              placeholder="Describe the activity details..."
              rows={3}
              value={formState.detail}
            />
          </label>

          <button
            className="h-12 w-full rounded-2xl bg-primary px-5 text-[15px] font-bold text-white shadow-[0_12px_28px_rgba(50,99,65,0.26)] transition hover:bg-primary-container disabled:cursor-not-allowed disabled:opacity-55"
            disabled={!canSubmit}
            type="submit"
          >
            Log to Timeline
          </button>
        </div>
      </form>
    </div>
  );
}

type StatusBadgeProps = {
  className: string;
  label: string;
};

function StatusBadge({ className, label }: StatusBadgeProps) {
  return <span className={`rounded-full px-3 py-1 text-[10px] font-bold uppercase tracking-[0.14em] ${className}`}>{label}</span>;
}

function formatTimelineDate(date: string) {
  const parsedDate = new Date(`${date}T12:00:00`);

  return new Intl.DateTimeFormat("en-US", { day: "numeric", month: "short" }).format(parsedDate);
}
