<script lang="ts">
  import { app } from "../app.svelte";
  import { THEMES, THEME_ORDER, type ThemeKey } from "../themes";
  import type { Protection } from "../types";
  import Icon from "./Icon.svelte";

  const themeLabel: Record<ThemeKey, string> = {
    light: "themeLight", dark: "themeDark", red: "themeRed", graphite: "themeGraphite",
  };
  const protections: { key: Protection; label: string }[] = [
    { key: "touchid", label: "touchId" },
    { key: "password", label: "macPassword" },
  ];
</script>

<div class="settings">
  <button class="back" type="button" onclick={() => app.go("accounts")}>
    <Icon name="chevron-left" size={18} stroke={2} />
    {app.t("accounts")}
  </button>

  <section>
    <h3>{app.t("secTheme")}</h3>
    <div class="themes">
      {#each THEME_ORDER as key}
        <button
          class="theme"
          class:sel={app.theme === key}
          type="button"
          aria-pressed={app.theme === key}
          onclick={() => app.setTheme(key)}>
          <span class="swatch" style="background: {THEMES[key].bg}; box-shadow: inset 0 0 0 1px {THEMES[key].border};">
            <span class="dot" style="background: {THEMES[key].accent};"></span>
          </span>
          <span class="theme-name">{app.t(themeLabel[key])}</span>
        </button>
      {/each}
    </div>
  </section>

  <section>
    <h3>{app.t("secSecurity")}</h3>
    <div class="card">
      <div class="row">
        <span>{app.t("unlockWith")}</span>
        <div class="seg" role="group" aria-label={app.t("unlockWith")}>
          {#each protections as p}
            <button
              class="seg-btn"
              class:sel={app.protection === p.key}
              type="button"
              aria-pressed={app.protection === p.key}
              onclick={() => app.setProtection(p.key)}>{app.t(p.label)}</button>
          {/each}
        </div>
      </div>
      <div class="row">
        <label for="autolock">{app.t("autoLock")}</label>
        <select id="autolock" value={String(app.settings.autoLock)} onchange={(e) => app.updateSettings({ autoLock: Number((e.target as HTMLSelectElement).value) })}>
          <option value="1">{app.t("autoLock1")}</option>
          <option value="5">{app.t("autoLock5")}</option>
          <option value="15">{app.t("autoLock15")}</option>
          <option value="0">{app.t("autoLockNever")}</option>
        </select>
      </div>
      <div class="row last">
        <label for="hide">{app.t("hideCode")}</label>
        <select id="hide" value={String(app.settings.hideAfter)} onchange={(e) => app.updateSettings({ hideAfter: Number((e.target as HTMLSelectElement).value) })}>
          <option value="5">{app.t("hide5")}</option>
          <option value="10">{app.t("hide10")}</option>
          <option value="30">{app.t("hide30")}</option>
        </select>
      </div>
    </div>
    <p class="note">{app.t("encNote")}</p>
  </section>

  <section>
    <h3>{app.t("secHelp")}</h3>
    <div class="card">
      <button class="link-row" type="button" onclick={() => app.replayTutorial()}>
        {app.t("replayTutorial")}
        <Icon name="chevron-right" size={16} stroke={2} />
      </button>
    </div>
    <p class="note">{app.t("aboutNote")}</p>
  </section>
</div>

<style>
  .settings {
    flex-grow: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 12px 16px 20px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  .back {
    align-self: flex-start;
    height: 32px;
    padding: 0 10px 0 4px;
    border: none;
    border-radius: 8px;
    background: transparent;
    display: flex;
    align-items: center;
    gap: 2px;
    font-size: 14px;
    font-weight: 500;
    color: var(--accentSoftText);
    cursor: pointer;
  }
  section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  h3 {
    margin: 0 4px;
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--muted);
  }
  .themes {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
  }
  .theme {
    padding: 8px 6px;
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    background: var(--surface);
    border: 2px solid var(--border);
    color: var(--text);
    cursor: pointer;
  }
  .theme.sel {
    border-color: var(--accent);
  }
  .swatch {
    width: 100%;
    height: 30px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }
  .theme-name {
    font-size: 12px;
    font-weight: 600;
  }
  .card {
    border-radius: 12px;
    overflow: hidden;
    background: var(--surface);
    border: 1px solid var(--border);
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 12px 14px;
    border-bottom: 1px solid var(--border);
    font-size: 14px;
  }
  .row.last {
    border-bottom: none;
  }
  .seg {
    display: flex;
    padding: 2px;
    border-radius: 8px;
    background: var(--surface2);
  }
  .seg-btn {
    height: 26px;
    padding: 0 12px;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 600;
    background: transparent;
    color: var(--muted);
    cursor: pointer;
  }
  .seg-btn.sel {
    background: var(--surface);
    color: var(--text);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  }
  select {
    height: 30px;
    padding: 0 8px;
    border-radius: 7px;
    font-size: 13px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text);
  }
  .link-row {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    border: none;
    background: transparent;
    font-size: 14px;
    text-align: left;
    color: var(--text);
    cursor: pointer;
  }
  .note {
    margin: 0 4px;
    font-size: 12px;
    line-height: 1.45;
    color: var(--muted);
  }
</style>
