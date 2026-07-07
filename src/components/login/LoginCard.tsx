import { FormEvent } from "react";
import { BrandLockup } from "../brand/BrandLockup";
import { Button } from "../ui/Button";
import { FormField } from "../ui/FormField";
import { Icon } from "../ui/Icon";

const WEST_MONROE_EMAIL_DOMAIN = "@westmonroe.com";

type LoginCardProps = {
  apiKey: string;
  email: string;
  onApiKeyChange: (value: string) => void;
  onEmailChange: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};

export function LoginCard({
  apiKey,
  email,
  onApiKeyChange,
  onEmailChange,
  onSubmit,
}: LoginCardProps) {
  const emailLocalPart = getEmailLocalPart(email);

  function handleEmailLocalPartChange(value: string) {
    const localPart = value.replace(/\s/g, "").replace(/@.*$/, "");
    onEmailChange(localPart ? `${localPart}${WEST_MONROE_EMAIL_DOMAIN}` : "");
  }

  return (
    <section className="glass-panel mx-auto w-full max-w-[440px] rounded-xl p-xl">
      <BrandLockup
        subtitle="Precision insights for executive decision-makers."
        title="Strategic Portfolio Hub"
      />

      <form className="mt-xl space-y-lg" onSubmit={onSubmit}>
        <div className="space-y-xs">
          <div className="flex items-center justify-between gap-sm px-xs">
            <label className="type-label text-on-surface-variant" htmlFor="email-local-part">
              WM Email
            </label>
          </div>

          <div className="flex overflow-hidden rounded-full border border-outline-variant bg-surface-container-lowest shadow-[inset_0_1px_0_rgba(255,255,255,0.5)] transition focus-within:border-primary-container focus-within:ring-4 focus-within:ring-primary-fixed/40">
            <input
              autoComplete="username"
              className="min-w-0 flex-1 bg-transparent py-md pl-md pr-sm text-[15px] leading-[1.6] text-on-surface outline-none placeholder:text-outline-variant"
              id="email-local-part"
              onChange={(event) => handleEmailLocalPartChange(event.currentTarget.value)}
              placeholder="rgambhir"
              required
              type="text"
              value={emailLocalPart}
            />
            <div className="flex shrink-0 items-center border-l border-outline-variant bg-surface-container-low px-md text-[13px] font-semibold tracking-[0.02em] text-muted">
              {WEST_MONROE_EMAIL_DOMAIN}
            </div>
          </div>
        </div>

        <FormField
          action={
            <a
              className="inline-flex items-center gap-xs text-[12px] font-semibold tracking-[0.05em] text-primary-container transition-colors hover:text-primary"
              href="https://platform.openai.com/api-keys"
              rel="noreferrer"
              target="_blank"
            >
              <Icon className="h-3.5 w-3.5" name="help" />
              How do I find my API key?
            </a>
          }
          autoComplete="off"
          icon={<Icon className="h-5 w-5" name="key" />}
          id="api-key"
          label="OpenAI API Key"
          onChange={onApiKeyChange}
          placeholder="sk-••••••••••••••••••••••••"
          type="password"
          value={apiKey}
        />

        <Button icon={<Icon className="h-[18px] w-[18px]" name="arrowRight" />} type="submit">
          Access Hub
        </Button>
      </form>

      <div className="mt-xl flex items-start gap-md border-t border-outline-variant/80 pt-lg">
        <div className="mt-0.5 text-outline">
          <Icon className="h-5 w-5" name="shield" />
        </div>
        <p className="type-body-sm leading-relaxed text-secondary">
          <span className="font-semibold text-on-surface">Privacy &amp; Security:</span> Your API key is encrypted and
          never stored on our servers. It is only used to facilitate direct requests to your private OpenAI instance.
        </p>
      </div>
    </section>
  );
}

function getEmailLocalPart(email: string) {
  return email.split("@")[0] ?? "";
}
