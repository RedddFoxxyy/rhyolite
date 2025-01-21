<script lang="ts">
  import { onMount } from "svelte";
  import { Editor } from "svelte-tiptap";
  import DocumentService from "../services/document.service";
  import TabService from "../services/tab.service";
  import type { Tab } from "../types/tab";
  import ContentEditor from "./content-editor/content-editor.svelte";

  interface DocumentTabItemProps {
    open?: boolean;
    tab: Tab;
    onclick: (event: Event) => void;
  }

  let { open, tab, onclick }: DocumentTabItemProps = $props();

  let documentTitle: string = $state("");
  let documentContent: any = $state();
  let wordCount: number = $state(0);
  let charCount: number = $state(0);
  let initialized: boolean = $state(false);

  onMount(async () => {
    documentTitle = tab.title;
    // content = tab.content;
    const doc = await DocumentService.loadDocument(tab.id, tab.title);
    initialized = true;

    if (!doc) return;
    documentContent = doc.content;
    documentTitle = doc.title;
    await TabService.updateTabTitleById(tab.id, documentTitle);
  });

  const handleTitleChange = (event: Event) => {
    const target = event.target as HTMLTextAreaElement;
    documentTitle = target.value;
    TabService.updateTabTitleById(tab.id, target.value);

    saveDocument();
  };

  let saveTimeout: number | undefined;
  const delaySave = 500;
  const handleContentChange = (editor: Editor) => {
    documentContent = editor.getHTML();
    // console.log(documentContent) // Uncomment for debugging.
    // Update word and character counts
    wordCount = editor.storage.characterCount.words();
    charCount = editor.storage.characterCount.characters();
    saveDocument();
  };

  const saveDocument = async () => {
    // Clear the previous timeout
    if (saveTimeout) clearTimeout(saveTimeout);
    // Set a new timeout to trigger `saveAction` after 0.5 seconds
    saveTimeout = setTimeout(() => {
      DocumentService.saveDocument({
        documentId: tab.id,
        documentTitle,
        documentContent,
      });
    }, delaySave ?? 500);
  };
</script>

<!-- TODO: Decide whether not open tabs should be hidden or removed from DOM -->
<div class={`${open ? "" : "hidden"} flex flex-col w-full max-w-screen-xl`}>
  <div class="flex h-[80px] mb-6 mx-auto justify-center w-[50%] min-w-[300px]">
    <textarea
      class="w-full h-full resize-none border-none bg-base rounded-lg py-7 text-text text-[2rem] focus:outline-none focus:ring-0 shadow-lg"
      placeholder="Enter title here..."
      value={documentTitle}
      oninput={handleTitleChange}
    ></textarea>
  </div>
  {#if initialized}
    <ContentEditor
      class="overflow-auto mb-20 p-2 min-h-96 w-[80%] min-w-[400px] mx-auto"
      content={documentContent}
      onchange={handleContentChange}
    />
  {/if}
  <div
    class="fixed flex flex-row gap-[20px] text-nowrap self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em] select-none"
  >
    <div>{wordCount} Words</div>
    <div>{charCount} Characters</div>
  </div>
</div>
