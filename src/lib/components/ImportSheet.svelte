<script lang="ts">
  import { app } from "../app.svelte";
  import { shareImport } from "../tauri";
  import Icon from "./Icon.svelte";

  let code = $state("");
  let error = $state("");
  let busy = $state(false);

  async function doImport() {
    error = "";
    busy = true;
    try {
      const acc = await shareImport(code.trim());
      app.importAccount(acc);
    } catch (e) {
      error = String(e).replace(/^Error:\s*/, "");
    } finally {
      busy = false;
    }
  }
</script>

<div class="overlay" role="button" tabindex="-1" aria-label={app.t("close")} onclick={() => app.closeSheet()} onkeydown={(e) => e.key === "Escape" && app.closeSheet()}></div>
<div class="sheet" role="dialog" aria-modal="true" aria-labelledby="imp-title">
  <div class="head">
    <h2 id="imp-title">{app.t("import")}</h2>
  </div>

  <div class="drop">
    <span class="d-icon" aria-hidden="true"><Icon name="download" size={24} stroke={1.6} /></span>
    <span class="d-title">{app.t("importDrop")}</span>
    <span class="d-sub">{app.t("importDropSub")}</span>
    <button class="choose" type="button">{app.t("importChoose")}</button>
  </div>

  <div class="field">
    <label for="imp-code">{app.t("importPasteLabel")}</label>
    <textarea
      id="imp-code"
      rows="3"
      placeholder={app.t("importPastePh")}
      spellcheck="false"
      autocomplete="off"
      bind:value={code}></textarea>
    {#if error}<span class="err">{error}</span>{/if}
  </div>

  <div class="foot">
    <button class="ghost" type="button" onclick={() => app.closeSheet()}>{app.t("close")}</button>
    <button class="primary" type="button" disabled={!code.trim() || busy} onclick={doImport}>
      {busy ? "…" : app.t("importBtn")}
    </button>
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
    padding: 20px;
    border-radius: 0 0 14px 14px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    max-height: calc(100% - 48px);
    overflow-y: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-top: none;
    color: var(--text);
    box-shadow: 0 18px 40px rgba(0, 0, 0, 0.35);
  }
  .head h2 {
    margin: 0;
    font-size: 17px;
    font-weight: 700;
  }
  .drop {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 22px 16px;
    border-radius: 12px;
    text-align: center;
    background: var(--bg);
    border: 1.5px dashed var(--border);
  }
  .d-icon {
    color: var(--accentSoftText);
  }
  .d-title {
    font-size: 14px;
    font-weight: 600;
  }
  .d-sub {
    font-size: 12px;
    line-height: 1.4;
    color: var(--muted);
    max-width: 240px;
  }
  .choose {
    height: 32px;
    margin-top: 4px;
    padding: 0 14px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    cursor: pointer;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  label {
    font-size: 13px;
    font-weight: 600;
  }
  textarea {
    box-sizing: border-box;
    padding: 10px 12px;
    border-radius: 8px;
    font-size: 13px;
    font-family: ui-monospace, "SF Mono", Menlo, monospace;
    line-height: 1.4;
    resize: vertical;
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--text);
  }
  .err {
    font-size: 12px;
    color: var(--error);
  }
  .foot {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  .ghost,
  .primary {
    height: 36px;
    padding: 0 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
  }
  .ghost {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text);
  }
  .primary {
    border: none;
    font-weight: 600;
    background: var(--accent);
    color: var(--onAccent);
  }
  .primary:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
