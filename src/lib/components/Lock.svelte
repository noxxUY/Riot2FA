<script lang="ts">
  import { app } from "../app.svelte";
  import { biometricUnlock } from "../tauri";
  import Icon from "./Icon.svelte";

  let busy = $state(false);
  let failed = $state(false);

  let isTouch = $derived(app.protection === "touchid");

  async function unlock() {
    if (busy) return;
    busy = true;
    failed = false;
    try {
      const ok = await biometricUnlock(isTouch);
      if (ok) app.unlock();
      else failed = true;
    } catch {
      failed = true;
    } finally {
      busy = false;
    }
  }
</script>

<div class="lock">
  <div class="badge" aria-hidden="true"><Icon name="lock" size={32} stroke={1.6} /></div>
  <div class="text">
    <h2>{app.t("lockedTitle")}</h2>
    <p>{app.t("lockedSub")}</p>
  </div>

  <div class="actions">
    <button class="primary" type="button" onclick={unlock} disabled={busy}>
      {#if busy}
        <span class="spinner" aria-hidden="true"></span>
        {app.t("unlocking")}
      {:else}
        <Icon name={isTouch ? "fingerprint" : "lock"} size={18} stroke={1.75} />
        {isTouch ? app.t("unlockTouch") : app.t("unlock")}
      {/if}
    </button>
    {#if failed}
      <span class="failed">{app.t("unlockFailed")}</span>
    {/if}
  </div>
</div>

<style>
  .lock {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 24px;
    padding: 32px;
    text-align: center;
  }
  .badge {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .text {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .text h2 {
    margin: 0;
    font-size: 20px;
    font-weight: 700;
  }
  .text p {
    margin: 0;
    font-size: 14px;
    color: var(--muted);
  }
  .actions {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    width: 100%;
    max-width: 260px;
  }
  .primary {
    height: 44px;
    width: 100%;
    border: none;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 14px;
    font-weight: 600;
    background: var(--accent);
    color: var(--onAccent);
    cursor: pointer;
  }
  .primary:disabled {
    opacity: 0.75;
    cursor: default;
  }
  .spinner {
    width: 15px;
    height: 15px;
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
  .failed {
    font-size: 13px;
    color: var(--accentSoftText);
  }
</style>
