<script lang="ts">
  import Close from "$lib/static/close.svg.svelte";
  import Restore from "$lib/static/restore.svg.svelte";
  import Maximise from "$lib/static/maximise.svg.svelte";
  import Minimise from "$lib/static/minimise.svg.svelte";
  import { getCurrentWindow, Window } from "@tauri-apps/api/window";
  import { onDestroy } from "svelte";
  import TabsStore from "../stores/tabs.store";
  import { type Tab } from "../types/tab";
  import { addNewDocumentTab } from "../services/document.service";
  import closeTab from "../services/tab.service";
  import ThemeStore from "../stores/theme.store";
  import type { Theme } from "../types/theme";
  import { Select } from "flowbite-svelte";

  let tabs: Tab[] = $state([]);
  let currentTab: Tab | null = $state(null);

  let isMaximized: boolean = $state(false);

  const appWindow = getCurrentWindow();

  // Initial state
  // getCurrentWindow()
  //   .isMaximized()
  //   .then((result) => {
  //     isMaximized = result;
  //   });

  // Listen for maximize and unmaximize events
  appWindow.listen("tauri://resize", async () => {
    isMaximized = await appWindow.isMaximized();
  });

  const unsubscribeTabsState = TabsStore.states.subscribe((value) => {
    tabs = value.tabs;
    currentTab = value.currentTab;
  });

  onDestroy(() => {
    unsubscribeTabsState();
  }); // Clean up
  $effect(() => {
    if (currentTab) {
      document
        .querySelector(
          currentTab.id === tabs[tabs.length - 1].id
            ? "#tablist>#new-tab-btn"
            : "#tablist>.active",
        )
        ?.scrollIntoView({
          behavior: "smooth",
          block: "nearest",
          inline: "nearest",
        });
    }
  });

  const onOpenTab = (tab: Tab) => {
    TabsStore.updateCurrentTabState(tab);
  };
</script>

<div
  data-tauri-drag-region
  class="flex grow-0 shrink-0 bg-base w-full basis-[40px] select-none justify-between items-center overflow-hidden"
>
  <div
    class="flex items-center h-full ml-7 px-4 flex-shrink-1 flex-grow-0 overflow-y-hidden overflow-x-auto gap-1"
    role="tablist"
    id="tablist"
    aria-label="Document tabs"
  >
    {#each tabs as tab}
      <button
        class="flex justify-left items-center px-4 h-[30px] w-48 rounded-[18px] flex-shrink text-text hover:bg-surface1"
        class:bg-surface0={currentTab?.id === tab.id}
        class:active={currentTab?.id === tab.id}
        role="tab"
        aria-controls="editor"
        onclick={() => onOpenTab(tab)}
      >
        <!-- truncate needs to be in a separate span to work properly somehow -->
        <span class="truncate text-clip">
          { tab.title || "Untitled" }
        </span>
      </button>
    {/each}

    <button
      type="button"
      class="flex justify-center items-center px-4 text-nowrap h-[30px] w-[30px] aspect-square rounded-[18px] flex-shrink text-text hover:bg-surface1"
      id="new-tab-btn"
      onclick={addNewDocumentTab}>+</button
    >
  </div>
  <div class="flex-grow"></div>
  <div class="flex flex-row items-stretch self-stretch flex-shrink-0">
    <button
      class="flex justify-center items-center w-12 mx-auto cursor-pointer focus-visible:bg-surface2 hover:bg-surface2"
      id="titlebar-minimize"
      onclick={() => appWindow.minimize()}
      aria-label="Minimize"
    >
      <Minimise />
    </button>
    <button
      class="flex justify-center items-center w-12 mx-auto cursor-pointer focus-visible:bg-surface2 hover:bg-surface2"
      id="titlebar-maximize"
      onclick={() => appWindow.toggleMaximize()}
      aria-label="Maximise"
    >
      {#if isMaximized}
        <Restore />
      {:else}
        <Maximise />
      {/if}
    </button>
    <button
      class="flex justify-center items-center w-12 mx-auto cursor-pointer focus-visible:bg-red-500 hover:bg-red-500"
      id="titlebar-close"
      onclick={() => appWindow.close()}
      aria-label="Close"
    >
      <Close />
    </button>
  </div>
</div>
