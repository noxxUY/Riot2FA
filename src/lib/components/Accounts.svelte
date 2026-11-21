<script lang="ts">
  import { app } from "../app.svelte";
  import AccountCard from "./AccountCard.svelte";
  import Icon from "./Icon.svelte";

  let filtered = $derived.by(() => {
    const q = app.query.trim().toLowerCase();
    if (!q) return app.accounts;
    return app.accounts.filter((a) =>
      `${a.riotId} ${a.label} ${a.region}`.toLowerCase().includes(q),
    );
  });
</script>

{#if app.accounts.length === 0}
  <div class="empty">
    <div class="badge" aria-hidden="true"><Icon name="shield-plus" size={30} stroke={1.6} /></div>
    <div class="empty-text">
      <h2>{app.t("emptyTitle")}</h2>
      <p>{app.t("emptySub")}</p>
    </div>
    <div class="empty-actions">
      <button class="ghost" type="button" onclick={() => app.openImport()}>{app.t("import")}</button>
      <button class="primary" type="button" onclick={() => app.openAdd()}>{app.t("addAccount")}</button>
    </div>
  </div>
{:else}
  <div class="search">
    <label for="search" class="sr-only">{app.t("searchPlaceholder")}</label>
    <span class="search-icon" aria-hidden="true"><Icon name="search" size={15} stroke={2} /></span>
    <input id="search" type="search" placeholder={app.t("searchPlaceholder")} bind:value={app.query} />
  </div>

  <div class="list">
    {#each filtered as account (account.id)}
      <AccountCard {account} />
    {/each}
    {#if filtered.length === 0}
      <p class="no-results">{app.t("noResults")} “{app.query}”.</p>
    {/if}
  </div>

  <div class="foot">
    <Icon name="copy" size={13} stroke={2} />
    {app.t("copyHint")}
  </div>
{/if}

<style>
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
  }
  .search {
    padding: 14px 16px 8px;
    position: relative;
  }
  .search-icon {
    position: absolute;
    left: 27px;
    top: 50%;
    transform: translateY(-30%);
    color: var(--muted);
    pointer-events: none;
  }
  input[type="search"] {
    width: 100%;
    height: 34px;
    box-sizing: border-box;
    padding: 0 12px 0 32px;
    border-radius: 9px;
    border: 1px solid transparent;
    font-size: 13px;
    background: var(--surface2);
    color: var(--text);
    outline-offset: 2px;
  }
  .list {
    flex-grow: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 6px 16px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .no-results {
    margin: 24px 0 0;
    text-align: center;
    font-size: 13px;
    color: var(--muted);
  }
  .foot {
    flex-shrink: 0;
    padding: 10px 16px 12px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-size: 12px;
    color: var(--muted);
  }
  .empty {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
    padding: 32px;
    text-align: center;
  }
  .badge {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .empty-text {
    display: flex;
    flex-direction: column;
    gap: 6px;
    max-width: 280px;
  }
  .empty-text h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 700;
  }
  .empty-text p {
    margin: 0;
    font-size: 14px;
    line-height: 1.45;
    color: var(--muted);
  }
  .empty-actions {
    display: flex;
    gap: 8px;
  }
  .ghost,
  .primary {
    height: 40px;
    padding: 0 18px;
    border-radius: 10px;
    font-size: 14px;
    cursor: pointer;
  }
  .ghost {
    font-weight: 500;
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
</style>
