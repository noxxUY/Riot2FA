import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Account } from "./types";

export const inTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export interface CodeResult {
  code: string;
  seconds_remaining: number;
}

export interface AccountInfo {
  riot_id: string;
  puuid: string | null;
}

export async function totpCode(seed: string): Promise<CodeResult> {
  if (!inTauri) return mockTotp(seed);
  return invoke<CodeResult>("totp_code", { seed });
}

export async function startRiotLogin(): Promise<void> {
  if (!inTauri) {
    console.warn("[preview] start_riot_login no disponible fuera de Tauri");
    return;
  }
  await invoke("start_riot_login");
}

export async function on<T>(
  event: string,
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  if (!inTauri) return () => {};
  return listen<T>(event, (e) => handler(e.payload));
}

const PREVIEW_KEY = "riot2fa.preview.accounts";

export async function vaultLoad(): Promise<Account[]> {
  if (!inTauri) {
    try {
      const raw = localStorage.getItem(PREVIEW_KEY);
      return raw ? (JSON.parse(raw) as Account[]) : [];
    } catch {
      return [];
    }
  }
  return invoke<Account[]>("vault_load");
}

export async function biometricUnlock(allowBiometrics: boolean): Promise<boolean> {
  if (!inTauri) return true;
  return invoke<boolean>("biometric_unlock", { allowBiometrics });
}

export async function shareImport(code: string): Promise<Account> {
  if (!inTauri) {
    throw new Error("Importar por código sólo funciona dentro de la app.");
  }
  return invoke<Account>("share_import", { code });
}

export async function vaultSave(accounts: Account[]): Promise<void> {
  if (!inTauri) {
    try {
      localStorage.setItem(PREVIEW_KEY, JSON.stringify(accounts));
    } catch {

    }
    return;
  }
  await invoke("vault_save", { accounts });
}

function mockTotp(seed: string): CodeResult {
  const period = 30;
  const now = Math.floor(Date.now() / 1000);
  let h = 0;
  const material = seed + Math.floor(now / period);
  for (let i = 0; i < material.length; i++) {
    h = (Math.imul(h, 31) + material.charCodeAt(i)) | 0;
  }
  const code = String(Math.abs(h) % 1_000_000).padStart(6, "0");
  return { code, seconds_remaining: period - (now % period) };
}
