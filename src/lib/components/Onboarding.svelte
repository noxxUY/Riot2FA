<script lang="ts">
  import { app, ONBOARDING_STEPS } from "../app.svelte";
  import { THEMES, THEME_ORDER, type ThemeKey } from "../themes";
  import type { Protection } from "../types";
  import Icon from "./Icon.svelte";

  const themeLabel: Record<ThemeKey, string> = {
    light: "themeLight", dark: "themeDark", red: "themeRed", graphite: "themeGraphite",
  };
  const protections: { key: Protection; label: string; desc: string; icon: string; recommended?: boolean }[] = [
    { key: "touchid", label: "touchId", desc: "touchIdDesc", icon: "fingerprint", recommended: true },
    { key: "password", label: "macPassword", desc: "macPasswordDesc", icon: "key" },
  ];

  const features = [
    { icon: "copy", title: "featCopyTitle", desc: "featCopyDesc" },
    { icon: "lock", title: "featEncTitle", desc: "featEncDesc" },
    { icon: "users", title: "featAllTitle", desc: "featAllDesc" },
  ];
  const steps = [
    { n: 1, title: "step1Title", desc: "step1Desc" },
    { n: 2, title: "step2Title", desc: "step2Desc" },
    { n: 3, title: "step3Title", desc: "step3Desc" },
  ];

  let nextLabel = $derived(
    app.stepIndex === 0
      ? app.t("getStarted")
      : app.stepIndex === ONBOARDING_STEPS.length - 1
        ? app.t("goToAccounts")
        : app.t("continue"),
  );
</script>

<div class="onb">
  <div class="body">
    {#if app.onboardingStep === "welcome"}
      <div class="welcome">
        <div class="hero">
          <div class="logo" aria-hidden="true"><Icon name="shield-check" size={34} stroke={1.75} /></div>
          <div class="hero-text">
            <h1>Riot2FA</h1>
            <p>{app.t("welcomeSub")}</p>
          </div>
        </div>
        <div class="features">
          {#each features as f}
            <div class="feature">
              <span class="f-icon" aria-hidden="true"><Icon name={f.icon} size={17} /></span>
              <div class="f-text">
                <span class="f-title">{app.t(f.title)}</span>
                <span class="f-desc">{app.t(f.desc)}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else if app.onboardingStep === "theme"}
      <div class="step">
        <div class="step-head"><h2>{app.t("themeTitle")}</h2><p>{app.t("themeSub")}</p></div>
        <div class="theme-grid">
          {#each THEME_ORDER as key}
            <button class="theme-card" class:sel={app.theme === key} type="button" aria-pressed={app.theme === key} onclick={() => app.setTheme(key)}>
              <span class="preview" style="background: {THEMES[key].bg}; border-bottom: 1px solid {THEMES[key].border};">
                <span class="bar" style="background: {THEMES[key].accent};"></span>
                <span class="line" style="background: {THEMES[key].surface}; border: 1px solid {THEMES[key].border};"></span>
                <span class="line" style="background: {THEMES[key].surface}; border: 1px solid {THEMES[key].border};"></span>
              </span>
              <span class="theme-foot">
                <span>{app.t(themeLabel[key])}</span>
                {#if app.theme === key}<span class="tick" aria-hidden="true"><Icon name="check" size={12} stroke={3} /></span>{/if}
              </span>
            </button>
          {/each}
        </div>
      </div>
    {:else if app.onboardingStep === "protection"}
      <div class="step">
        <div class="step-head"><h2>{app.t("protectTitle")}</h2><p>{app.t("protectSub")}</p></div>
        <div class="protects">
          {#each protections as p}
            <button class="protect" class:sel={app.protection === p.key} type="button" aria-pressed={app.protection === p.key} onclick={() => app.setProtection(p.key)}>
              <span class="p-icon" aria-hidden="true"><Icon name={p.icon} size={22} stroke={1.6} /></span>
              <span class="p-text">
                <span class="p-title">{app.t(p.label)}{#if p.recommended}<span class="badge">{app.t("recommended")}</span>{/if}</span>
                <span class="p-desc">{app.t(p.desc)}</span>
              </span>
              <span class="radio" class:on={app.protection === p.key} aria-hidden="true"></span>
            </button>
          {/each}
        </div>
        <div class="info">
          <Icon name="shield" size={16} stroke={1.75} />
          <span>{app.t("protectNote")}</span>
        </div>
      </div>
    {:else}
      <div class="step">
        <div class="step-head"><h2>{app.t("howtoTitle")}</h2><p>{app.t("howtoSub")}</p></div>
        <ol class="howto">
          {#each steps as s}
            <li>
              <span class="num" aria-hidden="true">{s.n}</span>
              <div class="how-text">
                <span class="how-title">{app.t(s.title)}</span>
                <span class="how-desc">{app.t(s.desc)}</span>
              </div>
            </li>
          {/each}
        </ol>
      </div>
    {/if}
  </div>

  <div class="nav">
    <div class="dots" role="img" aria-label={`${app.stepIndex + 1} / ${ONBOARDING_STEPS.length}`}>
      {#each ONBOARDING_STEPS as _, i}
        <span class="dot" class:active={i === app.stepIndex}></span>
      {/each}
    </div>
    {#if app.stepIndex > 0}
      <button class="back" type="button" onclick={() => app.prevStep()}>{app.t("back")}</button>
    {/if}
    <button class="next" type="button" onclick={() => app.nextStep()}>{nextLabel}</button>
  </div>
</div>

<style>
  .onb {
    flex-grow: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    padding: 32px 32px 24px;
  }
  .body {
    flex-grow: 1;
    min-height: 0;
    overflow-y: auto;
  }
  h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: -0.02em;
  }
  h2 {
    margin: 0;
    font-size: 22px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }
  .welcome {
    display: flex;
    flex-direction: column;
    gap: 28px;
    padding-top: 12px;
  }
  .hero {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .logo {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accent);
    color: var(--onAccent);
  }
  .hero-text {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .hero-text p {
    margin: 0;
    font-size: 16px;
    line-height: 1.45;
    color: var(--muted);
  }
  .features {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }
  .feature {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }
  .f-icon {
    width: 32px;
    height: 32px;
    flex-shrink: 0;
    border-radius: 9px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .f-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .f-title {
    font-weight: 600;
    font-size: 14px;
  }
  .f-desc {
    font-size: 13px;
    line-height: 1.4;
    color: var(--muted);
  }
  .step {
    display: flex;
    flex-direction: column;
    gap: 22px;
  }
  .step-head {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .step-head p {
    margin: 0;
    font-size: 14px;
    line-height: 1.45;
    color: var(--muted);
  }
  .theme-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }
  .theme-card {
    padding: 0;
    border-radius: 12px;
    overflow: hidden;
    text-align: left;
    background: var(--surface);
    border: 2px solid var(--border);
    color: var(--text);
    cursor: pointer;
  }
  .theme-card.sel {
    border-color: var(--accent);
  }
  .preview {
    height: 72px;
    padding: 10px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .preview .bar {
    height: 6px;
    width: 40%;
    border-radius: 3px;
  }
  .preview .line {
    height: 14px;
    border-radius: 5px;
  }
  .theme-foot {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px 11px;
    font-size: 14px;
    font-weight: 600;
  }
  .tick {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accent);
    color: var(--onAccent);
  }
  .protects {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .protect {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px;
    border-radius: 12px;
    background: var(--surface);
    border: 2px solid var(--border);
    color: var(--text);
    cursor: pointer;
    text-align: left;
  }
  .protect.sel {
    border-color: var(--accent);
  }
  .p-icon {
    width: 42px;
    height: 42px;
    flex-shrink: 0;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .p-text {
    display: flex;
    flex-direction: column;
    gap: 3px;
    flex-grow: 1;
  }
  .p-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 15px;
    font-weight: 600;
  }
  .badge {
    font-size: 11px;
    font-weight: 600;
    padding: 2px 7px;
    border-radius: 999px;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .p-desc {
    font-size: 13px;
    line-height: 1.4;
    color: var(--muted);
  }
  .radio {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
    box-sizing: border-box;
    border-radius: 50%;
    border: 2px solid var(--border);
  }
  .radio.on {
    border-color: var(--accent);
    background:
      radial-gradient(circle, var(--accent) 0 5px, transparent 6px);
  }
  .info {
    display: flex;
    gap: 10px;
    align-items: flex-start;
    padding: 12px 14px;
    border-radius: 10px;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--accentSoftText);
  }
  .info span {
    font-size: 13px;
    line-height: 1.45;
    color: var(--muted);
  }
  .howto {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .howto li {
    display: flex;
    gap: 14px;
    padding: 12px 0;
  }
  .num {
    width: 28px;
    height: 28px;
    flex-shrink: 0;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    font-weight: 700;
    background: var(--accentSoft);
    color: var(--accentSoftText);
  }
  .how-text {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .how-title {
    font-size: 14px;
    font-weight: 600;
  }
  .how-desc {
    font-size: 13px;
    line-height: 1.45;
    color: var(--muted);
  }
  .nav {
    display: flex;
    align-items: center;
    gap: 12px;
    padding-top: 16px;
  }
  .dots {
    display: flex;
    gap: 6px;
    flex-grow: 1;
  }
  .dots .dot {
    height: 6px;
    width: 6px;
    border-radius: 3px;
    background: var(--border);
    transition: width 0.2s ease;
  }
  .dots .dot.active {
    width: 20px;
    background: var(--accent);
  }
  .back {
    height: 40px;
    padding: 0 14px;
    border: none;
    border-radius: 10px;
    background: transparent;
    font-size: 14px;
    font-weight: 500;
    color: var(--muted);
    cursor: pointer;
  }
  .next {
    height: 40px;
    padding: 0 20px;
    border: none;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 600;
    background: var(--accent);
    color: var(--onAccent);
    cursor: pointer;
  }
</style>
