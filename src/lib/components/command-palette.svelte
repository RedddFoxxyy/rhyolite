<script lang="ts">
	import { commandPaletteStore } from "$lib/stores/commandPalette.svelte";
	import DocumentService from "$lib/services/document.service";
	import TabService from "$lib/services/tab.service";
	import { onMount } from "svelte";
	import { contentEditorStore } from "$lib/stores/contentEditor.svelte";
	import { listen } from "@tauri-apps/api/event";
	import type { Tab } from "$lib/types/tab";

	let selectedIndex: number = $state(-1);
	let searchText: string = $state("");
	let currentTabId: string | null = $state(null);

	interface Command {
		name: string;
		shortcut: string;
		action: () => void;
	}

	onMount(() => {
		const currentTablisten = listen<Tab>("Current_Tab", (event) => {
			currentTabId = event.payload.id;
		});
		return () => {
			currentTablisten.then((unsub) => unsub());
		};
	});

	let commands: Command[] = [
		{
			name: "Delete Tab",
			shortcut: "Ctrl + D",
			action: () => {
				if (currentTabId) {
					DocumentService.deleteDocumentTab(currentTabId);
				}
				commandPaletteStore.toggleVisibility();
			}
		},
		{
			name: "Close Tab",
			shortcut: "Ctrl + C",
			action: () => {
				TabService.closeTab();
				commandPaletteStore.toggleVisibility();
			}
		},
		{
			name: "New Tab",
			shortcut: "Ctrl + N",
			action: () => {
				DocumentService.addNewDocumentTab();
				commandPaletteStore.toggleVisibility();
			}
		},
		{
			name: "Next Tab",
			shortcut: "Ctrl + Tab or Ctrl + pgDown",
			action: () => {
				TabService.cycleTabs();
				commandPaletteStore.toggleVisibility();
			}
		},
		{
			name: "Go to First Tab",
			shortcut: "Ctrl + 1",
			action: () => {
				TabService.gotoTab1();
				commandPaletteStore.toggleVisibility();
			}
		},
		{
			name: "Go to Last Tab",
			shortcut: "Ctrl + 9",
			action: () => {
				TabService.gotoLastTab();
				commandPaletteStore.toggleVisibility();
			}
		},
		// TODO: Add this command after toolbar has been implemented.
		// {
		// 	name: "Toggle ToolBar",
		// 	shortcut: "Ctrl + T",
		// 	action: () => {
		// 		contentEditorStore.toggleToolbarVisibility();
		// 		commandPaletteStore.toggleVisibility();
		// 	}
		// }
	];

	function handleKeydown(event: KeyboardEvent) {
		if (!commandPaletteStore.isVisible()) return;
		console.log(event.key, event.shiftKey, event);
		if (event.key === "ArrowDown" || (event.code === "Tab" && !event.shiftKey)) {
			event.preventDefault();
			if (selectedIndex === -1) {
				selectedIndex = 0;
			} else {
				selectedIndex = (selectedIndex + 1) % commands.length;
			}
		}
		if (event.key === "ArrowUp" || (event.code === "Tab" && event.shiftKey)) {
			event.preventDefault();
			if (selectedIndex === -1) {
				selectedIndex = commands.length - 1;
			} else {
				selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
			}
		}
		if (event.key === "Enter") {
			event.preventDefault();
			if (selectedIndex >= 0 && selectedIndex < commands.length) {
				commands[selectedIndex].action();
			}
		}
		if (event.key === "Escape") {
			event.preventDefault();
			commandPaletteStore.toggleVisibility();
		}
	}
	$effect(() => {
		if (selectedIndex !== -1)
			document
				.querySelector(`#command-palette-options > button:nth-child(${selectedIndex + 1})`)
				?.scrollIntoView({ behavior: "smooth", block: "nearest" });
	});

	function handleWheel(event: WheelEvent) {
		if (!commandPaletteStore.isVisible()) return;

		event.preventDefault();
		if (event.deltaY > 0) {
			// Scrolling down
			selectedIndex = (selectedIndex + 1) % commands.length;
		} else {
			// Scrolling up
			selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
		}
	}

	// Reset selected index when command palette is closed
	$effect(() => {
		if (!commandPaletteStore.isVisible()) {
			selectedIndex = -1;
			searchText = "";
		}
	});

	$effect(() => {
		if (commandPaletteStore.isVisible()) {
			(document.querySelector("#commandPaletteTextarea") as HTMLTextAreaElement).focus();
		}
	});
</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->
{#if commandPaletteStore.isVisible()}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="fixed top-0 left-0 w-full h-full bg-black/50 z-20"
		tabindex="-1"
		aria-modal="true"
		role="dialog"
		onclick={(e) => {
			if (e.target === e.currentTarget) commandPaletteStore.toggleVisibility();
		}}
	>
		<div
			class="fixed top-[40%] left-1/2 flex flex-col bg-crust rounded-lg p-3 z-[60] w-min-[200px] w-[50%] h-fit min-h-[100px] max-h-[400px] -translate-x-1/2 -translate-y-1/2 overflow-hidden"
		>
			<div
				class="relative basis-[42px] w-full shrink-0 overflow-hidden shadow-none hover:shadow-xl focus:shadow-xl transition duration-300 rounded-lg"
			>
				<textarea
					id="commandPaletteTextarea"
					class="w-full h-full overflow-hidden resize-none p-2 cursor-text text-text bg-crust text-left box-border border-2 hover:border-subtext0 rounded-lg transition-all duration-200 border-overlay0 focus:border-subtext0 focus:outline-none focus:ring-0"
					placeholder="Select a Command"
					bind:value={searchText}
					onkeydown={handleKeydown}
				></textarea>
				<button
					class="absolute right-4 top-1/2 -translate-y-1/2 bg-transparent text-text opacity-70 hover:opacity-100 transition-opacity duration-200"
					onclick={() => commandPaletteStore.toggleVisibility()}>✕</button
				>
			</div>
			<div id="command-palette-options" class="flex overflow-y-auto flex-col pt-3">
				<div>
					{#each commands as command, index}
						<button
							type="button"
							class="flex px-3 justify-between items-center hover:bg-surface0 cursor-pointer w-full h-[34px] text-left text-text border-none shadow-none rounded-lg transition-colors duration-200"
							class:bg-surface0={selectedIndex === index}
							onclick={() => {
								command.action();
							}}
							onmouseenter={() => (selectedIndex = index)}
						>
							<span>{command.name}</span>
							<span class="shortcut">{command.shortcut}</span>
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
{/if}
