<script lang="ts">
  import { app } from "../app.svelte";
  import Icon from "./Icon.svelte";

  let { title }: { title: string } = $props();
</script>

<div class="bar" data-tauri-drag-region>
  <span class="title">{title}</span>
  <div class="actions">
    <button class="icon-btn" type="button" title={app.t("tipImport")} aria-label={app.t("tipImport")} onclick={() => app.openImport()}>
      <Icon name="download" />
    </button>
    <button class="icon-btn" type="button" title={app.t("tipSettings")} aria-label={app.t("tipSettings")} onclick={() => app.go("settings")}>
      <Icon name="settings" />
    </button>
    <button class="icon-btn" type="button" title={app.t("tipLock")} aria-label={app.t("tipLock")} onclick={() => app.lock()}>
      <Icon name="lock" />
    </button>
    <button class="add-btn" type="button" onclick={() => app.openAdd()}>
      <Icon name="plus" size={16} stroke={2.25} />
      {app.t("add")}
    </button>
  </div>
</div>

<style>
  .bar {
    height: 48px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 10px 0 16px;
    background: var(--chrome);
    border-bottom: 1px solid var(--border);
  }
  .title {
    font-size: 14px;
    font-weight: 600;
    flex-grow: 1;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .icon-btn {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--muted);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }
  .icon-btn:hover {
    background: var(--surface2);
    color: var(--text);
  }
  .add-btn {
    height: 32px;
    margin-left: 6px;
    padding: 0 12px 0 9px;
    border-radius: 8px;
    border: none;
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 13px;
    font-weight: 600;
    background: var(--accent);
    color: var(--onAccent);
    cursor: pointer;
  }
  .add-btn:active {
    transform: scale(0.97);
  }
</style>
