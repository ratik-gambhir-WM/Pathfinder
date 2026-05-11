type DealRoomHeaderProps = {
  subtitle: string;
};

export function DealRoomHeader({ subtitle }: DealRoomHeaderProps) {
  return (
    <header className="flex flex-col gap-5">
      <div className="space-y-2">
        <h1 className="type-display text-text-main">Deal Room</h1>
        <p className="type-subtle text-muted">{subtitle}</p>
      </div>
    </header>
  );
}
