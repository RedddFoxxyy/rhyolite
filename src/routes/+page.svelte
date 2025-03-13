<script lang="ts">
  import "$lib/styles/styles.css";

  import { onMount } from "svelte";
  import CommandPalette from "$lib/components/command-palette.svelte";
  import RecentFiles from "$lib/components/recentfilesmenu.svelte";
  import Workspace from "$lib/components/workspace.svelte";
  import TitleBar from "$lib/components/titlebar.svelte";
  import HomeHotkeys from "$lib/components/home-hotkeys.svelte";
  import DocumentService from "$lib/services/document.service";
  import Sidebar from "$lib/components/sidebar.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { themes_store } from "$lib/stores/themes.svelte";

  onMount(() => {
    // TabsStore.initTabsStore();
    DocumentService.loadRecentDocuments();
    invoke("exec_command", { cmd: "load_last_open_tabs" });
    if (document.readyState === "complete") {
      themes_store.initThemesStore();
    } else {
      window.addEventListener("load", themes_store.initThemesStore);
    }
  });
</script>

<main class="flex flex-col h-lvh">
  <TitleBar />
  <div class="flex items-stretch grow overflow-hidden">
    <Sidebar />
    <Workspace />
  </div>
  <HomeHotkeys />
  <CommandPalette />
  <RecentFiles />
</main>
