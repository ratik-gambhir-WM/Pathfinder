import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { FormEvent, ReactNode, useEffect, useRef, useState } from "react";
import { NavLink, useNavigate } from "react-router-dom";
import {
  getDataRoomPath,
  getDealRoomPath,
  getTeamLabel,
  WorkspaceAccountUser,
  WorkspaceDeal,
  WorkspaceLocationState,
  WorkspaceSidebarTool,
} from "../../data/workspace";
import { useThemeMode } from "../../hooks/useThemeMode";
import { WestMonroeMark } from "../brand/WestMonroeMark";
import { Icon } from "../ui/Icon";

type WorkspaceSidebarProps = {
  activeDealId?: string;
  activeHomeSection?: "account" | "hub" | "summarize" | "vault";
  activeSection?: "data-room" | "deal-room" | "diligence-graph" | "site-visits" | "timeline";
  deals: WorkspaceDeal[];
  email?: string;
  initiatives?: WorkspaceSidebarTool[];
  mode?: "deal-room" | "home";
  navigationState?: WorkspaceLocationState;
  onDealRoomSectionChange?: (section: "deal-room" | "diligence-graph" | "site-visits" | "timeline") => void;
  tools?: WorkspaceSidebarTool[];
};

const dealRoomSidebarLinks = [
  { icon: "dashboard" as const, key: "deal-room" as const, label: "Deal Room" },
  { icon: "timeline" as const, key: "timeline" as const, label: "Deal Activity" },
  { icon: "person" as const, key: "site-visits" as const, label: "Site Visits" },
  { icon: "graph" as const, key: "diligence-graph" as const, label: "Diligence Graph" },
  { icon: "folderOpen" as const, key: "data-room" as const, label: "Data Room Vault" },
  { icon: "grid" as const, label: "Synthesis Canvas" },
];

const dealTypeOptions = ["Buy-side", "Sell-side", "Carve-out", "Add-on", "Recapitalization", "Growth equity"];

type AddDealFormState = {
  buyerOrPlatformCompany: string;
  carveOutBusiness: string;
  dataRoomFolder: string;
  dealName: string;
  dealType: string;
  parentOrSellerCompany: string;
  peFirm: string;
  targetCompany: string;
};

const emptyAddDealForm: AddDealFormState = {
  buyerOrPlatformCompany: "",
  carveOutBusiness: "",
  dataRoomFolder: "",
  dealName: "",
  dealType: "",
  parentOrSellerCompany: "",
  peFirm: "",
  targetCompany: "",
};

export function WorkspaceSidebar({
  activeDealId,
  activeHomeSection = "hub",
  activeSection = "deal-room",
  deals,
  email,
  initiatives = [],
  mode = "home",
  navigationState,
  onDealRoomSectionChange,
  tools = [],
}: WorkspaceSidebarProps) {
  const [profileMenuOpen, setProfileMenuOpen] = useState(false);
  const [dealMenuOpen, setDealMenuOpen] = useState(false);
  const [addDealModalOpen, setAddDealModalOpen] = useState(false);
  const dealMenuRef = useRef<HTMLDivElement>(null);
  const teamLabel = getTeamLabel(email);
  const activeDeal = deals.find((deal) => deal.room.id === activeDealId) ?? deals[0];

  function handleAddDeal() {
    setDealMenuOpen(false);
    setAddDealModalOpen(true);
  }

  useEffect(() => {
    function handlePointerDown(event: PointerEvent) {
      if (!dealMenuRef.current?.contains(event.target as Node)) {
        setDealMenuOpen(false);
      }
    }

    if (dealMenuOpen) {
      document.addEventListener("pointerdown", handlePointerDown);
    }

    return () => document.removeEventListener("pointerdown", handlePointerDown);
  }, [dealMenuOpen]);

  return (
    <aside className="workspace-sidebar hidden h-full w-72 shrink-0 border-r border-white/80 bg-white/40 backdrop-blur-md lg:flex">
      <div className="flex h-full min-h-0 w-full flex-col p-4">
        <div className="space-y-6">
          <NavLink
            aria-label="Back to home page"
            className="flex items-center gap-4 rounded-[22px] px-2 py-2 transition hover:bg-white/40"
            state={navigationState}
            to="/hub"
          >
            <WestMonroeMark framed />
            <div>
              <h1 className="text-[1.1rem] font-bold leading-tight text-text-main [font-family:var(--font-heading)]">
                West Monroe
              </h1>
              <p className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted">Diligence</p>
            </div>
          </NavLink>

          {mode === "home" ? (
            <>
              <nav className="space-y-1">
                <NavLink
                  className={({ isActive }) =>
                    [
                      "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                      isActive && activeHomeSection === "hub"
                        ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
                        : "text-text-main hover:bg-white/40",
                    ].join(" ")
                  }
                  end
                  state={navigationState}
                  to="/hub"
                >
                  <WestMonroeMark className="h-7 w-7" />
                  <span className="text-[12px] font-semibold">Pathfinder</span>
                </NavLink>
              </nav>

              <SidebarSection
                action={
                  <DealSectionMenu
                    containerRef={dealMenuRef}
                    menuOpen={dealMenuOpen}
                    onAddDeal={handleAddDeal}
                    onToggleMenu={() => setDealMenuOpen((isOpen) => !isOpen)}
                  />
                }
                title="Active Deals"
              >
                {deals.map((deal) => (
                  <NavLink
                    className={({ isActive }) =>
                      [
                        "flex items-center justify-between rounded-2xl px-5 py-3 transition",
                        isActive ? "bg-white/58 shadow-[0_12px_30px_rgba(7,1,84,0.06)]" : "hover:bg-white/40",
                      ].join(" ")
                    }
                    key={deal.room.id}
                    state={navigationState}
                    to={getDealRoomPath(deal.room.id)}
                  >
                    <div className="flex items-center gap-4">
                      <span className={`h-3.5 w-3.5 rounded-full ${deal.colorClassName}`} />
                      <span className="text-[12px] font-medium text-text-main">{deal.room.name}</span>
                    </div>
                    {deal.complete ? <Icon className="h-5 w-5 text-primary/85" name="checkCircle" /> : null}
                  </NavLink>
                ))}
              </SidebarSection>

              <SidebarSection title="Internal Initiatives">
                {initiatives.map((item) => (
                  <SidebarLink icon={item.icon} key={item.name} label={item.name} />
                ))}
              </SidebarSection>

              <SidebarSection title="Research">
                <SidebarLink icon="search" label="Topics" />
                <SidebarLink
                  homeSection={activeHomeSection}
                  href="/hub/summarize"
                  icon="sparkles"
                  label="Quick Chat"
                  navigationState={navigationState}
                />
              </SidebarSection>

              <div className="border-t border-white/40 pt-6">
                <nav className="space-y-1">
                  {tools.map((item) => (
                    <SidebarLink
                      homeSection={activeHomeSection}
                      href={item.href}
                      icon={item.icon}
                      key={item.name}
                      label={item.name}
                      navigationState={navigationState}
                    />
                  ))}
                </nav>
              </div>
            </>
          ) : (
            <nav className="space-y-2">
              {dealRoomSidebarLinks.map((link) => {
                if (activeDeal && "key" in link && link.key !== "timeline") {
                  const destination = link.key === "deal-room" ? getDealRoomPath(activeDeal.room.id) : getDataRoomPath(activeDeal.room.id);

                  return (
                    <NavLink
                      className={() =>
                        [
                          "flex items-center gap-3 rounded-[22px] px-5 py-4 transition",
                          activeSection === link.key
                            ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
                            : "text-text-main hover:bg-white/40",
                        ].join(" ")
                      }
                      key={link.label}
                      onClick={() => {
                        if (link.key === "deal-room") {
                          onDealRoomSectionChange?.("deal-room");
                        }
                      }}
                      state={navigationState}
                      to={destination}
                    >
                      <Icon className="h-6 w-6" name={link.icon} />
                      <span className="text-[12px] font-semibold">{link.label}</span>
                    </NavLink>
                  );
                }

                if ("key" in link && (link.key === "timeline" || link.key === "diligence-graph" || link.key === "site-visits")) {
                  return (
                    <SidebarStaticItem
                      active={activeSection === link.key}
                      icon={link.icon}
                      key={link.label}
                      label={link.label}
                      onClick={() => onDealRoomSectionChange?.(link.key)}
                    />
                  );
                }

                return <SidebarStaticItem icon={link.icon} key={link.label} label={link.label} />;
              })}
            </nav>
          )}
        </div>

        <div className="mt-auto pt-6">
          <div className="relative">
            {profileMenuOpen ? <ProfilePreferences email={email} navigationState={navigationState} /> : null}
            <button
              aria-expanded={profileMenuOpen}
              aria-haspopup="menu"
              className="flex w-full items-center gap-4 rounded-2xl px-4 py-3 text-left transition hover:bg-white/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
              onClick={() => setProfileMenuOpen((isOpen) => !isOpen)}
              type="button"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-secondary-fixed-dim text-white">
                <span className="text-lg font-semibold">{teamLabel.slice(0, 1)}</span>
              </div>
              <div className="min-w-0">
                <p className="truncate text-[17px] font-medium text-text-main">{teamLabel}</p>
                <p className="truncate text-[12px] font-semibold text-secondary">
                  {mode === "deal-room" && activeDeal ? activeDeal.room.name : "Focus: Project Alpha"}
                </p>
              </div>
            </button>
          </div>
        </div>
      </div>

      {addDealModalOpen ? <AddDealModal onClose={() => setAddDealModalOpen(false)} /> : null}
    </aside>
  );
}

type AddDealModalProps = {
  onClose: () => void;
};

function AddDealModal({ onClose }: AddDealModalProps) {
  const [form, setForm] = useState<AddDealFormState>(emptyAddDealForm);
  const [companyError, setCompanyError] = useState("");
  const [dealTypeError, setDealTypeError] = useState("");
  const [folderError, setFolderError] = useState("");
  const companyFields = getCompanyFieldsForDealType(form.dealType);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        onClose();
      }
    }

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  function updateField(field: keyof AddDealFormState, value: string) {
    setForm((current) => ({ ...current, [field]: value }));
    if (field === "dealType") {
      setDealTypeError("");
      setCompanyError("");
    }

    if (
      field === "targetCompany" ||
      field === "buyerOrPlatformCompany" ||
      field === "parentOrSellerCompany" ||
      field === "carveOutBusiness"
    ) {
      setCompanyError("");
    }
  }

  async function handleChooseFolder() {
    setFolderError("");

    try {
      const selection = await open({
        directory: true,
        fileAccessMode: "scoped",
        multiple: false,
        title: "Choose main data room folder",
      });

      if (typeof selection === "string") {
        updateField("dataRoomFolder", selection);
      }
    } catch (error) {
      setFolderError(error instanceof Error ? error.message : String(error));
    }
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!form.dataRoomFolder) {
      setFolderError("Choose a main data room folder.");
      return;
    }

    if (!form.dealType) {
      setDealTypeError("Select a type of deal.");
      return;
    }

    if (companyFields.some((field) => !form[field.name].trim())) {
      setCompanyError("Complete the company fields for this deal type.");
      return;
    }

    onClose();
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-text-main/26 px-6 backdrop-blur-sm">
      <button aria-label="Close add deal dialog" className="absolute inset-0 cursor-default" onClick={onClose} type="button" />

      <form
        className="relative z-10 flex w-full max-w-[560px] flex-col gap-5 rounded-[28px] border border-outline-variant bg-white p-6 shadow-[0_28px_70px_rgba(7,1,84,0.2)]"
        onSubmit={handleSubmit}
      >
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] font-bold uppercase tracking-[0.2em] text-muted">Active Deals</p>
            <h2 className="mt-2 text-[2rem] font-bold leading-none text-text-main [font-family:var(--font-heading)]">
              Add deal
            </h2>
          </div>
          <button
            aria-label="Close add deal dialog"
            className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full text-muted transition hover:bg-surface-container-high hover:text-text-main focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
            onClick={onClose}
            type="button"
          >
            <Icon className="h-5 w-5 rotate-45" name="plus" />
          </button>
        </div>

        <div className="grid gap-4">
          <ModalTextField
            autoComplete="off"
            label="Deal name"
            onChange={(value) => updateField("dealName", value)}
            placeholder="Project Gamma"
            value={form.dealName}
          />

          <div className="space-y-2">
            <label className="px-1 text-[11px] font-bold uppercase tracking-[0.16em] text-muted" htmlFor="add-deal-data-room">
              Main data room folder
            </label>
            <div className="flex gap-3">
              <input
                className="min-w-0 flex-1 rounded-2xl border border-outline-variant bg-surface-container-lowest px-4 py-3 text-[14px] text-text-main outline-none transition placeholder:text-muted/60 focus:border-primary-container focus:ring-4 focus:ring-primary-fixed/40"
                id="add-deal-data-room"
                placeholder="Choose a folder"
                readOnly
                required
                value={form.dataRoomFolder}
              />
              <button
                className="flex shrink-0 items-center gap-2 rounded-2xl border border-outline-variant bg-white px-4 py-3 text-[13px] font-semibold text-primary transition hover:bg-surface-container-high focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
                onClick={handleChooseFolder}
                type="button"
              >
                <Icon className="h-4 w-4" name="folderOpen" />
                <span>Browse</span>
              </button>
            </div>
            {folderError ? <p className="px-1 text-[12px] font-medium text-error">{folderError}</p> : null}
          </div>

          <div className="space-y-2">
            <DealTypePicker error={dealTypeError} onChange={(value) => updateField("dealType", value)} value={form.dealType} />
          </div>

          {form.dealType ? (
            <div className={`grid gap-4 ${companyFields.length > 1 ? "sm:grid-cols-2" : ""}`}>
              {companyFields.map((field) => (
                <ModalTextField
                  autoComplete="organization"
                  key={field.name}
                  label={field.label}
                  onChange={(value) => updateField(field.name, value)}
                  placeholder={field.placeholder}
                  value={form[field.name]}
                />
              ))}
              {companyError ? <p className="px-1 text-[12px] font-medium text-error sm:col-span-2">{companyError}</p> : null}
            </div>
          ) : null}

          <div className="grid gap-4">
            <ModalTextField
              autoComplete="organization"
              label="PE firm"
              onChange={(value) => updateField("peFirm", value)}
              placeholder="West Monroe Capital"
              value={form.peFirm}
            />
          </div>
        </div>

        <div className="flex items-center justify-end gap-3 pt-2">
          <button
            className="rounded-full px-5 py-3 text-[13px] font-semibold text-muted transition hover:bg-surface-container-high hover:text-text-main focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
            onClick={onClose}
            type="button"
          >
            Cancel
          </button>
          <button
            className="rounded-full bg-primary-container px-6 py-3 text-[13px] font-semibold text-on-primary-container shadow-[0_10px_30px_rgba(7,1,84,0.18)] transition hover:bg-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
            type="submit"
          >
            Create deal
          </button>
        </div>
      </form>
    </div>
  );
}

type DealTypePickerProps = {
  error?: string;
  onChange: (value: string) => void;
  value: string;
};

function DealTypePicker({ error = "", onChange, value }: DealTypePickerProps) {
  const [open, setOpen] = useState(false);
  const pickerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    function handlePointerDown(event: PointerEvent) {
      if (!pickerRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("pointerdown", handlePointerDown);
    return () => document.removeEventListener("pointerdown", handlePointerDown);
  }, []);

  return (
    <div className="relative" ref={pickerRef}>
      <label className="px-1 text-[11px] font-bold uppercase tracking-[0.16em] text-muted" id="add-deal-type-label">
        Type of deal
      </label>
      <button
        aria-expanded={open}
        aria-haspopup="listbox"
        aria-labelledby="add-deal-type-label"
        className={`mt-2 flex w-full items-center justify-between rounded-2xl border border-outline-variant bg-surface-container-lowest px-4 py-3 text-left text-[14px] outline-none transition focus:border-primary-container focus:ring-4 focus:ring-primary-fixed/40 ${
          value ? "text-text-main" : "text-muted/60"
        }`}
        onClick={() => setOpen((isOpen) => !isOpen)}
        type="button"
      >
        <span>{value || "Select deal type"}</span>
        <Icon className={`h-4 w-4 text-muted transition ${open ? "rotate-180" : ""}`} name="chevronDown" />
      </button>

      {open ? (
        <div
          aria-labelledby="add-deal-type-label"
          className="absolute left-0 right-0 top-[calc(100%+0.5rem)] z-30 overflow-hidden rounded-2xl border border-outline-variant bg-white p-1.5 shadow-[0_18px_44px_rgba(7,1,84,0.14)]"
          role="listbox"
        >
          {dealTypeOptions.map((dealType) => {
            const selected = dealType === value;

            return (
              <button
                aria-selected={selected}
                className={`flex w-full items-center justify-between rounded-xl px-3 py-2.5 text-left text-[14px] font-medium transition ${
                  selected ? "bg-primary/10 text-text-main" : "text-text-main/82 hover:bg-surface-container-high"
                }`}
                key={dealType}
                onClick={() => {
                  onChange(dealType);
                  setOpen(false);
                }}
                role="option"
                type="button"
              >
                <span>{dealType}</span>
                {selected ? <Icon className="h-4 w-4 text-primary" name="check" /> : null}
              </button>
            );
          })}
        </div>
      ) : null}
      {error ? <p className="mt-2 px-1 text-[12px] font-medium text-error">{error}</p> : null}
    </div>
  );
}

type CompanyFieldConfig = {
  label: string;
  name: Extract<
    keyof AddDealFormState,
    "buyerOrPlatformCompany" | "carveOutBusiness" | "parentOrSellerCompany" | "targetCompany"
  >;
  placeholder: string;
};

function getCompanyFieldsForDealType(dealType: string): CompanyFieldConfig[] {
  switch (dealType) {
    case "Buy-side":
      return [
        {
          label: "Buyer / platform company",
          name: "buyerOrPlatformCompany",
          placeholder: "Platform Co",
        },
        {
          label: "Target company",
          name: "targetCompany",
          placeholder: "Target Co",
        },
      ];
    case "Carve-out":
      return [
        {
          label: "Parent / seller company",
          name: "parentOrSellerCompany",
          placeholder: "Parent Co",
        },
        {
          label: "Carve-out business",
          name: "carveOutBusiness",
          placeholder: "Business Unit",
        },
      ];
    case "Add-on":
      return [
        {
          label: "Platform company",
          name: "buyerOrPlatformCompany",
          placeholder: "Platform Co",
        },
        {
          label: "Add-on target",
          name: "targetCompany",
          placeholder: "Target Co",
        },
      ];
    case "Sell-side":
      return [
        {
          label: "Target company",
          name: "targetCompany",
          placeholder: "Target Co",
        },
      ];
    case "Recapitalization":
    case "Growth equity":
      return [
        {
          label: "Target company",
          name: "targetCompany",
          placeholder: "Target Co",
        },
      ];
    default:
      return [];
  }
}

type ModalTextFieldProps = {
  autoComplete?: string;
  label: string;
  onChange: (value: string) => void;
  placeholder: string;
  value: string;
};

function ModalTextField({ autoComplete, label, onChange, placeholder, value }: ModalTextFieldProps) {
  const id = `add-deal-${label.toLowerCase().replace(/\s+/g, "-")}`;

  return (
    <div className="space-y-2">
      <label className="px-1 text-[11px] font-bold uppercase tracking-[0.16em] text-muted" htmlFor={id}>
        {label}
      </label>
      <input
        autoComplete={autoComplete}
        className="w-full rounded-2xl border border-outline-variant bg-surface-container-lowest px-4 py-3 text-[14px] text-text-main outline-none transition placeholder:text-muted/60 focus:border-primary-container focus:ring-4 focus:ring-primary-fixed/40"
        id={id}
        onChange={(event) => onChange(event.currentTarget.value)}
        placeholder={placeholder}
        required
        type="text"
        value={value}
      />
    </div>
  );
}

type ProfilePreferencesProps = {
  email?: string;
  navigationState?: WorkspaceLocationState;
};

function ProfilePreferences({ email, navigationState }: ProfilePreferencesProps) {
  const navigate = useNavigate();
  const [accountError, setAccountError] = useState("");
  const [accountLoading, setAccountLoading] = useState(false);
  const { setThemeMode, themeMode } = useThemeMode();

  async function handleAccountInfo() {
    const workspaceEmail = email?.trim();

    if (!workspaceEmail) {
      navigate("/hub/account", { state: navigationState });
      return;
    }

    setAccountError("");
    setAccountLoading(true);

    try {
      const accountUser = await invoke<WorkspaceAccountUser | null>("get_user_by_email", { email: workspaceEmail });
      navigate("/hub/account", {
        state: {
          ...navigationState,
          accountLookupComplete: true,
          accountUser,
          email: workspaceEmail,
        } satisfies WorkspaceLocationState,
      });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setAccountError(message);
      navigate("/hub/account", {
        state: {
          ...navigationState,
          accountLookupComplete: true,
          accountLookupError: message,
          email: workspaceEmail,
        } satisfies WorkspaceLocationState,
      });
    } finally {
      setAccountLoading(false);
    }
  }

  return (
    <div
      className="absolute bottom-full left-0 z-20 mb-3 w-full rounded-2xl border border-outline-variant bg-white p-3 shadow-[0_18px_44px_rgba(7,1,84,0.12)]"
      role="menu"
    >
      <div className="space-y-3">
        <p className="px-2 text-[10px] font-bold uppercase tracking-[0.18em] text-muted">Theme</p>
        <div className="grid grid-cols-2 gap-1 rounded-full border border-outline-variant bg-surface-container-high p-1">
          <ThemeModeButton active={themeMode === "slate-frost"} label="Slate" onClick={() => setThemeMode("slate-frost")} />
          <ThemeModeButton active={themeMode === "dark"} label="Dark" onClick={() => setThemeMode("dark")} />
        </div>
        <div className="border-t border-outline-variant pt-3">
          <button
            className="flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-left text-[12px] font-semibold text-text-main transition hover:bg-surface-container-high focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed disabled:cursor-wait disabled:opacity-70"
            disabled={accountLoading}
            onClick={() => void handleAccountInfo()}
            role="menuitem"
            type="button"
          >
            <Icon className="h-4 w-4 text-muted" name="personSearch" />
            <span>{accountLoading ? "Loading account..." : "Account info"}</span>
          </button>
          {accountError ? <p className="mt-2 px-3 text-[11px] font-medium text-error">{accountError}</p> : null}
        </div>
      </div>
    </div>
  );
}

type ThemeModeButtonProps = {
  active: boolean;
  disabled?: boolean;
  label: string;
  onClick: () => void;
};

function ThemeModeButton({ active, disabled = false, label, onClick }: ThemeModeButtonProps) {
  return (
    <button
      aria-pressed={active}
      className={[
        "h-9 rounded-full text-[12px] font-semibold transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed disabled:cursor-not-allowed disabled:opacity-45",
        active ? "bg-primary-container text-on-primary-container shadow-[0_6px_16px_rgba(7,1,84,0.16)]" : "text-primary hover:bg-white",
      ].join(" ")}
      disabled={disabled}
      onClick={onClick}
      type="button"
    >
      {label}
    </button>
  );
}

type SidebarSectionProps = {
  action?: ReactNode;
  children: ReactNode;
  title: string;
};

function SidebarSection({ action, children, title }: SidebarSectionProps) {
  return (
    <section className="space-y-3">
      <div className="flex items-center justify-between px-5">
        <h2 className="text-[11px] font-bold uppercase tracking-[0.2em] text-muted">{title}</h2>
        {action}
      </div>
      <nav className="space-y-1">{children}</nav>
    </section>
  );
}

type DealSectionMenuProps = {
  containerRef: React.RefObject<HTMLDivElement | null>;
  menuOpen: boolean;
  onAddDeal: () => void;
  onToggleMenu: () => void;
};

function DealSectionMenu({ containerRef, menuOpen, onAddDeal, onToggleMenu }: DealSectionMenuProps) {
  return (
    <div className="relative -mr-1" ref={containerRef}>
      <button
        aria-expanded={menuOpen}
        aria-haspopup="menu"
        aria-label="Active deals actions"
        className="flex h-7 w-7 items-center justify-center rounded-full text-muted transition hover:bg-white/60 hover:text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
        onClick={onToggleMenu}
        type="button"
      >
        <Icon className="h-4 w-4" name="plus" />
      </button>

      {menuOpen ? (
        <div
          className="absolute right-0 top-full z-20 mt-2 w-36 rounded-2xl border border-outline-variant bg-white p-1.5 shadow-[0_18px_44px_rgba(7,1,84,0.12)]"
          role="menu"
        >
          <button
            className="flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-[12px] font-semibold text-text-main transition hover:bg-surface-container-high focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-fixed"
            onClick={onAddDeal}
            role="menuitem"
            type="button"
          >
            <Icon className="h-4 w-4 text-muted" name="plus" />
            <span>Add deal</span>
          </button>
        </div>
      ) : null}
    </div>
  );
}

type SidebarLinkProps = {
  homeSection?: "account" | "hub" | "summarize" | "vault";
  href?: string;
  icon: "personSearch" | "terminal" | "timeline" | "folderOpen" | "sparkles" | "search";
  label: string;
  navigationState?: WorkspaceLocationState;
};

function SidebarLink({ homeSection, href, icon, label, navigationState }: SidebarLinkProps) {
  if (href) {
    const isVaultLink = label === "Global Vault";

    return (
      <NavLink
        className={({ isActive }) =>
          [
            "flex w-full items-center gap-4 rounded-2xl px-5 py-3 text-left transition",
            isActive && (!isVaultLink || homeSection === "vault")
              ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
              : "text-text-main hover:bg-white/40",
          ].join(" ")
        }
        end
        state={navigationState}
        to={href}
      >
        <Icon className="h-6 w-6 text-current" name={icon} />
        <span className="text-[12px] font-medium">{label}</span>
      </NavLink>
    );
  }

  return (
    <button
      className="flex w-full items-center gap-4 rounded-2xl px-5 py-3 text-left text-text-main transition hover:bg-white/40"
      type="button"
    >
      <Icon className="h-6 w-6 text-muted" name={icon} />
      <span className="text-[12px] font-medium">{label}</span>
    </button>
  );
}

type SidebarStaticItemProps = {
  active?: boolean;
  icon: "dashboard" | "folderOpen" | "graph" | "grid" | "person" | "timeline";
  label: string;
  onClick?: () => void;
};

function SidebarStaticItem({ active = false, icon, label, onClick }: SidebarStaticItemProps) {
  return (
    <button
      className={[
        "flex w-full items-center gap-3 rounded-[22px] px-5 py-4 text-left transition",
        active
          ? "border border-primary/20 bg-primary/8 text-primary shadow-[inset_0_0_0_1px_rgba(80,101,142,0.12)]"
          : "text-text-main hover:bg-white/40",
      ].join(" ")}
      onClick={onClick}
      type="button"
    >
      <Icon className={`h-6 w-6 ${active ? "text-current" : "text-muted"}`} name={icon} />
      <span className={`text-[12px] ${active ? "font-semibold" : "font-medium"}`}>{label}</span>
    </button>
  );
}
