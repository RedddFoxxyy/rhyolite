import documentCmds from "$lib/tauri-cmd/document";
import tabCmds from "$lib/tauri-cmd/tab";
import { commandPaletteStore } from "$lib/stores/commandPalette.svelte";
import { contentEditorStore } from "$lib/stores/contentEditor.svelte";
import { onMount } from "svelte";
import { listen } from "@tauri-apps/api/event";
import type { Tab } from "$lib/types/tab";

let activeKeys = new Set();
let currentTabId: string | null = $state(null);

export function attachHotkeyListener() {
	onMount(() => {
		const currentTablisten = listen<Tab>("Current_Tab", (event) => {
			currentTabId = event.payload.id;
		});
		return () => {
			currentTablisten.then((unsub) => unsub());
		};
	});

	document.addEventListener("keyup", handleKeyup);
	document.addEventListener("keydown", handleKeydown);
}

function handleKeyup(event: KeyboardEvent) {
	// remove the keys from the activeKeys set
	activeKeys.delete(event.key);
}

function handleKeydown(event: KeyboardEvent) {
	// check if key is already in the activeKeys set
	if (!activeKeys.has(event.key)) {
		activeKeys.add(event.key);
	} else return;

	if (event.ctrlKey) {
		event.preventDefault();
		switch (event.key) {
			case "d":
				if (currentTabId) documentCmds.deleteDocumentTab(currentTabId);
				break;

			case "w":
				// const currentTabId = TabsStore.getCurrentTabState()?.id;
				if (currentTabId) tabCmds.closeTab(currentTabId);
				break;

			case "n":
				documentCmds.addNewDocumentTab();
				break;

			case "t":
				contentEditorStore.toggleToolbarVisibility();
				break;

			case "Tab":
			case "PageDown":
				tabCmds.cycleTabs();
				break;

			case "1":
				tabCmds.gotoTab1();
				break;

			case "9":
				tabCmds.gotoLastTab();
				break;

			case "p":
				commandPaletteStore.toggleVisibility();
				break;
		}
	}
}
