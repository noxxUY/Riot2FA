export type ThemeKey = "light" | "dark" | "red" | "graphite";

export interface Theme {
  key: ThemeKey;
  bg: string;
  chrome: string;
  surface: string;
  surface2: string;
  text: string;
  muted: string;
  border: string;
  accent: string;
  onAccent: string;
  accentSoft: string;
  accentSoftText: string;
  error: string;

  scheme: "light" | "dark";
}

export const THEMES: Record<ThemeKey, Theme> = {
  light: {
    key: "light",
    bg: "#F5F5F7", chrome: "#ECECEF", surface: "#FFFFFF", surface2: "#EFEFF2",
    text: "#1D1D1F", muted: "#6B6B70", border: "#E0E0E5",
    accent: "#D42A35", onAccent: "#FFFFFF", accentSoft: "#FBE4E6", accentSoftText: "#A51F29",
    error: "#B3202B", scheme: "light",
  },
  dark: {
    key: "dark",
    bg: "#141416", chrome: "#1A1A1D", surface: "#1E1E21", surface2: "#29292D",
    text: "#F5F5F7", muted: "#A1A1A8", border: "#2F2F34",
    accent: "#FF4E58", onAccent: "#1A0507", accentSoft: "#3B1B1F", accentSoftText: "#FF9EA4",
    error: "#FF9EA4", scheme: "dark",
  },
  red: {
    key: "red",
    bg: "#2A0A0F", chrome: "#320D13", surface: "#3A1118", surface2: "#4A1720",
    text: "#FFF1F2", muted: "#EDB8BE", border: "#5A212A",
    accent: "#FF5C66", onAccent: "#2A0A0F", accentSoft: "#5C1C26", accentSoftText: "#FFC4C9",
    error: "#FFC4C9", scheme: "dark",
  },
  graphite: {
    key: "graphite",
    bg: "#2A2C30", chrome: "#2F3236", surface: "#33363B", surface2: "#3D4046",
    text: "#ECEDEF", muted: "#B0B3B9", border: "#46494F",
    accent: "#EF5A62", onAccent: "#1E0A0C", accentSoft: "#4A3537", accentSoftText: "#FFB5BA",
    error: "#FFB5BA", scheme: "dark",
  },
};

export const THEME_ORDER: ThemeKey[] = ["light", "dark", "red", "graphite"];

export function applyTheme(key: ThemeKey) {
  const t = THEMES[key] ?? THEMES.dark;
  const root = document.documentElement;
  for (const [name, value] of Object.entries(t)) {
    if (name === "key" || name === "scheme") continue;
    root.style.setProperty(`--${name}`, value as string);
  }
  root.style.colorScheme = t.scheme;
}
