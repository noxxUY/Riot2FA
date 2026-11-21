<script lang="ts">
  import { app } from "$lib/app.svelte";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import Onboarding from "$lib/components/Onboarding.svelte";
  import Lock from "$lib/components/Lock.svelte";
  import Accounts from "$lib/components/Accounts.svelte";
  import Settings from "$lib/components/Settings.svelte";
  import AddSheet from "$lib/components/AddSheet.svelte";
  import ImportSheet from "$lib/components/ImportSheet.svelte";
  import Toast from "$lib/components/Toast.svelte";

  app.start();
</script>

<div class="app">
  {#if app.screen === "accounts"}
    <Toolbar title={app.t("accounts")} />
  {/if}

  <div class="content">
    {#if app.screen === "onboarding"}
      <Onboarding />
    {:else if app.screen === "lock"}
      <Lock />
    {:else if app.screen === "accounts"}
      <Accounts />
    {:else if app.screen === "settings"}
      <Settings />
    {/if}
  </div>

  {#if app.sheet === "add"}
    <AddSheet />
  {:else if app.sheet === "import"}
    <ImportSheet />
  {/if}

  <Toast />
</div>

<style>
  :global(:root) {
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", Inter, system-ui, sans-serif;
    font-size: 14px;
  }
  :global(body) {
    margin: 0;
    background: var(--bg, #141416);
    color: var(--text, #f5f5f7);
  }
  :global(*) {
    box-sizing: border-box;
  }
  :global(button) {
    font: inherit;
  }
  :global(:focus-visible) {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  :global(input::placeholder) {
    color: var(--muted);
  }
  @media (prefers-reduced-motion: reduce) {
    :global(*) {
      transition: none !important;
    }
  }

  .app {
    position: relative;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg);
    color: var(--text);
  }
  .content {
    flex-grow: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
</style>
