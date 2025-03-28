import type { Theme, ThemeListItem } from "$lib/types/theme";
import { invoke } from "@tauri-apps/api/core";
import { settingsMenuStore } from "$lib/stores/settingsMenu.svelte";

class ThemesStore {
	themesList: ThemeListItem[] = $state([]);

	#currentTheme: Theme | null = $state(null);
	// originalTheme cannot be made private through the # prefix because otherwise javascript
	// will emit an error since the value is accessed through the rust side
	private originalTheme: Theme | null = $state(null);
	#originalThemeName: string | null = $state(null);

	loadThemes() {
		invoke("exec_command", { cmd: "get_loaded_themes" });
	}

	saveOriginalTheme() {
		if (this.#currentTheme) {
			this.originalTheme = this.#currentTheme;
		}
	}

	async initThemesStore() {
		invoke("exec_command", { cmd: "get_current_theme" });
	}

	setCurrentTheme(theme: Theme) {
		this.#currentTheme = theme;
		applyTheme(theme);
	}

	updateThemesList(themesList: ThemeListItem[]) {
		this.themesList = themesList;
	}

	changeTheme(theme: string) {
		// invoke("exec_command", {
		//   cmd: "set_theme",
		//   payload: JSON.stringify(theme),
		// });
		this.originalTheme = this.#currentTheme;
		settingsMenuStore.toggleVisibility();
	}

	previewTheme(theme: string) {
		this.originalTheme = this.#currentTheme;
		invoke("exec_command", {
			cmd: "set_theme",
			payload: JSON.stringify(theme)
		});
	}

	// TODO: We need to also update the reset of theme
	// on the back end, I was thinking of some function called
	// reset theme that can be invoked on backend to reset the
	// theme on backend too.
	resetTheme() {
		if (this.originalTheme) {
			this.#currentTheme = this.originalTheme;
			this.setCurrentTheme(this.originalTheme);
			invoke("exec_command", {
				cmd: "reset_theme",
				payload: JSON.stringify(this.originalTheme)
			});
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
