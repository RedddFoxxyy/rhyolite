<script lang="ts">
	import documentCmds from "$lib/tauri-cmd/document";
	import tabCmds from "$lib/tauri-cmd/tab";
	import { commandPaletteStore } from "$lib/stores/commandPalette.svelte";
	import { contentEditorStore } from "$lib/stores/contentEditor.svelte";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import type { Tab } from "$lib/types/tab";

	let activeKeys = new Set();
	let currentTabId: string | null = $state(null);

	onMount(() => {
		const currentTablisten = listen<Tab>("Current_Tab", (event) => {
			currentTabId = event.payload.id;
		});
		return () => {
			currentTablisten.then((unsub) => unsub());
		};
	});

	const handleKeyup = (event: KeyboardEvent): void => {
		// remove the keys from the activeKeys set
		activeKeys.delete(event.key);
	};
	const handleKeydown = (event: KeyboardEvent): void => {
		// check if key is already in the activeKeys set
		if (!activeKeys.has(event.key)) {
			activeKeys.add(event.key);
		} else return;
		if (event.ctrlKey && event.key === "d") {
			event.preventDefault();
			if (currentTabId) {
				documentCmds.deleteDocumentTab(currentTabId);
			}
		}
		if (event.ctrlKey && event.key === "c") {
			event.preventDefault();
			// const currentTabId = TabsStore.getCurrentTabState()?.id;
			if (currentTabId) {
				tabCmds.closeTab(currentTabId);
			}
		}
		if (event.ctrlKey && event.key === "n") {
			event.preventDefault();
			documentCmds.addNewDocumentTab();
		}
		if (event.ctrlKey && event.key === "t") {
			event.preventDefault();
			contentEditorStore.toggleToolbarVisibility();
		}
		if ((event.ctrlKey && event.key === "Tab") || (event.ctrlKey && event.key === "PageDown")) {
			event.preventDefault();
			tabCmds.cycleTabs();
		}
		if (event.ctrlKey && event.altKey && event.key === "c") {
			event.preventDefault();
			tabCmds.closeTab();
		}
		if (event.ctrlKey && event.key === "1") {
			event.preventDefault();
			tabCmds.gotoTab1();
		}
		if (event.ctrlKey && event.key === "9") {
			event.preventDefault();
			tabCmds.gotoLastTab();
		}
		if (event.ctrlKey && event.key === "p") {
			event.preventDefault();
			commandPaletteStore.toggleVisibility();
		}
	};
</script>

<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />
