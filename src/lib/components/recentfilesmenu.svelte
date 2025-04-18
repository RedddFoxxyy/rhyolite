<script lang="ts">
	import { recentFilesStore } from "$lib/stores/recentFiles.svelte";
	import { onMount } from "svelte";
	import type { RecentFileInfo } from "$lib/types/document";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import documentCmds from "$lib/tauri-cmd/document";

	let files: RecentFileInfo[] = $state([]);
	let filteredFiles: RecentFileInfo[] = $state([]);
	let selectedIndex: number = $state(-1);
	let searchText: string = $state("");

	onMount(() => {
		// Listen for the 'recent_files_metadata' event from the backend
		const recentFileslisten: Promise<UnlistenFn> = listen<RecentFileInfo[]>(
			"recent_files_metadata",
			(event) => {
				files = event.payload;
			}
		);
		return () => {
			recentFileslisten.then((unsub) => unsub());
		};
	});

	function loadFiles(): void {
		try {
			documentCmds.getRecentlyOpenedFiles();
		} catch (error) {
			console.error("Failed to load files:", error);
		}
	}

	function openFile(file: RecentFileInfo): void {
		try {
			documentCmds.loadDocument(file);
			recentFilesStore.toggleVisibility();
		} catch (error) {
			console.error("Failed to open file:", error);
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
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
				recentFilesStore.toggleVisibility();
				break;
		}
	}

	$effect(() => {
		filteredFiles = files.filter((RecentFileInfo) => 
			RecentFileInfo.title.toLowerCase().includes(searchText.toLowerCase())
		);
	});

	$effect(() => {
		if (recentFilesStore.isVisible()) {
			loadFiles();
			(document.querySelector("#recentFilesTextArea") as HTMLTextAreaElement).focus();
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
		class="fixed flex justify-center items-center top-0 left-0 w-full h-full bg-black/20 backdrop-blur-xs z-20"
		tabindex="-1"
		aria-modal="true"
		role="dialog"
		onclick={(e) => {
			if (e.target === e.currentTarget) recentFilesStore.toggleVisibility();
		}}
	>
		<div
			class="fixed top-[40%] left-1/2 flex flex-col bg-crust rounded-lg p-3 z-[60] w-min-[200px] w-[50%] h-[40%] gap-2 overflow-hidden -translate-x-1/2 -translate-y-1/2"
		>
			<div
				class="relative basis-[42px] w-full shrink-0 overflow-hidden shadow-none hover:shadow-xl focus:shadow-xl transition duration-300 rounded-lg"
			>
				<textarea
					id="recentFilesTextArea"
					class="w-full h-full overflow-hidden resize-none p-2 cursor-text text-text bg-crust text-left box-border border-2 hover:border-subtext0 rounded-lg transition-all duration-200 border-overlay0 focus:border-subtext0 focus:outline-none focus:ring-0"
					placeholder="Search Recent Files"
					bind:value={searchText}
					onkeydown={handleKeydown}
				></textarea>
				<button
					class="absolute right-4 top-1/2 -translate-y-1/2 bg-transparent text-text opacity-70 hover:opacity-100 transition-opacity duration-200"
					onclick={() => recentFilesStore.toggleVisibility()}>✕</button
				>
			</div>
			<div class="flex overflow-y-auto flex-col gap-[0.5px]">
				<div class="mx-2">
					{#each filteredFiles as file, index}
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
