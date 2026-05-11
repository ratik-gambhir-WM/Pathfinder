import { FormEvent } from "react";
import { BrandLockup } from "../brand/BrandLockup";
import { Button } from "../ui/Button";
import { FormField } from "../ui/FormField";
import { Icon } from "../ui/Icon";

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
  return (
    <section className="glass-panel mx-auto w-full max-w-[440px] rounded-xl p-xl">
      <BrandLockup
        subtitle="Precision insights for executive decision-makers."
        title="Strategic Portfolio Hub"
      />

      <form className="mt-xl space-y-lg" onSubmit={onSubmit}>
        <FormField
          autoComplete="email"
          icon={<Icon className="h-5 w-5" name="mail" />}
          id="email"
          label="Email Address"
          onChange={onEmailChange}
          placeholder="name@westmonroe.com"
          type="email"
          value={email}
        />

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
