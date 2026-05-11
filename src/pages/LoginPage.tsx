import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { LoginCard } from "../components/login/LoginCard";
import { AppShell } from "../components/layout/AppShell";

export function LoginPage() {
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [apiKey, setApiKey] = useState("");

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    navigate("/hub", {
      state: {
        email,
      },
    });
  }

  return (
    <AppShell centered>
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
