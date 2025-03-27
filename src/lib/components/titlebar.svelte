<script lang="ts">
	import Close from "$lib/static/close.svg.svelte";
	import Maximise from "$lib/static/maximise.svg.svelte";
	import Minimise from "$lib/static/minimise.svg.svelte";
	import Restore from "$lib/static/restore.svg.svelte";
	import { listen } from "@tauri-apps/api/event";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { addNewDocumentTab } from "$lib/tauri-cmd/document";
	import tabCmds from "$lib/tauri-cmd/tab";
	import { type Tab } from "$lib/types/tab";
	import { platform } from "@tauri-apps/plugin-os";

	let tabs: Tab[] = $state([]);
	let currentTab: Tab | null = $state(null);
	let isMaximized: boolean = $state(false);
	let isMacOS: boolean = $state(false); // New state for OS detection

	const appWindow = getCurrentWindow();

	appWindow.listen("tauri://resize", async () => {
		isMaximized = await appWindow.isMaximized();
	});

	onMount(async () => {
		// First, check the platform
		try {
			const current_platform = platform();
			console.log("Platform:", current_platform);
			isMacOS = current_platform === "macos";
		} catch (error) {
			console.error("Error detecting platform:", error);
		}
	});

	// Handle the event listeners in a separate onMount that returns cleanup
	onMount(() => {
		// Listen for the 'Tabs' event from the backend
		const tabsPromise = listen<Tab[]>("Tabs", (event) => {
			tabs = event.payload;
		});

		const currentTabPromise = listen<Tab>("Current_Tab", (event) => {
			currentTab = event.payload;
		});
		return () => {
			tabsPromise.then((unsub) => unsub());
			currentTabPromise.then((unsub) => unsub());
		};
	});

	$effect(() => {
		if (currentTab) {
			document
				.querySelector(
					currentTab.id === tabs[tabs.length - 1].id
						? "#tablist>#new-tab-btn"
						: "#tablist>.active"
				)
				?.scrollIntoView({
					behavior: "smooth",
					block: "nearest",
					inline: "nearest"
				});
		}
	});
</script>

<div
	data-tauri-drag-region
	class="flex grow-0 shrink-0 bg-base w-full select-none justify-between items-center overflow-hidden {isMacOS
		? 'pt-6 py-2'
		: 'basis-[40px]'}"
>
	<div
		class="flex items-center h-full ml-7 px-4 flex-shrink-1 flex-grow-0 overflow-y-hidden overflow-x-auto gap-1 {isMacOS
			? 'ml-0 '
			: 'ml-7'}"
		role="tablist"
		id="tablist"
		aria-label="Document tabs"
	>
		{#each tabs as tab}
			<div class="relative group flex items-center justify-between">
				<button
					class={`flex justify-left items-center pl-4 pr-2 text-nowrap h-[30px] w-fit rounded-[18px] shrink text-text transition-all duration-150 hover:bg-surface1 ${currentTab?.id === tab.id ? "bg-surface0" : ""}`}
					class:active={currentTab?.id === tab.id}
					role="tab"
					aria-controls="editor"
					onclick={() => tabCmds.switchTab(tab)}
				>
					{tab.title.length > 20
						? tab.title.slice(0, 20) + "..."
						: tab.title || "Untitled"}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div
						class="text-text bg-transparent ml-2 p-1 rounded-[18px] h-[20px] w-[20px] flex justify-center items-center opacity-0 transition-all duration-150 hover:bg-surface2 hover:text-subtext1 hover:opacity-80"
						class:opacity-100={currentTab?.id === tab.id}
						onclick={(e) => {
							e.stopPropagation();
							tabCmds.closeTab(tab.id);
						}}
					>
						<Close />
					</div>
				</button>
			</div>
		{/each}
		<button
			type="button"
			class="flex justify-center items-center px-4 text-nowrap h-[30px] w-[30px] aspect-square rounded-[18px] shrink text-text transition-all duration-150 hover:bg-surface1"
			id="new-tab-btn"
			onclick={addNewDocumentTab}>+</button
		>
	</div>
	<div class="flex-grow"></div>
	{#if !isMacOS}
		<div class="flex flex-row items-stretch self-stretch shrink-0">
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
	{/if}
</div>
