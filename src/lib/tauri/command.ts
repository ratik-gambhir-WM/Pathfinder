import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type { TauriCommandName } from "../constants";

export class TauriCommandError extends Error {
  command: TauriCommandName;
  originalError: unknown;

  constructor(command: TauriCommandName, originalError: unknown) {
    super(`Tauri command "${command}" failed: ${getTauriErrorMessage(originalError)}`);
    this.name = "TauriCommandError";
    this.command = command;
    this.originalError = originalError;
    Object.setPrototypeOf(this, TauriCommandError.prototype);
  }
}

export async function execute<TResponse = unknown, TArgs extends InvokeArgs = InvokeArgs>(
  command: TauriCommandName,
  args?: TArgs,
): Promise<TResponse> {
  try {
    return await invoke<TResponse>(command, args);
  } catch (error) {
    throw new TauriCommandError(command, error);
  }
}

function getTauriErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  if (error && typeof error === "object" && "message" in error) {
    const message = (error as { message: unknown }).message;
    if (typeof message === "string") {
      return message;
    }
  }

  try {
    return JSON.stringify(error);
  } catch {
    return String(error);
  }
}
