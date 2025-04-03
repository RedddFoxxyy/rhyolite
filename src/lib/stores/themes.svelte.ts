import type { Theme, ThemeListItem } from "$lib/types/theme";
import { invoke } from "@tauri-apps/api/core";
import { settingsMenuStore } from "$lib/stores/settingsMenu.svelte";

class ThemesStore {
	themesList: ThemeListItem[] = $state([]);

	#currentTheme: Theme | null = $state(null);
	private originalTheme: Theme | null = $state(null);
	#isPreviewing: boolean = $state(false);

	loadThemes() {
		invoke("exec_command", { cmd: "get_loaded_themes" });
	}

	saveOriginalTheme() {
		if (this.#currentTheme && !this.originalTheme) {
			console.log("Saving original theme:", this.#currentTheme);
			this.originalTheme = JSON.parse(JSON.stringify(this.#currentTheme));
		}
	}

	async initThemesStore() {
		invoke("exec_command", { cmd: "get_current_theme" });
	}

	setCurrentTheme(theme: Theme) {
		console.log("Setting current theme:", theme);
		this.#currentTheme = theme;
		applyTheme(theme);
	}

	updateThemesList(themesList: ThemeListItem[]) {
		this.themesList = themesList;
	}

	changeTheme(theme: string) {
		console.log("Changing theme permanently to:", theme);
		this.#isPreviewing = false;

		invoke("exec_command", {
			cmd: "set_theme",
			payload: JSON.stringify(theme)
		});

		this.originalTheme = null;

		settingsMenuStore.toggleVisibility();
	}

	previewTheme(theme: string) {
		console.log("Previewing theme:", theme);
		this.saveOriginalTheme();
		this.#isPreviewing = true;

		invoke("exec_command", {
			cmd: "set_theme",
			payload: JSON.stringify(theme)
		});
	}

	resetTheme() {
		if (this.#isPreviewing && this.originalTheme) {
			console.log("Resetting to original theme:", this.originalTheme);
			this.setCurrentTheme(this.originalTheme);

			invoke("exec_command", {
				cmd: "set_theme",
				payload: JSON.stringify(this.originalTheme.filename || "")
			});

			this.#isPreviewing = false;
		}
	}
}

export const themesStore = new ThemesStore();

function colorToRgb(color: string) {
	let match = /^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$/i.exec(color);
	if (match) {
		return match.slice(1).map((hex) => parseInt(hex, 16));
	}
	match = /^#([a-f0-9])([a-f0-9])([a-f0-9])$/i.exec(color);
	if (match) {
		return match.slice(1).map((hex) => parseInt(hex + hex, 16));
	}
	match = /^rgb\(\s*(\d+)(?:\s*,\s*|\s+)(\d+)(?:\s*,\s*|\s+)(\d+)\s*\)\s*$/i.exec(color);
	if (match) {
		return match.slice(1).map((num) => parseInt(num));
	}
	throw new Error(`Unsupported color: "${color}"`);
}

function applyTheme(theme: Theme) {
	const root: HTMLHtmlElement = document.querySelector(":root")!;
	Object.entries(theme.colors).forEach(([name, value]) => {
		root.style.setProperty(`--color-${name}`, colorToRgb(value).join(" "));
	});
	root.style.setProperty(`--theme-name`, theme.info.name);
	root.style.setProperty(`--theme-colorscheme`, theme.info.colorscheme);
}
