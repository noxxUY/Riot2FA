export interface Account {
  id: string;

  riotId: string;

  label: string;
  region: string;

  seed: string;
}

export type Screen = "onboarding" | "lock" | "accounts" | "settings";
export type Protection = "touchid" | "password";
export type OnboardingStep = "welcome" | "theme" | "protection" | "tutorial";

export interface Settings {

  autoLock: number;

  hideAfter: number;
}
