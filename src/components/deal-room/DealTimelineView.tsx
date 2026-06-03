import { FormEvent, useMemo, useState } from "react";
import { DealRoomData, DealTask, DealTimelineItem, DealTimelineTone } from "../../data/workspace";
import { WorkspaceCard } from "../hub/WorkspaceCard";
import { Icon } from "../ui/Icon";

type DealTimelineViewProps = {
  deal: DealRoomData;
  events: DealTimelineItem[];
  onEventsChange: (events: DealTimelineItem[]) => void;
};

type TimelineCategory = "Deliverable" | "Key Activity" | "Key Meeting / Call";

type TimelineFormState = {
  category: TimelineCategory;
  date: string;
  detail: string;
  title: string;
};

type CalendarDay = {
  date: Date;
  dateKey: string;
  dayLabel: string;
};

const categoryOptions: TimelineCategory[] = ["Key Meeting / Call", "Key Activity", "Deliverable"];
const weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

const initialFormState: TimelineFormState = {
  category: "Key Meeting / Call",
  date: "",
  detail: "",
  title: "",
};

const toneByCategory: Record<TimelineCategory, DealTimelineTone> = {
  Deliverable: "accent",
  "Key Activity": "muted",
  "Key Meeting / Call": "primary",
};

const categoryStyles: Record<TimelineCategory, { bar: string; legend: string }> = {
  Deliverable: {
    bar: "bg-[#0055ff] text-white",
    legend: "bg-[#0055ff]",
  },
  "Key Activity": {
    bar: "bg-secondary-container text-on-secondary-container",
    legend: "bg-secondary-container",
  },
  "Key Meeting / Call": {
    bar: "bg-primary text-white",
    legend: "bg-primary",
  },
};

const calendarWeeks = createCalendarWeeks("2026-09-28", 5);

export function DealTimelineView({ deal, events, onEventsChange }: DealTimelineViewProps) {
  const [formState, setFormState] = useState<TimelineFormState>(initialFormState);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [toastMessage, setToastMessage] = useState("");

  const eventsByDate = useMemo(() => {
    return events.reduce<Record<string, DealTimelineItem[]>>((groupedEvents, item) => {
      const dateKey = getCalendarDateKey(item.date);
      groupedEvents[dateKey] = [...(groupedEvents[dateKey] ?? []), item];
      return groupedEvents;
    }, {});
  }, [events]);

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
      tone: toneByCategory[formState.category],
    };

    onEventsChange([...events, timelineItem]);
    setIsModalOpen(false);
    setToastMessage("Timeline updated successfully");
    window.setTimeout(() => setToastMessage(""), 3200);
  }

  return (
    <>
      <div className="flex flex-col gap-10">
        <header className="flex flex-col gap-5 xl:flex-row xl:items-start xl:justify-between">
          <div className="space-y-1">
            <h1 className="type-display text-text-main">Meeting Timeline</h1>
            <p className="text-[13px] text-text-main/78">{deal.overviewSubtitle}</p>
          </div>

          <div className="flex flex-wrap items-center gap-6">
            <TimelineLegend />
            <button
              className="inline-flex h-10 items-center justify-center gap-2 rounded-full bg-primary px-5 text-[12px] font-bold text-white shadow-[0_10px_26px_rgba(50,99,65,0.24)] transition hover:bg-primary-container focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
              onClick={openModal}
              type="button"
            >
              <Icon className="h-4 w-4" name="plus" />
              Log Activity
            </button>
          </div>
        </header>

        <div className="grid grid-cols-12 gap-8">
          <WorkspaceCard className="col-span-12 rounded-[28px] p-6 xl:col-span-8">
            <div className="mb-7 flex items-center justify-between">
              <h2 className="type-h1 text-text-main">Engagement Timeline</h2>
              <button
                aria-label="Timeline actions"
                className="inline-flex h-10 w-10 items-center justify-center rounded-full text-muted transition hover:bg-white/58 hover:text-text-main"
                type="button"
              >
                <Icon className="h-5 w-5" name="more" />
              </button>
            </div>

            <div className="overflow-x-auto">
              <div
                className="grid min-w-[740px] border-l border-t border-outline-variant bg-white/20"
                style={{ gridTemplateColumns: "40px repeat(5, minmax(0, 1fr))" }}
              >
                <div className="border-b border-r border-outline-variant bg-white/35" />
                {weekdays.map((weekday) => (
                  <div
                    className="border-b border-r border-outline-variant bg-[#00004d] px-3 py-3 text-center text-[12px] font-bold uppercase tracking-[0.05em] text-white"
                    key={weekday}
                  >
                    {weekday}
                  </div>
                ))}

                {calendarWeeks.map((week, weekIndex) => (
                  <CalendarWeekRow eventsByDate={eventsByDate} key={weekIndex} week={week} weekIndex={weekIndex} />
                ))}
              </div>
            </div>

            <div className="mt-8 flex flex-col gap-4 rounded-xl border border-primary/10 bg-primary/5 p-4 sm:flex-row sm:items-center sm:justify-between">
              <div className="flex items-start gap-3">
                <Icon className="mt-0.5 h-5 w-5 shrink-0 text-primary" name="alert" />
                <p className="text-[12px] leading-5 text-text-main/78">
                  Viewing 5-week overview for <span className="font-bold text-text-main">Phase 1: Discovery</span>. Events are placed by
                  date.
                </p>
              </div>
              <button className="text-left text-[12px] font-bold text-primary transition hover:text-primary-container" type="button">
                Download PDF
              </button>
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

function TimelineLegend() {
  return (
    <div className="flex flex-wrap items-center gap-4 text-[10px] font-bold uppercase tracking-[0.08em] text-text-main">
      {categoryOptions.map((category) => (
        <div className="flex items-center gap-1.5" key={category}>
          <span className={`h-3 w-3 rounded-sm ${categoryStyles[category].legend}`} />
          {category === "Key Meeting / Call" ? "Key Meeting" : category}
        </div>
      ))}
    </div>
  );
}

type CalendarWeekRowProps = {
  eventsByDate: Record<string, DealTimelineItem[]>;
  week: CalendarDay[];
  weekIndex: number;
};

function CalendarWeekRow({ eventsByDate, week, weekIndex }: CalendarWeekRowProps) {
  return (
    <>
      <div className="flex min-h-[100px] items-center justify-center border-b border-r border-outline-variant px-1 text-center text-[11px] italic leading-4 text-text-main/78">
        Week
        <br />
        {weekIndex}
      </div>
      {week.map((day) => (
        <CalendarCell day={day} events={eventsByDate[day.dateKey] ?? []} key={day.dateKey} />
      ))}
    </>
  );
}

type CalendarCellProps = {
  day: CalendarDay;
  events: DealTimelineItem[];
};

function CalendarCell({ day, events }: CalendarCellProps) {
  return (
    <div className="min-h-[100px] border-b border-r border-outline-variant p-2 text-[10px] font-medium text-text-main/78">
      <div>{day.dayLabel}</div>
      <div className="mt-2 space-y-1">
        {events
          .slice()
          .sort((first, second) => first.title.localeCompare(second.title))
          .map((event) => (
            <CalendarEventBar event={event} key={event.id} />
          ))}
      </div>
    </div>
  );
}

type CalendarEventBarProps = {
  event: DealTimelineItem;
};

function CalendarEventBar({ event }: CalendarEventBarProps) {
  const category = normalizeCategory(event.category);
  const styles = categoryStyles[category];

  return (
    <div
      className={`h-6 max-w-full truncate px-3 py-1 text-[10px] font-bold leading-4 [clip-path:polygon(5%_0%,95%_0%,100%_50%,95%_100%,5%_100%,0%_50%)] ${styles.bar}`}
      title={`${event.title} - ${formatTimelineDate(event.date)}`}
    >
      {event.title}
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
  const featuredTasks = tasks.filter((task, index) => index === 0 || task.priority);
  const [checkedState, setCheckedState] = useState(() => featuredTasks.map((task) => Boolean(task.done)));

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

      <div className="space-y-8">
        {featuredTasks.map((task, index) => {
          const isChecked = checkedState[index] ?? false;
          const statusLabel = isChecked ? "Completed" : task.priority ? "High Priority" : "Open Task";

          return (
            <article className="flex gap-4" key={task.id}>
              <button
                aria-label={`${isChecked ? "Mark incomplete" : "Mark complete"}: ${task.label}`}
                aria-pressed={isChecked}
                className={`mt-1 flex h-5 w-5 shrink-0 items-center justify-center rounded-full border-2 bg-white/70 transition hover:scale-105 ${
                  task.priority ? "border-error/20" : "border-primary/20"
                }`}
                onClick={() => setCheckedState((current) => current.map((value, valueIndex) => (valueIndex === index ? !value : value)))}
                type="button"
              >
                <span className={`h-2.5 w-2.5 rounded-full ${task.priority ? "bg-error" : "bg-primary"}`} />
              </button>

              <div className="min-w-0 flex-1">
                <p className={`mb-2 text-[10px] font-bold uppercase tracking-[0.16em] ${task.priority ? "text-error/65" : "text-muted/72"}`}>
                  {statusLabel}
                </p>
                <h3 className={`text-[15px] font-bold text-text-main ${isChecked ? "line-through opacity-55" : ""}`}>{task.label}</h3>
                <div
                  className={`mt-4 rounded-[14px] border px-4 py-3 text-[13px] leading-6 ${
                    task.priority ? "border-error/20 bg-error-container/18 text-on-error-container italic" : "border-white/55 bg-white/48 text-text-main/78"
                  }`}
                >
                  {isChecked
                    ? "Task completed."
                    : task.priority
                      ? "Needs immediate follow-up."
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
                onChange={(event) => onChange({ ...formState, category: event.target.value as TimelineCategory })}
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

function createCalendarWeeks(startDate: string, weekCount: number) {
  return Array.from({ length: weekCount }, (_, weekIndex) =>
    Array.from({ length: 5 }, (_, dayIndex) => {
      const date = addDays(parseLocalDate(startDate), weekIndex * 7 + dayIndex);
      return {
        date,
        dateKey: toDateKey(date),
        dayLabel: formatCalendarDayLabel(date),
      };
    }),
  );
}

function addDays(date: Date, days: number) {
  const nextDate = new Date(date);
  nextDate.setDate(nextDate.getDate() + days);
  return nextDate;
}

function parseLocalDate(date: string) {
  return new Date(`${date}T12:00:00`);
}

function toDateKey(date: Date) {
  return date.toISOString().slice(0, 10);
}

function getCalendarDateKey(date: string) {
  const parsedDate = parseLocalDate(date);
  const day = parsedDate.getDay();

  if (day === 6) {
    return toDateKey(addDays(parsedDate, -1));
  }

  if (day === 0) {
    return toDateKey(addDays(parsedDate, -2));
  }

  return toDateKey(parsedDate);
}

function normalizeCategory(category: string): TimelineCategory {
  if (category === "Deliverable") {
    return "Deliverable";
  }

  if (category === "Key Activity" || category === "Site Visit") {
    return "Key Activity";
  }

  return "Key Meeting / Call";
}

function formatCalendarDayLabel(date: Date) {
  const day = date.getDate();

  if (day === 1) {
    return `${date.toLocaleString("en-US", { month: "short" }).toUpperCase()} ${day}`;
  }

  return String(day);
}

function formatTimelineDate(date: string) {
  const parsedDate = parseLocalDate(date);

  return new Intl.DateTimeFormat("en-US", { day: "numeric", month: "short" }).format(parsedDate);
}
