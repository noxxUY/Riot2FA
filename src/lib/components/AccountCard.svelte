<script lang="ts">
  import { app } from "../app.svelte";
  import { totpCode } from "../tauri";
  import type { Account } from "../types";
  import Icon from "./Icon.svelte";

  let { account }: { account: Account } = $props();

  const CIRC = 56.55;

  let code = $state("------");
  let lastPeriod = -1;

  function initials(riotId: string): string {
    const name = riotId.split("#")[0].trim();
    const parts = name.split(/\s+/);
    const s = parts.length > 1 ? parts[0][0] + parts[1][0] : name.slice(0, 2);
    return s.toLowerCase();
  }

  async function refresh() {
    try {
      const r = await totpCode(account.seed);
      code = r.code;
    } catch {
      code = "------";
    }
  }

  $effect(() => {
    const period = Math.floor(app.now / 30);
    if (period !== lastPeriod) {
      lastPeriod = period;
      refresh();
    }
  });

  let remaining = $derived(30 - (app.now % 30));
  let revealed = $derived(app.revealId === account.id);
  let low = $derived(remaining <= 5);
  let meta = $derived(account.label ? `${account.label} · ${account.region}` : account.region);
  let codeText = $derived(code.length === 6 ? `${code.slice(0, 3)} ${code.slice(3)}` : code);

  async function onClick() {
    try {
      const r = await totpCode(account.seed);
      code = r.code;
      try {
        await navigator.clipboard?.writeText(r.code);
      } catch {

      }
      app.reveal(account.id);
      app.showToast(app.t("toastCopied"));
    } catch {

    }
  }
</script>

<button
  class="card"
  type="button"
  class:revealed
  aria-label={revealed ? `${app.t("copied")} · ${account.riotId}` : `${app.t("clickToCopy")} · ${account.riotId}`}
  onclick={onClick}>
  <span class="avatar" aria-hidden="true">{initials(account.riotId)}</span>

  <span class="info">
    <span class="riot-id">{account.riotId}</span>
    <span class="meta">{meta}</span>
  </span>

  <span class="code-wrap">
    <span class="code" class:blur={!revealed} aria-hidden="true">{codeText}</span>
    <span class="hint" class:on={revealed}>{revealed ? app.t("copied") : app.t("clickToCopy")}</span>
  </span>

  <span class="ring" aria-hidden="true">
    <svg width="26" height="26" viewBox="0 0 24 24">
      <circle class="track" cx="12" cy="12" r="9" />
      <circle
        class="prog"
        class:low
        cx="12"
        cy="12"
        r="9"
        stroke-dasharray={CIRC}
        stroke-dashoffset={(CIRC * (1 - remaining / 30)).toFixed(2)} />
    </svg>
    <span class="secs" class:low>{remaining}</span>
  </span>
</button>

<style>
  .card {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 12px 12px 14px;
    border-radius: 12px;
    text-align: left;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease, transform 0.08s ease;
  }
  .card:hover {
    background: var(--surface2);
  }
  .card:active {
    transform: scale(0.99);
  }
  .card.revealed {
    border-color: var(--accent);
  }
  .avatar {
    width: 38px;
    height: 38px;
    flex-shrink: 0;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    font-weight: 700;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex-grow: 1;
    min-width: 0;
  }
  .riot-id {
    font-size: 15px;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .meta {
    font-size: 12px;
    color: var(--muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .code-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 2px;
  }
  .code {
    font-family: ui-monospace, "SF Mono", Menlo, monospace;
    font-variant-numeric: tabular-nums;
    font-size: 20px;
    font-weight: 600;
    letter-spacing: 0.02em;
    user-select: none;
    transition: filter 0.2s ease;
  }
  .code.blur {
    filter: blur(7px);
    opacity: 0.8;
  }
  .hint {
    font-size: 11px;
    font-weight: 500;
    color: var(--muted);
  }
  .hint.on {
    color: var(--accentSoftText);
  }
  .ring {
    position: relative;
    width: 26px;
    height: 26px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .ring svg {
    position: absolute;
    inset: 0;
    transform: rotate(-90deg);
    fill: none;
    stroke-width: 2.25;
    stroke-linecap: round;
  }
  .track {
    stroke: var(--border);
  }
  .prog {
    stroke: var(--muted);
    transition: stroke-dashoffset 1s linear;
  }
  .prog.low {
    stroke: var(--accent);
  }
  .secs {
    position: relative;
    font-size: 9px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--muted);
  }
  .secs.low {
    color: var(--accent);
  }
</style>
