<script lang="ts">
	import CommandIcon from "$lib/assets/command_palette.svg.svelte";
	import RecentFilesIcon from "$lib/assets/recent_files.svg.svelte";
	import { commandPaletteStore } from "$lib/stores/commandPalette.svelte";
	import { recentFilesStore } from "$lib/stores/recentFiles.svelte";
	import SettingsIcon from "$lib/assets/settings.svg.svelte";
	import { settingsMenuStore } from "$lib/stores/settingsMenu.svelte";
	import SettingsMenu from "./settingsdropdown.svelte";
</script>

<main class="h-full basis-[40px] grow-0 shrink-0 border-r-2 border-r-surface0">
	<div class="flex flex-col justify-between h-full">
		<div class="flex flex-col py-[10px] gap-2 px-2">
			<button
				class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-gray-200/10 transition-all duration-200 focus:outline-none focus:ring-0"
				id="CommandPalette_button"
				onclick={(e) => {
					e.stopPropagation();
					commandPaletteStore.toggleVisibility();
				}}
				aria-label="Open Command Palette"
				title="Open Command Palette"
			>
				<CommandIcon />
			</button>
			<button
				class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0"
				id="Files_Menu_button"
				onclick={() => recentFilesStore.toggleVisibility()}
				aria-label="Open Files Menu"
				title="Open Files Menu"
			>
				<RecentFilesIcon />
			</button>
		</div>
		<div class="gap-2 pb-2 px-2">
			<button
				class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0"
				id="Settings_button"
				onclick={(e) => {
					e.stopPropagation();
					settingsMenuStore.toggleVisibility();
				}}
				aria-label="Open Settings Menu"
				title="Open Settings Menu"
			>
				<SettingsIcon />
			</button>
			<SettingsMenu />
		</div>
	</div>
</main>
