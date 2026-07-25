import { Icon } from "../ui/Icon";

type EdgePanelOpenButtonProps = {
  label: string;
  onClick: () => void;
  side: "left" | "right";
};

export function EdgePanelOpenButton({ label, onClick, side }: EdgePanelOpenButtonProps) {
  const sideClasses = side === "left" ? "left-0 border-l-0" : "right-0 border-r-0";
  const iconName = side === "left" ? "chevronRight" : "chevronLeft";

  return (
    <button
      aria-label={label}
      className={`absolute top-1/2 z-30 flex h-14 w-11 -translate-y-1/2 items-center justify-center border border-outline-variant bg-surface-container-lowest/90 text-primary shadow-sm transition hover:bg-surface-container-lowest hover:text-text-main ${sideClasses}`}
      onClick={onClick}
      title={label}
      type="button"
    >
      <Icon className="h-6 w-6" name={iconName} />
    </button>
  );
}
