import { DealRoomData } from "../../data/workspace";
import { WorkspaceCard } from "../hub/WorkspaceCard";
import { Icon } from "../ui/Icon";

type DealBriefingCardProps = {
  compact?: boolean;
  keyQuestions: DealRoomData["keyQuestions"];
  thesis: string;
};

export function DealBriefingCard({ compact = false, keyQuestions, thesis }: DealBriefingCardProps) {
  return (
    <WorkspaceCard
      className={`rounded-[28px] p-8 ${
        compact ? "col-span-12 flex min-h-[540px] flex-col xl:col-span-4" : "col-span-12"
      }`}
    >
      <div className={`gap-8 ${compact ? "flex flex-col" : "grid xl:grid-cols-2"}`}>
        <section className="space-y-4">
          <div className="flex items-center gap-3">
            <Icon className="h-6 w-6 text-primary" name="help" />
            <h2 className="type-h2 text-text-main">Key Questions</h2>
          </div>

          <div className="space-y-3">
            {keyQuestions.map((question) => (
              <div
                className="flex items-start gap-3 rounded-2xl border border-white/70 bg-white/42 px-4 py-4"
                key={question}
              >
                <span className="mt-1 h-2.5 w-2.5 rounded-full bg-primary" />
                <p className={`${compact ? "text-[0.96rem] leading-6" : "text-[1rem] leading-7"} text-text-main`}>
                  {question}
                </p>
              </div>
            ))}
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-3">
            <Icon className="h-6 w-6 text-accent" name="sparkles" />
            <h2 className="type-h2 text-text-main">Investment Thesis</h2>
          </div>

          <div className="rounded-[24px] border border-white/70 bg-white/45 p-5">
            <p className={`${compact ? "text-[0.96rem] leading-7" : "text-[1rem] leading-8"} text-text-main/86 italic`}>
              "{thesis}"
            </p>
          </div>
        </section>
      </div>
    </WorkspaceCard>
  );
}
