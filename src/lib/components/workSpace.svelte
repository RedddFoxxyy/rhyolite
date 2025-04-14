<script lang="ts">
	import { onMount } from "svelte";
	import type { ViewUpdate } from "@codemirror/view";
	import documentCmds from "$lib/tauri-cmd/document";
	import type { Tab } from "$lib/types/tab";
	import CodemirrorEditor from "$lib/components/content-editor/codemirror-editor.svelte";
	import { contentEditorStore } from "$lib/stores/contentEditor.svelte";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { Code, BookOpen } from "lucide-svelte";

	let currentTab: Tab | null = $state(null);
	let documentTitle: string = $state("");

	onMount(() => {
		const currentTablisten: Promise<UnlistenFn> = listen<Tab>("Current_Tab", (event) => {
			currentTab = event.payload;
			documentTitle = currentTab.title;
		});
		return () => {
			currentTablisten.then((unsub) => unsub());
		};
	});

	function handleTitleChange(event: Event) {
		const target: HTMLTextAreaElement = event.target as HTMLTextAreaElement;
		documentTitle = target.value;
		if (currentTab) {
			documentCmds.updateTabTitle(currentTab.id, target.value);
			saveDocument();
		}
	}

	let saveTimeout: number | undefined;
	const delaySave: number = 200;

	async function handleContentChange(update: ViewUpdate): Promise<void> {
		if (update.docChanged) {
			contentEditorStore.setDocumentContent(update.state.doc.toString());
			saveDocument();
		}
	}

	async function saveDocument(): Promise<void> {
		// Clear the previous timeout
		if (saveTimeout) clearTimeout(saveTimeout);
		// Set a new timeout to trigger `saveAction` after 0.2 seconds
		saveTimeout = setTimeout(() => {
			if (currentTab) {
				documentCmds.saveDocument(
					currentTab.id,
					documentTitle,
					contentEditorStore.getDocumentContent()
				);
			}
		}, delaySave ?? 200);
	}
</script>

<!-- TODO: Decide whether not open tabs should be hidden or removed from DOM -->
<div class=" flex flex-col w-full max-w-screen">
	<div class="flex h-[80px] mb-6 mx-auto justify-center w-[50%] min-w-[300px]">
		<textarea
			class="w-full h-full resize-none border-none bg-base rounded-lg py-7 text-text text-[2rem] focus:outline-none focus:ring-0 shadow-lg"
			placeholder="Enter title here..."
			value={documentTitle}
			oninput={handleTitleChange}
			id="document-title-input"
		></textarea>
	</div>
	<CodemirrorEditor onContentChange={handleContentChange} />
	<div
		class="fixed flex flex-row gap-[20px] text-nowrap self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em] select-none"
	>
		<button
			onclick={() => contentEditorStore.toggleDocumentMode()}
			class="w-full rounded-lg text-left text-text bg-transparent cursor-pointer transition-all duration-300 hover:bg-surface1"
			title={contentEditorStore.isPreviewMode()
				? "Switch to Source Mode."
				: "Switch to Preview Mode."}
		>
			{#if contentEditorStore.isPreviewMode()}
				<Code class="w-4 h-4" />
			{:else}
				<BookOpen class="w-4 h-4" />
			{/if}
		</button>
		<div>{contentEditorStore.wordCount} Words</div>
		<div>{contentEditorStore.charCount} Characters</div>
	</div>
</div>
