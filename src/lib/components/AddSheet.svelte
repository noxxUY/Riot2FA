<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { app } from "../app.svelte";
  import { startRiotLogin, on } from "../tauri";
  import type { Account } from "../types";
  import Icon from "./Icon.svelte";

  type Status = "idle" | "signing" | "activating";
  let status = $state<Status>("idle");
  let error = $state("");

  let cleanups: Array<() => void> = [];

  onMount(async () => {
    cleanups.push(
      await on("riot-login-captured", () => {
        status = "activating";
      }),
    );
    cleanups.push(
      await on<Account>("riot-account-added", (acc) => {
        status = "idle";
        app.importAccount(acc);
      }),
    );
    cleanups.push(
      await on<string>("riot-add-error", (msg) => {
        status = "idle";
        error = msg;
      }),
    );
    cleanups.push(await on("riot-login-cancelled", () => (status = "idle")));
    cleanups.push(await on("riot-login-timeout", () => (status = "idle")));
  });
  onDestroy(() => cleanups.forEach((c) => c()));

  async function signIn() {
    error = "";
    status = "signing";
    await startRiotLogin();
  }

  let busy = $derived(status !== "idle");
  let btnLabel = $derived(
    status === "signing"
      ? app.t("addSigningIn")
      : status === "activating"
        ? app.t("addActivating")
        : error
          ? app.t("retry")
          : app.t("addSignInBtn"),
  );
</script>

<div
  class="overlay"
  role="button"
  tabindex="-1"
  aria-label={app.t("close")}
  onclick={() => !busy && app.closeSheet()}
  onkeydown={(e) => e.key === "Escape" && !busy && app.closeSheet()}>
</div>
<div class="sheet" role="dialog" aria-modal="true" aria-labelledby="add-title">
  <div class="head">
    <div class="badge" aria-hidden="true"><Icon name="shield-check" size={24} stroke={1.7} /></div>
    <h2 id="add-title">{app.t("addSignInTitle")}</h2>
    <p>{app.t("addSignInBody")}</p>
  </div>

  <button class="signin" type="button" onclick={signIn} disabled={busy}>
    {#if busy}
      <span class="spinner" aria-hidden="true"></span>
    {:else}
      <Icon name="shield" size={18} stroke={1.75} />
    {/if}
    {btnLabel}
  </button>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  <div class="note">
    <Icon name="lock" size={15} stroke={1.75} />
    <span>{app.t("addSignInNote")}</span>
  </div>

  <div class="foot">
    <button class="ghost" type="button" onclick={() => app.closeSheet()} disabled={busy}>{app.t("cancel")}</button>
  </div>
</div>

<style>
  .overlay {
    position: absolute;
    inset: 48px 0 0 0;
    background: rgba(0, 0, 0, 0.45);
    border: none;
    cursor: default;
  }
  .sheet {
    position: absolute;
    top: 48px;
    left: 0;
    right: 0;
    box-sizing: border-box;
    padding: 22px 20px 16px;
    border-radius: 0 0 14px 14px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-height: calc(100% - 48px);
    overflow-y: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-top: none;
    color: var(--text);
    box-shadow: 0 18px 40px rgba(0, 0, 0, 0.35);
  }
  .head {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .badge {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accent);
    color: var(--onAccent);
  }
  .head h2 {
    margin: 0;
    font-size: 19px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }
  .head p {
    margin: 0;
    font-size: 14px;
    line-height: 1.45;
    color: var(--muted);
  }
  .signin {
    height: 46px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 9px;
    border: none;
    border-radius: 11px;
    font-size: 15px;
    font-weight: 600;
    background: var(--accent);
    color: var(--onAccent);
    cursor: pointer;
  }
  .signin:disabled {
    opacity: 0.75;
    cursor: default;
  }
  .spinner {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid color-mix(in srgb, var(--onAccent) 35%, transparent);
    border-top-color: var(--onAccent);
    animation: spin 0.7s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .error {
    margin: 0;
    padding: 10px 12px;
    border-radius: 9px;
    font-size: 13px;
    line-height: 1.45;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .note {
    display: flex;
    gap: 10px;
    align-items: flex-start;
    padding: 12px 14px;
    border-radius: 10px;
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--accentSoftText);
  }
  .note span {
    font-size: 12.5px;
    line-height: 1.45;
    color: var(--muted);
  }
  .foot {
    display: flex;
    justify-content: flex-end;
  }
  .ghost {
    height: 36px;
    padding: 0 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text);
    cursor: pointer;
  }
  .ghost:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
