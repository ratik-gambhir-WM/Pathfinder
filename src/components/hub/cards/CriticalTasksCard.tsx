import { Icon } from "../../ui/Icon";
import { WorkspaceCard } from "../WorkspaceCard";

type TaskTag =
  | { tone: "error"; value: string }
  | { tone: "success"; value: string }
  | { tone: "muted"; value: string }
  | { tone: "icon"; value: "more" };

type TaskItem = {
  checked: boolean;
  label: string;
  tag: TaskTag;
};

type CriticalTasksCardProps = {
  assignees: string[];
  tasks: readonly TaskItem[];
};

export function CriticalTasksCard({ assignees, tasks }: CriticalTasksCardProps) {
  return (
    <WorkspaceCard className="relative col-span-12 flex min-h-[430px] flex-col justify-between overflow-hidden p-8 lg:col-span-8">
      <div className="absolute -right-16 -top-14 h-64 w-64 rounded-full bg-primary/6 blur-3xl" />

      <div className="relative z-10 flex h-full flex-col">
        <div className="mb-8 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="flex h-16 w-16 items-center justify-center rounded-[1.4rem] bg-primary/10 text-primary">
              <Icon className="h-8 w-8" name="listAlt" />
            </div>
            <div>
              <h2 className="type-h2 text-text-main">Critical Tasks</h2>
              <p className="text-[12px] font-medium uppercase tracking-[0.12em] text-muted">4 Pending Actions</p>
            </div>
          </div>

          <div className="flex -space-x-2">
            {assignees.map((assignee, index) => (
              <div
                className={`flex h-11 w-11 items-center justify-center rounded-full border-2 border-surface text-[13px] font-bold ${
                  index === 0 ? "bg-primary-fixed text-on-primary-fixed" : "bg-secondary-fixed text-on-secondary-fixed"
                }`}
                key={assignee}
              >
                {assignee}
              </div>
            ))}
          </div>
        </div>

        <div className="flex flex-1 flex-col gap-4">
          {tasks.map((task) => (
            <TaskRow item={task} key={task.label} />
          ))}
        </div>

        <div className="mt-8 border-t border-white/55 pt-6">
          <button className="inline-flex items-center gap-3 text-xs font-bold uppercase tracking-[0.18em] text-primary transition hover:gap-4">
            View Full Task Board
            <Icon className="h-4 w-4" name="arrowRight" />
          </button>
        </div>
      </div>
    </WorkspaceCard>
  );
}

function TaskRow({ item }: { item: TaskItem }) {
  return (
    <div className="flex items-center gap-4 rounded-[1.6rem] border border-white/60 bg-white/40 p-5 transition hover:bg-white/60">
      <button
        aria-label={item.checked ? "Completed task" : "Incomplete task"}
        className={`flex h-8 w-8 items-center justify-center rounded-xl border transition ${
          item.checked
            ? "border-primary bg-primary text-white"
            : "border-outline-variant bg-white/70 text-transparent"
        }`}
        type="button"
      >
        <Icon className="h-4 w-4" name="check" />
      </button>

      <div className="flex flex-1 items-center justify-between gap-4">
        <p className="text-[17px] font-medium text-text-main">{item.label}</p>
        <TaskTagView tag={item.tag} />
      </div>
    </div>
  );
}

function TaskTagView({ tag }: { tag: TaskTag }) {
  if (tag.tone === "icon") {
    return <Icon className="h-5 w-5 text-muted" name="more" />;
  }

  const className =
    tag.tone === "error"
      ? "bg-error/10 text-error"
      : tag.tone === "success"
        ? "bg-primary/10 text-primary"
        : "text-muted";

  return (
    <span className={`rounded-full px-3 py-1 text-[10px] font-bold uppercase tracking-[0.14em] ${className}`}>
      {tag.value}
    </span>
  );
}
