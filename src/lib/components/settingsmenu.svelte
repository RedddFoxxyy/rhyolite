<script lang="ts">
	import { ChevronRight, SlidersHorizontal, Palette, Keyboard, Info } from "lucide-svelte";
	import { onDestroy } from "svelte";
	import { settingsMenuStore } from "$lib/stores/settingsMenu.svelte";
	import { themes_store } from "$lib/stores/themes.svelte";
	import type { Theme, ThemeListItem } from "$lib/types/theme";
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";

	let showThemeOptions = $state(false);
	let self: HTMLElement | null = $state(null);

	onMount(() => {
		const currentThemelisten = listen<Theme>("update_current_theme", (event) => {
			themes_store.set_current_theme(event.payload);
			// originalTheme = event.payload;
		});
		const themeListlisten = listen<ThemeListItem[]>("themes_list", (event) => {
			themes_store.update_themes_list(event.payload);
		});
		return () => {
			currentThemelisten.then((unsub) => unsub());
			themeListlisten.then((unsub) => unsub());
		};
	});

	const layout = {
		position: { top: 150, left: 46, bottom: 10 },
		dimensions: { width: 200, height: 200 }
	};

	const menuButtons = [
		{
			label: "General Settings",
			icon: SlidersHorizontal,
			onClick: () => console.log("Opening General Settings...")
		},
		{
			label: "Theme",
			icon: Palette,
			onClick: () => (showThemeOptions = !showThemeOptions),
			hasSubmenu: true
		},
		{
			label: "Keyboard Shortcuts",
			icon: Keyboard,
			onClick: () => console.log("Opening Keyboard Shortcuts...")
		},
		{
			label: "About",
			icon: Info,
			onClick: () => console.log("Opening About...")
		}
	];

	const handleCloseEvent = (e: MouseEvent | KeyboardEvent) => {
		if (
			(e instanceof MouseEvent && !self?.contains(e.target as Node)) ||
			(e instanceof KeyboardEvent && e.key === "Escape")
		) {
			e.stopPropagation();
			settingsMenuStore.toggleVisibility();
		}
	};

	$effect(() => {
		if (settingsMenuStore.isVisible()) {
			document.addEventListener("click", handleCloseEvent);
			document.addEventListener("keydown", handleCloseEvent);
			themes_store.load_themes();
		} else {
			document.removeEventListener("click", handleCloseEvent);
			document.removeEventListener("keydown", handleCloseEvent);
			showThemeOptions = false;
			themes_store.resetTheme(); // Reset to original theme when closing without selecting
		}
	});

	onDestroy(() => {
		document.removeEventListener("click", handleCloseEvent);
		document.removeEventListener("keydown", handleCloseEvent);
		themes_store.resetTheme(); // Ensure theme is reset if component is destroyed while previewing
	});
</script>

{#if settingsMenuStore.isVisible()}
	<div
		bind:this={self}
		class="absolute rounded-lg p-1 z-50 transition-all duration-300 transform bg-base shadow-xl"
		class:translate-y-0={settingsMenuStore.isVisible()}
		class:opacity-100={settingsMenuStore.isVisible()}
		class:translate-y-5={!settingsMenuStore.isVisible()}
		class:opacity-0={!settingsMenuStore.isVisible()}
		style="bottom: {layout.position.bottom}px; left: {layout.position.left}px; width: {layout
			.dimensions.width}px;"
	>
		{#each menuButtons as { label, icon: Icon, onClick, hasSubmenu }}
			<button
				class="w-full p-1 rounded-lg text-left text-text cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1 flex flex-row justify-between items-center"
				onclick={onClick}
			>
				<div class="flex flex-row gap-1.5 items-center">
					<Icon class="w-4 h-4" />
					{label}
				</div>
				{#if hasSubmenu}
					<ChevronRight class="w-4 h-4" />
				{/if}
			</button>
		{/each}

		{#if showThemeOptions}
			<div
				role="menu"
				tabindex="0"
				class="absolute left-full rounded-lg p-1 bottom-[50%] mt-8 ml-1 w-max bg-base shadow-xl"
				style="width: {layout.dimensions.width}px;"
				onmouseleave={themes_store.resetTheme}
			>
				{#each themes_store.themes_list as theme_list_item}
					<button
						class="w-full p-1 rounded-lg text-left text-text bg-transparent cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1"
						onmouseenter={() => themes_store.previewTheme(theme_list_item.filename)}
						onclick={() => themes_store.changeTheme(theme_list_item.filename)}
					>
						{theme_list_item.name}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}
