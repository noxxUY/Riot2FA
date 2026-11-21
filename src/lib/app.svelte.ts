import { applyTheme, type ThemeKey } from "./themes";
import { detectLang, translator, type Lang } from "./i18n";
import type { Account, OnboardingStep, Protection, Screen, Settings } from "./types";
import { vaultLoad, vaultSave } from "./tauri";

export const ONBOARDING_STEPS: OnboardingStep[] = [
  "welcome", "theme", "protection", "tutorial",
];

const PREFS_KEY = "riot2fa.prefs.v1";

interface Prefs {
  theme: ThemeKey;
  lang: Lang;
  protection: Protection;
  settings: Settings;
  onboardingDone: boolean;
}

function loadPrefs(): Partial<Prefs> {
  try {
    const raw = localStorage.getItem(PREFS_KEY);
    return raw ? JSON.parse(raw) : {};
  } catch {
    return {};
  }
}

class AppState {
  screen = $state<Screen>("onboarding");
  onboardingStep = $state<OnboardingStep>("welcome");
  theme = $state<ThemeKey>("dark");
  protection = $state<Protection>("touchid");
  lang = $state<Lang>("en");
  accounts = $state<Account[]>([]);
  settings = $state<Settings>({ autoLock: 5, hideAfter: 10 });
  query = $state("");
  toast = $state("");

  sheet = $state<null | "add" | "import">(null);

  revealId = $state<string | null>(null);
  revealUntil = $state(0);

  now = $state(Math.floor(Date.now() / 1000));

  t = $derived(translator(this.lang));

  private toastTimer: ReturnType<typeof setTimeout> | null = null;
  private started = false;

  start() {
    if (this.started) return;
    this.started = true;

    const p = loadPrefs();
    this.lang = p.lang ?? detectLang();
    this.theme = p.theme ?? "dark";
    this.protection = p.protection ?? "touchid";
    if (p.settings) this.settings = p.settings;
    applyTheme(this.theme);

    if (p.onboardingDone) {
      this.screen = this.protection ? "lock" : "accounts";
    }

    setInterval(() => {
      this.now = Math.floor(Date.now() / 1000);
      if (this.revealId && Date.now() > this.revealUntil) this.revealId = null;
    }, 1000);

    this.loadAccounts();
  }

  private async loadAccounts() {
    try {
      this.accounts = await vaultLoad();
    } catch (e) {
      console.error("vault_load falló:", e);
    }
  }

  private persist() {
    vaultSave(this.accounts).catch((e) => console.error("vault_save falló:", e));
  }

  private savePrefs(onboardingDone = true) {
    const prefs: Prefs = {
      theme: this.theme,
      lang: this.lang,
      protection: this.protection,
      settings: this.settings,
      onboardingDone,
    };
    try {
      localStorage.setItem(PREFS_KEY, JSON.stringify(prefs));
    } catch {

    }
  }

  setTheme(key: ThemeKey) {
    this.theme = key;
    applyTheme(key);
    this.savePrefs(this.screen !== "onboarding");
  }

  setProtection(p: Protection) {
    this.protection = p;
    this.savePrefs(this.screen !== "onboarding");
  }

  setLang(lang: Lang) {
    this.lang = lang;
    this.savePrefs(this.screen !== "onboarding");
  }

  updateSettings(patch: Partial<Settings>) {
    this.settings = { ...this.settings, ...patch };
    this.savePrefs(this.screen !== "onboarding");
  }

  get stepIndex() {
    return ONBOARDING_STEPS.indexOf(this.onboardingStep);
  }
  nextStep() {
    const i = this.stepIndex;
    if (i < ONBOARDING_STEPS.length - 1) {
      this.onboardingStep = ONBOARDING_STEPS[i + 1];
    } else {
      this.finishOnboarding();
    }
  }
  prevStep() {
    const i = this.stepIndex;
    if (i > 0) this.onboardingStep = ONBOARDING_STEPS[i - 1];
  }
  finishOnboarding() {
    this.savePrefs(true);
    this.screen = "accounts";
  }
  replayTutorial() {
    this.onboardingStep = "welcome";
    this.screen = "onboarding";
  }

  lock() {
    this.revealId = null;
    this.screen = "lock";
  }
  unlock() {
    this.screen = "accounts";
  }

  go(screen: Screen) {
    this.screen = screen;
  }

  addAccount(a: Omit<Account, "id">) {
    const account: Account = { ...a, id: "a" + Date.now() };
    this.accounts = [...this.accounts, account];
    this.persist();
    this.showToast(this.t("toastAdded"));
    return account;
  }
  removeAccount(id: string) {
    this.accounts = this.accounts.filter((a) => a.id !== id);
    this.persist();
  }

  importAccount(acc: Account) {
    this.accounts = [...this.accounts, acc];
    this.persist();
    this.showToast(this.t("toastAdded"));
    this.closeSheet();
  }

  reveal(id: string) {
    this.revealId = id;
    this.revealUntil = Date.now() + this.settings.hideAfter * 1000;
  }

  openAdd() {
    this.sheet = "add";
  }
  openImport() {
    this.sheet = "import";
  }
  closeSheet() {
    this.sheet = null;
  }

  showToast(msg: string) {
    this.toast = msg;
    if (this.toastTimer) clearTimeout(this.toastTimer);
    this.toastTimer = setTimeout(() => (this.toast = ""), 2200);
  }
}

export const app = new AppState();
