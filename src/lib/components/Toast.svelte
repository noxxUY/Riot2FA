<script lang="ts">
  import { app } from "../app.svelte";
  import Icon from "./Icon.svelte";
</script>

<div class="wrap" aria-live="polite">
  {#if app.toast}
    <span class="toast">
      <Icon name="check" size={14} stroke={2.75} />
      {app.toast}
    </span>
  {/if}
</div>

<style>
  .wrap {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 20px;
    display: flex;
    justify-content: center;
    pointer-events: none;
  }
  .toast {
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 8px 14px;
    border-radius: 999px;
    font-size: 13px;
    font-weight: 600;
    background: var(--text);
    color: var(--bg);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
  }
</style>
