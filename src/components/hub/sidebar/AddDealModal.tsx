import { open } from "@tauri-apps/plugin-dialog";
import { FormEvent, useEffect, useState } from "react";
import { Icon } from "../../ui/Icon";
import { DealTypePicker } from "./DealTypePicker";
import { ModalTextField } from "./ModalTextField";

type AddDealModalProps = {
  onClose: () => void;
};

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

export function AddDealModal({ onClose }: AddDealModalProps) {
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
