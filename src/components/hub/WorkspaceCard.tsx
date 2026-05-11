import { ReactNode } from "react";

type WorkspaceCardProps = {
  children: ReactNode;
  className?: string;
};

export function WorkspaceCard({ children, className = "" }: WorkspaceCardProps) {
  return <section className={`workspace-card ${className}`}>{children}</section>;
}
