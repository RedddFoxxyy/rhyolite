<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import TabsStore from "../stores/tabs.store";
  import tabService from "../services/tab.service";
  import { type Tab, TabType } from "../types/tab";
  import Document from "./document.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";

  let tabs: Tab[] = $state([]);
  let currentTab: Tab | null = $state(null);

  onMount(() => {
    // Listen for the 'Tabs' event from the backend
    const tabslisten = listen<Tab[]>("Tabs", (event) => {
      tabs = event.payload;
    });
    const currentTablisten = listen<Tab>("Current_Tab", (event) => {
      // Update the Svelte store with the new counter value
      currentTab = event.payload;
    });
    return () => {
      tabslisten.then((unsub) => unsub());
      currentTablisten.then((unsub) => unsub());
    };
  });

  // const unsubscribeTabsState = TabsStore.states.subscribe((value) => {
  //   tabs = value.tabs;
  //   currentTab = value.currentTab;
  // });
  // onDestroy(unsubscribeTabsState); // Clean up

  const onOpenTab = (tab: Tab) => {
    //TabsStore.updateCurrentTabState(tab);
    tabService.switchTab(tab);
  };
</script>

<div class="flex grow justify-center mt-[30px] px-10 overflow-auto">
  <!-- {#each tabs as tab} -->
  <!-- {#if tab.tabType === TabType.Document || tab.tabType === undefined} -->
  <!-- TODO: Q: How to switch between tabs? -->
  <!-- 1: Have all tabs as separate DOM Elements, set display:none on inactive tabs -->
  <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
  <!-- 2: Have only active tab in DOM -->
  <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
  <Document />
  <!-- {/if} -->
  <!-- {/each} -->
</div>
