<script lang="ts">
  import {tabsStore} from "../stores/tabs.svelte";
  import { type Tab, TabType } from "../types/tab";
  import Document from "./document.svelte";

  const onOpenTab = (tab: Tab) => {
    tabsStore.updateCurrentTab(tab);
  };
</script>

<div class="flex grow justify-center mt-[30px] px-10 overflow-auto">
  {#each tabsStore.getTabs() as tab}
    {#if tab.tabType === TabType.Document || tab.tabType === undefined}
      <!-- TODO: Q: How to switch between tabs? -->
      <!-- 1: Have all tabs as separate DOM Elements, set display:none on inactive tabs -->
      <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
      <!-- 2: Have only active tab in DOM -->
      <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
      {@const currentTab = tabsStore.getCurrentTab()}
      <Document
        {tab}
        open={currentTab?.id === tab.id}
        onclick={() => onOpenTab(tab)}
      />
    {/if}
  {/each}
</div>
