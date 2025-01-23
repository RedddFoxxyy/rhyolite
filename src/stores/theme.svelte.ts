import type { Theme } from "../types/theme";
import { defaultTheme } from "$lib/themes/default";
import { greenScreenTheme } from "$lib/themes/greenscreen";
import { coffeeTheme } from "$lib/themes/coffee";
import { catppuccinMacchiatoTheme } from "$lib/themes/catppuccinMacchiato";
import { catppuccinMochaTheme } from "$lib/themes/catppuccinMocha";
import { catppuccinLatteTheme } from "$lib/themes/catppuccinLatte";
import { rosePaneMainTheme } from "$lib/themes/rosePineMain";

const DEFAULT_THEMES: Theme[] = [
  defaultTheme,
  greenScreenTheme,
  coffeeTheme,
  catppuccinMacchiatoTheme,
  catppuccinMochaTheme,
  catppuccinLatteTheme,
  rosePaneMainTheme,
];

class ThemeStore {
  // TODO: pick from some store
  #themes = $state(DEFAULT_THEMES);
  // TODO: use last used theme or from browser colorscheme
  #currentTheme = $state(DEFAULT_THEMES[0]);

  constructor() {
    applyTheme(this.#currentTheme);
  }

  resetCurrentTheme() {
    this.#currentTheme = this.#themes[0];
  }

  updateTheme(themes: Theme[]): Theme[] {
    this.#themes = themes;
    return themes;
  }

  updateCurrentTheme(currentTheme: Theme): Theme {
    this.#currentTheme = currentTheme;
    applyTheme(currentTheme);
    return currentTheme;
  }

  getThemeById(themeName: string): Theme | undefined{
    return this.#themes.find((theme) => theme.name === themeName); // Replace 'id' with the actual property name if it's different
  }

  getCurrentTheme(): Theme{
    return this.#currentTheme;
  }

  getThemes(): Theme[] {
    return this.#themes;
  }

}

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

export const themeStore = new ThemeStore();
