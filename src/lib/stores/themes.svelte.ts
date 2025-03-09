import type { Theme, ThemeListItem } from "$lib/types/theme";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";
import settingsMenuStore from "$lib/stores/settings-menu.store";

class ThemesStore {
  themes_list: ThemeListItem[] = $state([]);
  #current_theme: Theme | null = $state(null);
  #original_theme: Theme | null = $state(null);
  #original_theme_name: string | null = $state(null);

  load_themes() {
    invoke("exec_command", { cmd: "get_loaded_themes" });
  }

  save_original_theme() {
    if (this.#current_theme) {
      this.#original_theme = this.#current_theme;
    }
  }

  async initThemesStore() {
    invoke("exec_command", { cmd: "get_current_theme" });
  }

  set_current_theme(theme: Theme) {
    this.#current_theme = theme;
    applyTheme(theme);
  }

  update_themes_list(themes_list: ThemeListItem[]) {
    this.themes_list = themes_list;
  }

  changeTheme(theme: string) {
    // invoke("exec_command", {
    //   cmd: "set_theme",
    //   payload: JSON.stringify(theme),
    // });
    this.#original_theme = this.#current_theme;
    settingsMenuStore.toggleSettingsMenu();
  }

  previewTheme(theme: string) {
    this.#original_theme = this.#current_theme;
    invoke("exec_command", {
      cmd: "set_theme",
      payload: JSON.stringify(theme),
    });
  }

  // TODO: We need to also update the reset of theme
  // on the back end, I was thinking of some function called
  // reset theme that can be invoked on backend to reset the
  // theme on backend too.
  resetTheme() {
    if (this.#original_theme) {
      this.#current_theme = this.#original_theme;
      this.set_current_theme(this.#original_theme);
      invoke("exec_command", {
        cmd: "reset_theme",
        payload: JSON.stringify(this.#original_theme),
      });
    }
  }
}

export const themes_store = new ThemesStore();

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
  root.style.setProperty(`--theme-name`, theme.info.name);
  root.style.setProperty(`--theme-colorscheme`, theme.info.colorscheme);
};
