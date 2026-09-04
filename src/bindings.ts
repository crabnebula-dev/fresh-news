/**
 * Typed wrappers around the Rust commands in `src-tauri/src/commands.rs`.
 *
 * `invoke` is untyped (`Promise<unknown>` in, `any` out), so keep every call to
 * it in this file. The rest of the app imports these functions and gets real
 * types and real errors.
 */
import { invoke } from "@tauri-apps/api/core";

/** Mirrors `commands::Error` — `#[serde(tag = "kind", content = "message")]`. */
export type CommandError = { kind: "InvalidInput"; message: string };

/** Mirrors `commands::AppInfo`. */
export interface AppInfo {
  name: string;
  version: string;
  tauriVersion: string;
  os: string;
  arch: string;
}

/** Narrows a caught value to a structured error from Rust. */
export function isCommandError(value: unknown): value is CommandError {
  return (
    typeof value === "object" &&
    value !== null &&
    "kind" in value &&
    typeof (value as { kind: unknown }).kind === "string"
  );
}

/** Turns anything thrown by `invoke` into a message worth showing a user. */
export function errorMessage(value: unknown): string {
  if (isCommandError(value)) return value.message;
  if (value instanceof Error) return value.message;
  return String(value);
}

export function greet(name: string): Promise<string> {
  return invoke("greet", { name });
}

export function appInfo(): Promise<AppInfo> {
  return invoke("app_info");
}

export function bumpCounter(by: number): Promise<number> {
  // Struct arguments are passed as a single object matching the Rust field name.
  return invoke("bump_counter", { args: { by } });
}

export function slowTask(millis: number): Promise<string> {
  return invoke("slow_task", { millis });
}
