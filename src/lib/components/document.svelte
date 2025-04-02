<script lang="ts">
	import { onMount } from "svelte";
	import { Editor } from "svelte-tiptap";
	import documentCmds from "$lib/tauri-cmd/document";
	import type { Tab } from "$lib/types/tab";
	import ContentEditor from "$lib/components/content-editor/content-editor.svelte";
	import CodemirrorEditor from "$lib/components/content-editor/codemirror-editor.svelte";
	import { listen } from "@tauri-apps/api/event";

	let currentTab: Tab | null = $state(null);
	let documentTitle: string = $state("");
	let documentContent: any = $state();
	let wordCount: number = $state(0);
	let charCount: number = $state(0);

	onMount(() => {
		const currentTablisten = listen<Tab>("Current_Tab", (event) => {
			currentTab = event.payload;
			documentTitle = currentTab.title;
		});
		return () => {
			currentTablisten.then((unsub) => unsub());
		};
	});

	function handleTitleChange(event: Event) {
		const target = event.target as HTMLTextAreaElement;
		documentTitle = target.value;
		if (currentTab) {
			documentCmds.updateTabTitle(currentTab.id, target.value);
			saveDocument();
		}
	}

	let saveTimeout: number | undefined;
	const delaySave = 200;

	function handleContentChange(editor: Editor) {
		documentContent = editor.getHTML();
		// console.log(documentContent) // Uncomment for debugging.
		// Update word and character counts
		wordCount = editor.storage.characterCount.words();
		charCount = editor.storage.characterCount.characters();
		saveDocument();
	}

	async function saveDocument() {
		// Clear the previous timeout
		if (saveTimeout) clearTimeout(saveTimeout);
		// Set a new timeout to trigger `saveAction` after 0.2 seconds
		saveTimeout = setTimeout(() => {
			if (currentTab) {
				documentCmds.saveDocument(currentTab.id, documentTitle, documentContent);
			}
		}, delaySave ?? 200);
	}
</script>

<!-- TODO: Decide whether not open tabs should be hidden or removed from DOM -->
<div class={` flex flex-col w-full max-w-screen-xl`}>
	<div class="flex h-[80px] mb-6 mx-auto justify-center w-[50%] min-w-[300px]">
		<textarea
			class="w-full h-full resize-none border-none bg-base rounded-lg py-7 text-text text-[2rem] focus:outline-none focus:ring-0 shadow-lg"
			placeholder="Enter title here..."
			value={documentTitle}
			oninput={handleTitleChange}
		></textarea>
	</div>
	<CodemirrorEditor/>
	<div
		class="fixed flex flex-row gap-[20px] text-nowrap self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em] select-none"
	>
		<div>{wordCount} Words</div>
		<div>{charCount} Characters</div>
	</div>
</div>
