export type ThemeType = "Basic" | "Advance";

export type ColorScheme = "Light" | "Dark";

export interface ThemeInfo {
	name: string;
	author: string;
	themetype: ThemeType;
	colorscheme: ColorScheme;
}

export type Colors = {
	text: string;
	subtext2: string;
	subtext1: string;
	subtext0: string;
	overlay2: string;
	overlay1: string;
	overlay0: string;
	surface2: string;
	surface1: string;
	surface0: string;
	base: string;
	crust: string;
	mantle: string;
	accent: string;
	highlight: string;
	border: string;
};

export type Theme = {
	info: ThemeInfo;
	colors: Colors;
};

export interface ThemeListItem {
	filename: string; // File stem (e.g., "dark")
	name: string; // Display name from TOML (e.g., "My Dark Theme")
}
