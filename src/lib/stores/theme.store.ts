import { type Writable, writable, get } from "svelte/store";
import type { Theme } from "../types/theme";
import { defaultTheme } from "$lib/themes/default";
import { greenScreenTheme } from "$lib/themes/greenscreen";
import { coffeeTheme } from "$lib/themes/coffee";
import { catppuccinMacchiatoTheme } from "$lib/themes/catppuccinMacchiato";
import { catppuccinMochaTheme } from "$lib/themes/catppuccinMocha";
import { catppuccinLatteTheme } from "$lib/themes/catppuccinLatte";
import { rosePaneMainTheme } from "$lib/themes/rosePineMain";
import { rosePaneMoonTheme } from "$lib/themes/rosePineMoon";
import { rosePineDawnTheme } from "$lib/themes/rosePineDawn";
import { CrimsonNocturne } from "$lib/themes/CrimsonNocturne";
export interface IThemesStates {
  themes: Theme[];
  currentTheme: Theme;
}

const DEFAULT_THEMES: Theme[] = [
  defaultTheme,
  CrimsonNocturne,
  greenScreenTheme,
  coffeeTheme,
  catppuccinMacchiatoTheme,
  catppuccinMochaTheme,
  catppuccinLatteTheme,
  rosePaneMainTheme,
  rosePaneMoonTheme,
  rosePineDawnTheme,
];

const states: Writable<IThemesStates> = writable<IThemesStates>({
  themes: DEFAULT_THEMES,
  currentTheme: DEFAULT_THEMES[0],
});

const initThemesStore = async () => {
  // TODO: pick from some store
  const themes = DEFAULT_THEMES;
  // TODO: use last used theme or from browser colorscheme
  const currentTheme = themes[0];
  applyTheme(currentTheme);
  states.update(() => ({ themes, currentTheme }));
};

const resetCurrentTheme = () => {
  const themes: Theme[] = getThemesState();
  const currentTheme: Theme = themes[0];
  updateCurrentThemeState(currentTheme);
};

const updateThemesState = (themes: Theme[]): Theme[] => {
  states.update((data) => ({
    themes: themes,
    currentTheme: data.currentTheme,
  }));
  return themes;
};

const colorToRgb = (color: string) => {
  let match = /^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$/i.exec(color);
  if (match) {
    return match.slice(1).map((hex) => parseInt(hex, 16));
  }
  match = /^#([a-f0-9])([a-f0-9])([a-f0-9])$/i.exec(color);
  if (match) {
    return match.slice(1).map((hex) => parseInt(hex + hex, 16));
  }
  match =
    /^rgb\(\s*(\d+)(?:\s*,\s*|\s+)(\d+)(?:\s*,\s*|\s+)(\d+)\s*\)\s*$/i.exec(
      color,
    );
  if (match) {
    return match.slice(1).map((num) => parseInt(num));
  }
  throw new Error(`Unsupported color: "${color}"`);
};

const applyTheme = (theme: Theme) => {
  const root: HTMLHtmlElement = document.querySelector(":root")!;
  Object.entries(theme.colors).forEach(([name, value]) => {
    root.style.setProperty(`--color-${name}`, colorToRgb(value).join(" "));
  });
  root.style.setProperty(`--theme-name`, theme.name);
  root.style.setProperty(`--theme-colorscheme`, theme.colorscheme);
};

const updateCurrentThemeState = (currentTheme: Theme): Theme => {
  states.update((data) => ({
    themes: data.themes,
    currentTheme: currentTheme,
  }));
  applyTheme(currentTheme);
  return currentTheme;
};

const getThemeById = (themeName: string): Theme | undefined => {
  const { themes }: { themes: Theme[] } = get(states); // Access the current state of states
  return themes.find((theme) => theme.name === themeName); // Replace 'id' with the actual property name if it's different
};

const getCurrentThemeState = (): Theme => {
  const { currentTheme }: { currentTheme: Theme } = get(states);
  return currentTheme;
};

const getThemesState = (): Theme[] => {
  const { themes }: { themes: Theme[] } = get(states);
  return themes;
};

export default {
  states,
  initThemesStore,
  resetCurrentTheme,
  updateThemesState,
  updateCurrentThemeState,
  getThemeById,
  getCurrentThemeState,
  getThemesState,
};
