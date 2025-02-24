<script lang="ts">
  import { onMount } from "svelte";
  import { Editor } from "svelte-tiptap";
  import DocumentService from "../services/document.service";
  import TabService from "../services/tab.service";
  import type { Tab } from "../types/tab";
  import ContentEditor from "./content-editor/content-editor.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";

  let currentTab: Tab | null = $state(null);
  let documentTitle: string = $state("");
  let documentContent: any = $state();
  let wordCount: number = $state(0);
  let charCount: number = $state(0);
  let initialized: boolean = $state(false);
  onMount(() => {
    initialized = true;
    // const docContentlisten = listen<any>("current_editor_content", (event) => {
    //   documentContent = event.payload;
    // });
    const currentTablisten = listen<Tab>("Current_Tab", (event) => {
      currentTab = event.payload;
      documentTitle = currentTab.title;
      invoke("exec_command", {
        cmd: "get_document_content",
        payload: JSON.stringify({
          id: currentTab.id,
          title: currentTab.title,
        }),
      });
    });
    return () => {
      currentTablisten.then((unsub) => unsub());
      // docContentlisten.then((unsub) => unsub());
    };
  });

  const handleTitleChange = (event: Event) => {
    const target = event.target as HTMLTextAreaElement;
    documentTitle = target.value;
    // TabService.updateTabTitleById(currentTab.id, target.value);
    if (currentTab) {
      invoke("exec_command", {
        cmd: "update_tab_title",
        payload: JSON.stringify({ id: currentTab.id, title: target.value }),
      });
      saveDocument();
    }
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
      if (currentTab) {
        DocumentService.saveDocument({
          documentId: currentTab.id,
          documentTitle,
          documentContent,
        });
      }
    }, delaySave ?? 500);
  };
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
