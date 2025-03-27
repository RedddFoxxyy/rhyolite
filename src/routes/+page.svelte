<script lang="ts">
	import { onMount } from "svelte";
	import CommandPalette from "$lib/components/command-palette.svelte";
	import RecentFiles from "$lib/components/recentfilesmenu.svelte";
	import Document from "$lib/components/document.svelte";
	import TitleBar from "$lib/components/titlebar.svelte";
	import HomeHotkeys from "$lib/components/home-hotkeys.svelte";
	import DocumentService from "$lib/tauri-cmd/document";
	import Sidebar from "$lib/components/sidebar.svelte";
	import { themes_store } from "$lib/stores/themes.svelte";

	onMount(() => {
		// TabsStore.initTabsStore();
		DocumentService.initFrontendState();
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
		<div class="flex grow justify-center mt-[30px] px-10 overflow-auto">
			<!-- TODO: Q: How to switch between tabs? -->
			<!-- 1: Have all tabs as separate DOM Elements, set display:none on inactive tabs -->
			<!--    Pro: possibly retained DOM states. Con: Too large DOM-->
			<!-- 2: Have only active tab in DOM -->
			<!--    Pro: possibly retained DOM states. Con: Too large DOM-->
			<Document />
		</div>
	</div>
	<HomeHotkeys />
	<CommandPalette />
	<RecentFiles />
</main>
