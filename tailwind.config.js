import flowbitePlugin from "flowbite/plugin";
import flowbiteTypographyPlugin from "flowbite-typography";

/** @type {import('tailwindcss').Config} */
export default {
	content: [
		"./src/**/*.{html,js,svelte,ts}",
		"./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}"
	],
	darkMode: "selector",
	theme: {
		extend: {
			colors: {
				text: "rgb(var(--color-text))",
				subtext2: "rgb(var(--color-subtext2))",
				subtext1: "rgb(var(--color-subtext1))",
				subtext0: "rgb(var(--color-subtext0))",
				overlay2: "rgb(var(--color-overlay2))",
				overlay1: "rgb(var(--color-overlay1))",
				overlay0: "rgb(var(--color-overlay0))",
				surface2: "rgb(var(--color-surface2))",
				surface1: "rgb(var(--color-surface1))",
				surface0: "rgb(var(--color-surface0))",
				base: "rgb(var(--color-base))",
				crust: "rgb(var(--color-crust))",
				mantle: "rgb(var(--color-mantle))"
			}
		}
	},
	plugins: [
		flowbitePlugin,
		flowbiteTypographyPlugin({
			wysiwyg: true
		})
	]
};
