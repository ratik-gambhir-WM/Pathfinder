import { FormEvent, memo, useCallback, useMemo, useState } from "react";
import westMonroeLogo from "./assets/Screenshot 2026-05-07 at 8.24.22 PM.png";

type AuthStep = "email" | "apiKey";

type StepConfig = {
  autoComplete: string;
  fieldId: string;
  inputMode?: "email";
  label: string;
  placeholder: string;
  type: "email" | "password";
};

const STEP_CONTENT: Record<AuthStep, StepConfig> = {
  email: {
    autoComplete: "email",
    fieldId: "email",
    inputMode: "email",
    label: "Email",
    placeholder: "Email",
    type: "email",
  },
  apiKey: {
    autoComplete: "off",
    fieldId: "api-key",
    label: "API key",
    placeholder: "Paste your API key",
    type: "password",
  },
};

function App() {
  const [step, setStep] = useState<AuthStep>("email");
  const [email, setEmail] = useState("");
  const [apiKey, setApiKey] = useState("");

  const config = STEP_CONTENT[step];
  const value = step === "email" ? email : apiKey;
  const setValue = step === "email" ? setEmail : setApiKey;
  const canContinue = useMemo(() => value.trim().length > 0, [value]);

  const handleSubmit = useCallback(
    (event: FormEvent<HTMLFormElement>) => {
      event.preventDefault();

      if (!canContinue) {
        return;
      }

      if (step === "email") {
        setStep("apiKey");
      }
    },
    [canContinue, step],
  );

  return (
    <main className="min-h-screen bg-wm-canvas px-5 py-12 text-wm-ink sm:px-8 sm:py-16">
      <div className="mx-auto flex min-h-[calc(100vh-6rem)] w-full max-w-[760px] flex-col items-center justify-center gap-12">
        <BrandHeader />
        <AuthCard
          canContinue={canContinue}
          config={config}
          onSubmit={handleSubmit}
          onValueChange={setValue}
          value={value}
        />
      </div>
    </main>
  );
}

const BrandHeader = memo(function BrandHeader() {
  return (
    <div className="flex w-full justify-center">
      <img
        alt="West Monroe"
        className="h-auto w-full max-w-[560px]"
        decoding="async"
        src={westMonroeLogo}
      />
    </div>
  );
});

type AuthCardProps = {
  canContinue: boolean;
  config: StepConfig;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onValueChange: (value: string) => void;
  value: string;
};

const AuthCard = memo(function AuthCard({
  canContinue,
  config,
  onSubmit,
  onValueChange,
  value,
}: AuthCardProps) {
  return (
    <section
      aria-label={`${config.label} entry`}
      className="w-full rounded-lg border border-wm-border bg-wm-panel px-7 py-12 shadow-[0_2px_4px_rgba(24,24,27,0.03)] sm:px-12 sm:py-16"
    >
      <form className="mx-auto w-full max-w-[640px]" onSubmit={onSubmit}>
        <FieldWithSubmit
          canContinue={canContinue}
          config={config}
          onValueChange={onValueChange}
          value={value}
        />
      </form>
    </section>
  );
});

type FieldWithSubmitProps = {
  canContinue: boolean;
  config: StepConfig;
  onValueChange: (value: string) => void;
  value: string;
};

const FieldWithSubmit = memo(function FieldWithSubmit({
  canContinue,
  config,
  onValueChange,
  value,
}: FieldWithSubmitProps) {
  return (
    <div className="grid gap-3">
      <label className="text-[1.08rem] font-bold leading-none text-wm-ink" htmlFor={config.fieldId}>
        {config.label}
      </label>

      <div className="grid min-h-16 grid-cols-[minmax(0,1fr)_4rem] overflow-hidden rounded-lg border border-wm-border bg-white shadow-[0_2px_8px_rgba(24,24,27,0.08)] transition focus-within:border-wm-navy focus-within:ring-4 focus-within:ring-wm-navy/10">
        <input
          autoComplete={config.autoComplete}
          autoFocus
          className="min-w-0 border-0 bg-transparent px-5 text-[1.15rem] text-wm-ink outline-none placeholder:text-wm-muted"
          id={config.fieldId}
          inputMode={config.inputMode}
          key={config.fieldId}
          onChange={(event) => onValueChange(event.currentTarget.value)}
          placeholder={config.placeholder}
          required
          type={config.type}
          value={value}
        />
        <ArrowSubmitButton disabled={!canContinue} label={`Continue with ${config.label}`} />
      </div>
    </div>
  );
});

type ArrowSubmitButtonProps = {
  disabled: boolean;
  label: string;
};

const ArrowSubmitButton = memo(function ArrowSubmitButton({
  disabled,
  label,
}: ArrowSubmitButtonProps) {
  return (
    <button
      aria-label={label}
      className="grid h-full min-w-16 place-items-center border-0 border-l border-wm-border bg-zinc-100 text-wm-navy transition enabled:cursor-pointer enabled:hover:bg-wm-navy enabled:hover:text-white enabled:focus-visible:bg-wm-navy enabled:focus-visible:text-white enabled:focus-visible:outline-none disabled:cursor-not-allowed disabled:text-wm-muted/45"
      disabled={disabled}
      type="submit"
    >
      <svg aria-hidden="true" className="h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path
          d="M5 12h13m-5-6 6 6-6 6"
          stroke="currentColor"
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth="2.25"
        />
      </svg>
    </button>
  );
});

export default App;
