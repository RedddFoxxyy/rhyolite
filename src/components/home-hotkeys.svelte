<script lang="ts">
  import DocumentService from "../services/document.service";
  import TabService from "../services/tab.service";
  import CommandPaletteStore from "../stores/command-palette.store";
  import TabsStore from "../stores/tabs.store";
  import ContentEditorStore from "../stores/content-editor.store";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import type { Tab } from "../types/tab";

  let activeKeys = new Set();
  let currentTabId: string | null = $state(null);

  onMount(() => {
    const currentTablisten = listen<Tab>("Current_Tab", (event) => {
      // Update the Svelte store with the new counter value
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
        DocumentService.deleteDocumentTab(currentTabId);
      }
    }
    if (event.ctrlKey && event.key === "c") {
      event.preventDefault();
      // const currentTabId = TabsStore.getCurrentTabState()?.id;
      if (currentTabId) {
        TabService.closeTab(currentTabId);
      }
    }
    if (event.ctrlKey && event.key === "n") {
      event.preventDefault();
      DocumentService.addNewDocumentTab();
    }
    if (event.ctrlKey && event.key === "t") {
      event.preventDefault();
      ContentEditorStore.toggleToolbarVisibility();
    }
    if (
      (event.ctrlKey && event.key === "Tab") ||
      (event.ctrlKey && event.key === "PageDown")
    ) {
      event.preventDefault();
      TabService.cycleTabs();
    }
    if (event.ctrlKey && event.altKey && event.key === "c") {
      event.preventDefault();
      TabService.closeTab();
    }
    if (event.ctrlKey && event.key === "1") {
      event.preventDefault();
      TabService.gotoTab1();
    }
    if (event.ctrlKey && event.key === "9") {
      event.preventDefault();
      TabService.gotoLastTab();
    }
    if (event.ctrlKey && event.key === "p") {
      event.preventDefault();
      CommandPaletteStore.toggleVisibility();
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />
