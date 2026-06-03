import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { FormEvent, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { LoginCard } from "../components/login/LoginCard";
import { AppShell } from "../components/layout/AppShell";
import { Button } from "../components/ui/Button";
import { Icon } from "../components/ui/Icon";
import { persistWorkspaceEmail } from "../hooks/useWorkspaceSession";

type LoginDemoCommandResponse = {
  echoedEmail: string;
  message: string;
  source: string;
};

type LoginDemoEventResponse = {
  echoedEmail: string;
  message: string;
  originalNote: string;
  source: string;
};

export function LoginPage() {
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [commandResult, setCommandResult] = useState("No command response yet.");
  const [eventResult, setEventResult] = useState("No event response yet.");

  console.log("SETTTING UP LOG EHHEHEHEHE")

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    persistWorkspaceEmail(email);

    navigate("/hub", {
      state: {
        email,
      },
    });
  }

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    void listen<LoginDemoEventResponse>("login-demo:backend-response", (event) => {
      console.log("HIT EVENT IN FRONT END FROM TAURI BACKEND")
      setEventResult(JSON.stringify(event.payload, null, 2));
    }).then((cleanup) => {
      unlisten = cleanup;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  async function handleCommandDemo() {
    const payload = {
      email: email || "demo@pathfinder.local",
      source: "login-page-command-button",
    };

    const response = await invoke<LoginDemoCommandResponse>("login_demo_command", {
      payload,
    });
    console.log("GOT RESPONSE HEHEHEHE");
    console.log(response.message);

    setCommandResult(JSON.stringify({ request: payload, response }, null, 2));
  }

  async function handleEventDemo() {
    const payload = {
      email: email || "demo@pathfinder.local",
      note: "Frontend event payload from the login page",
      source: "login-page-event-button",
    };

    setEventResult(JSON.stringify({ request: payload, status: "waiting for Rust event reply" }, null, 2));
    await emit("login-demo:frontend-request", payload);
  }

  return (
    <AppShell centered showFooter={false}>
      <div className="mx-auto flex w-full max-w-[440px] flex-col gap-lg">
        <LoginCard
          apiKey={apiKey}
          email={email}
          onApiKeyChange={setApiKey}
          onEmailChange={setEmail}
          onSubmit={handleSubmit}
        />

        <section className="glass-panel rounded-xl p-xl">
          <div className="flex items-start gap-md">
            <div className="rounded-full bg-primary-fixed/50 p-sm text-primary">
              <Icon className="h-5 w-5" name="terminal" />
            </div>
            <div className="space-y-xs">
              <h2 className="text-sm font-semibold text-on-surface">Tauri Demo</h2>
              <p className="text-sm leading-relaxed text-secondary">
                One button calls a Rust command. One button sends a frontend event to Rust and listens for the reply event.
              </p>
            </div>
          </div>

          <div className="mt-lg grid gap-sm sm:grid-cols-2">
            <Button className="sm:w-full" onClick={() => void handleCommandDemo()}>
              Run Command Example
            </Button>
            <Button className="sm:w-full" onClick={() => void handleEventDemo()}>
              Run Event Example
            </Button>
          </div>

          <div className="mt-lg space-y-md">
            <div>
              <p className="text-[12px] font-semibold uppercase tracking-[0.08em] text-outline">
                Command flow
              </p>
              <pre className="mt-xs overflow-x-auto rounded-lg bg-surface-container-low px-md py-sm text-xs leading-relaxed text-on-surface">
                {commandResult}
              </pre>
            </div>

            <div>
              <p className="text-[12px] font-semibold uppercase tracking-[0.08em] text-outline">
                Event flow
              </p>
              <pre className="mt-xs overflow-x-auto rounded-lg bg-surface-container-low px-md py-sm text-xs leading-relaxed text-on-surface">
                {eventResult}
              </pre>
            </div>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
