<script lang="ts">
  import {recentFilesStore} from "../stores/recent-files.svelte";
  import DocumentService from "../services/document.service";
  import { apiProvider } from "../services/api.service";
  import type { RecentFileInfo } from "../types/document";

  let files: RecentFileInfo[] = $state([]);
  let selectedIndex: number = $state(-1);
  let searchText: string = $state("");

  async function loadFiles() {
    try {
      const docs = await apiProvider.getRecentlyOpenedFiles();
      files = docs.map((doc) => ({
        id: doc.id,
        title: doc.title,
      }));
      // console.log("Loaded files:", files);
    } catch (error) {
      console.error("Failed to load files:", error);
    }
  }

  async function openFile(file: RecentFileInfo) {
    try {
      await apiProvider.loadTab({
        documentId: file.id,
        documentTitle: file.title,
      });
      await DocumentService.getAllDocumentTabs();
      await apiProvider.sendCurrentOpenTab(file.id);
      toggleFilesMenu();
    } catch (error) {
      console.error("Failed to open file:", error);
    }
  }

  function toggleFilesMenu() {
    recentFilesStore.toggleVisibility();
    // loadFiles();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!recentFilesStore.isVisible()) return;

    switch (event.key) {
      case "ArrowDown":
      case "Tab":
        if (!event.shiftKey) {
          event.preventDefault();
          selectedIndex = (selectedIndex + 1) % files.length;
        }
        break;
      case "ArrowUp":
        event.preventDefault();
        selectedIndex = (selectedIndex - 1 + files.length) % files.length;
        break;
      case "Enter":
        event.preventDefault();
        if (selectedIndex >= 0 && selectedIndex < files.length) {
          openFile(files[selectedIndex]);
        }
        break;
      case "Escape":
        event.preventDefault();
        toggleFilesMenu();
        break;
    }
  }

  $effect(() => {
    if (recentFilesStore.isVisible()) {
      loadFiles();
      (
        document.querySelector("#recentFilesTextarea") as HTMLTextAreaElement
      )?.focus();
    } else {
      selectedIndex = -1;
      searchText = "";     
    }
  });
</script>

{#if recentFilesStore.isVisible()}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed top-0 left-0 w-full h-full bg-black/60"
    tabindex="-1"
    aria-modal="true"
    role="dialog"
    onclick={(e) => {
      if (e.target === e.currentTarget) toggleFilesMenu();
    }}
  >
    <div
      class="fixed top-[40%] left-1/2 flex flex-col bg-crust rounded-lg p-3 z-[60] w-min-[200px] w-[50%] h-[40%] gap-2 -translate-x-1/2 -translate-y-1/2 overflow-hidden"
    >
      <div class="relative basis-[42px] w-full shrink-0 mb-2 overflow-hidden">
        <textarea
          id="recentFilesTextarea"
          class="w-full h-full overflow-hidden resize-none pr-8 p-2 cursor-text text-text bg-surface0 text-left box-border border-2 hover:border-subtext0 outline-none rounded transition-all duration-200 border-overlay0 focus:border-subtext0 focus:outline-none focus:ring-0"
          placeholder="Search Recent Files"
          bind:value={searchText}
          onkeydown={handleKeydown}
        ></textarea>
        <button
          class="absolute right-2 top-1/2 -translate-y-1/2 bg-transparent text-text opacity-70 hover:opacity-100 transition-opacity duration-200"
          onclick={() => toggleFilesMenu()}>✕</button
        >
      </div>
      <div class="flex overflow-y-auto flex-col gap-[0.5px]">
        <div class="mx-2">
          {#each files as file, index}
            <button
              type="button"
              class="flex px-4 justify-between items-center p-1 hover:bg-surface0 cursor-pointer w-full h-[34px] text-left text-text border-none shadow-none rounded transition-colors duration-200"
              class:bg-surface0={selectedIndex === index}
              onclick={() => openFile(file)}
              onmouseenter={() => (selectedIndex = index)}
            >
              <span>{file.title || "Untitled"}</span>
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>
{/if}
