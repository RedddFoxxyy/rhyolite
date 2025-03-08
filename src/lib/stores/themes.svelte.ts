import type { Theme } from "$lib/types/theme";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";
import themeStore from "./theme.store";

class ThemesStore {
  #themes_list: string[] = $state([]);
  #current_theme: Theme | null = $state(null);
  #original_theme: Theme | null = $state(null);

  save_original_theme() {
    if (this.#current_theme) {
      this.#original_theme = this.#current_theme;
    }
  }

  async initThemesStore() {}

  set_current_theme(theme: Theme) {
    this.#current_theme = theme;
    applyTheme(theme);
  }

  update_themes_list(themes_list: string[]) {
    this.#themes_list = themes_list;
  }
}

const themes_store = new ThemesStore();

onMount(() => {
  const currentThemelisten = listen<Theme>("update_current_theme", (event) => {
    themes_store.set_current_theme(event.payload);
    // originalTheme = event.payload;
  });
  const themeListlisten = listen<string[]>("themes_list", (event) => {
    themes_store.update_themes_list(event.payload);
  });
  return () => {
    currentThemelisten.then((unsub) => unsub());
    themeListlisten.then((unsub) => unsub());
  };
});

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
