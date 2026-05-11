import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { LoginCard } from "../components/login/LoginCard";
import { AppShell } from "../components/layout/AppShell";
import { persistWorkspaceEmail } from "../hooks/useWorkspaceSession";

export function LoginPage() {
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [apiKey, setApiKey] = useState("");

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    persistWorkspaceEmail(email);

    navigate("/hub", {
      state: {
        email,
      },
    });
  }

  return (
    <AppShell centered showFooter={false}>
      <LoginCard
        apiKey={apiKey}
        email={email}
        onApiKeyChange={setApiKey}
        onEmailChange={setEmail}
        onSubmit={handleSubmit}
      />
    </AppShell>
  );
}
