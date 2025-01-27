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
  import tabService from "../services/tab.service";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let tabs: Tab[] = $state([]);
  let currentTab: Tab | null = $state(null);

  let isMaximized: boolean = $state(false);

  const appWindow = getCurrentWindow();

  let hoverTabId: string | null = $state(null);

  const onTabClose = async (tabId: string) => {
    await tabService.closeTab(tabId);
  };

  appWindow.listen("tauri://resize", async () => {
    isMaximized = await appWindow.isMaximized();
  });

  // const unsubscribeTabsState = TabsStore.states.subscribe((value) => {
  //   tabs = value.tabs;
  //   currentTab = value.currentTab;
  // });

  onMount(() => {
    // TabsStore.initTabsStore();
    // Listen for the 'counter-updated' event from the backend
    const tabslisten = listen<Tab[]>("Tabs", (event) => {
      tabs = event.payload;
    });
    const currentTablisten = listen<Tab>("Tabs", (event) => {
      // Update the Svelte store with the new counter value
      currentTab = event.payload;
    });
    return () => {
      tabslisten.then((unsub) => unsub());
      currentTablisten.then((unsub) => unsub());
    };
  });

  // onDestroy(() => {
  //   unsubscribeTabsState();
  // }); // Clean up
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
    tabService.switchTab(tab.id);
    //invoke("update_states");
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
      <div class="relative group flex items-center justify-between">
        <button
          class={`flex justify-left items-center pl-4 pr-2 text-nowrap h-[30px] w-fit rounded-[18px] flex-shrink text-text transition-colors duration-100 hover:bg-surface1 ${currentTab?.id === tab.id ? "bg-surface0" : ""}`}
          class:active={currentTab?.id === tab.id}
          role="tab"
          aria-controls="editor"
          onclick={() => onOpenTab(tab)}
          onmouseenter={() => (hoverTabId = tab.id)}
          onmouseleave={() => (hoverTabId = null)}
        >
          {tab.title.length > 20
            ? tab.title.slice(0, 20) + "..."
            : tab.title || "Untitled"}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="text-text bg-transparent ml-2 p-1 rounded-[18px] h-[20px] w-[20px] flex justify-center items-center opacity-0 transition-opacity duration-100 hover:bg-surface2 hover:text-subtext1"
            class:opacity-100={currentTab?.id === tab.id ||
              hoverTabId === tab.id}
            onclick={(e) => {
              e.stopPropagation();
              const tabToCloseId = hoverTabId || tab.id;
              // console.log(`close tab ${tabToCloseId}`);
              onTabClose(tabToCloseId);
            }}
          >
            <Close />
          </div>
        </button>
      </div>
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
