import { ReactNode } from "react";
import { workspaceDeals, workspaceInitiatives, workspaceTools } from "../../data/workspace";
import { useWorkspaceSession } from "../../hooks/useWorkspaceSession";
import { WorkspaceLayout } from "./WorkspaceLayout";
import { WorkspaceSidebar } from "./WorkspaceSidebar";

type WorkspaceHomeShellProps = {
  activeHomeSection?: "hub" | "summarize" | "vault";
  children: ReactNode;
};

export function WorkspaceHomeShell({ activeHomeSection = "hub", children }: WorkspaceHomeShellProps) {
  const { email, navigationState } = useWorkspaceSession();

  return (
    <WorkspaceLayout
      sidebar={
        <WorkspaceSidebar
          activeHomeSection={activeHomeSection}
          deals={workspaceDeals}
          email={email}
          initiatives={workspaceInitiatives}
          navigationState={navigationState}
          tools={workspaceTools}
        />
      }
    >
      {children}
    </WorkspaceLayout>
  );
}
