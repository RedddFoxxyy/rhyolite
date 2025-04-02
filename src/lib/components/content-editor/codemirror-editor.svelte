<script lang="ts">
	import { EditorView, basicSetup, minimalSetup } from "codemirror";
	import { ViewPlugin, keymap } from "@codemirror/view";
	import type { ViewUpdate } from "@codemirror/view";
	import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
	import { EditorState, StateEffect } from "@codemirror/state";
	import { languages } from "@codemirror/language-data";
	import { markdown, commonmarkLanguage, markdownLanguage } from "@codemirror/lang-markdown";
	import { Marked } from "marked";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import { editorTheme } from "./components/cm-theme.svelte";
	import { contentEditorStore } from "$lib/stores/contentEditor.svelte";

	// HTMLElements for Codemirror container and marked preview container
	let editorContainer = $state<HTMLElement | null>(null);
	let markedPreviewContainer = $state<HTMLElement | null>(null);

	// Initial Editor content.
	let initial_editor_content: string = $state("Welcome to Rhyolite!");

	// Init Codemirror Editor View.
	let codemirrorEditorView: EditorView;

	// Init marked parsed content variable.
	let parsed_content: string | Promise<string> = $state("");

	// Create a Marked Instance.
	const marked: Marked = new Marked({ gfm: true });

	const { onContentChange = (update: ViewUpdate) => {} } = $props<{
		onContentChange?: (update: ViewUpdate) => void;
	}>();

	onMount(() => {
		setupEditorView();
		const docContentlisten = listen<string>("current_editor_content", (event) => {
			const documentContent = event.payload;
			// console.log("Setting editor content from event");
			if (codemirrorEditorView) {
				setEditorContent(codemirrorEditorView, documentContent);
			} else {
				// Fallback if event arrives before editor is ready (less likely but possible)
				console.warn(
					"EditorView not ready when receiving content, updating initial content instead."
				);
				initial_editor_content = documentContent;
				// Note: This won't update the editor if it mounts *later* with the old initial content.
				// Robust handling might require queueing the update.
			}

			// onchange($editor);
		});
		return () => {
			docContentlisten.then((unsub) => unsub());
		};
	});

	$effect(() => {
        if (contentEditorStore.isPreviewMode()) {
            updateMarkdownPreview();
        }
    });

	async function updateMarkdownPreview() {
		if (!codemirrorEditorView || !markedPreviewContainer) return;
		const currentContent: string = codemirrorEditorView.state.doc.toString();
		parsed_content = await marked.parse(currentContent);
		markedPreviewContainer.innerHTML = parsed_content;
	}

	// Initialise the codemirror editor view with required extentions.
	function setupEditorView() {
		if (codemirrorEditorView || !editorContainer) return; // Might be a hacky fix for the null issue
		if (codemirrorEditorView) return;
		codemirrorEditorView = new EditorView({
			extensions: [
				editorTheme,
				EditorView.lineWrapping,
				minimalSetup,
				markdown({
					codeLanguages: languages,
					base: markdownLanguage,
					completeHTMLTags: true,
					addKeymap: true
				}),
				history(),
				EditorView.updateListener.of(onContentChange)
			],
			parent: editorContainer,
			doc: initial_editor_content
		});
	}

	/**
	 * Updates the CodeMirror editor's content via a transaction.
	 * @param view The EditorView instance.
	 * @param newContent The new string content.
	 */
	function setEditorContent(view: EditorView, newContent: string) {
		if (!view) {
			console.error("EditorView is not initialized yet.");
			return;
		}
		const currentDocLength = view.state.doc.length;
		view.dispatch({
			changes: { from: 0, to: currentDocLength, insert: newContent }
		});
	}
</script>

<main class=" overflow-auto mb-20 p-2 min-h-96 w-[90%] h-full min-w-[400px] mx-auto">
    <div bind:this={editorContainer} class="text-text" style={contentEditorStore.isPreviewMode() ? 'display: none;' : ''}></div>
    
    {#if contentEditorStore.isPreviewMode()}
        {@const _ = updateMarkdownPreview()}
        <div bind:this={markedPreviewContainer} class="markdown-preview text-text"></div>
    {/if}
</main>

<!-- This styling needs to be moved to a css file, also can this be replaced with tailwind class? -->
<style>
	.markdown-preview :global(h1) {
    font-size: 2em;
    margin-top: 0.67em;
    margin-bottom: 0.67em;
    font-weight: bold;
    color: var(--color-text);
}

.markdown-preview :global(h2) {
    font-size: 1.5em;
    margin-top: 0.83em;
    margin-bottom: 0.83em;
    font-weight: bold;
    color: var(--color-text);
}

.markdown-preview :global(h3) {
    font-size: 1.17em;
    margin-top: 1em;
    margin-bottom: 1em;
    font-weight: bold;
    color: var(--color-text);
}

.markdown-preview :global(p) {
    margin-top: 1em;
    margin-bottom: 1em;
    line-height: 1.6;
	color: var(--color-text);
}

.markdown-preview :global(ul) {
    padding-left: 2em;
    margin-top: 1em;
    margin-bottom: 1em;
	color: var(--color-text);
	list-style-type: disc;
	list-style-type: decimal;
}
 
.markdown-preview :global(ol) {
    padding-left: 2em;
    margin-top: 1em;
    margin-bottom: 1em;
	color: var(--color-text);
}

.markdown-preview :global(blockquote) {
    border-left: 4px solid #ccc;
    padding-left: 1em;
    margin-left: 0;
    color: var(--color-text);
}

.markdown-preview :global(code) {
    background-color: #f5f5f5;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-family: monospace;
}

.markdown-preview :global(pre) {
    background-color: #f5f5f5;
    padding: 1em;
    border-radius: 5px;
    overflow-x: auto;
}

.markdown-preview :global(pre code) {
    background-color: transparent;
    padding: 0;
}

.markdown-preview :global(a) {
    color: #0366d6;
    text-decoration: none;
}

.markdown-preview :global(a:hover) {
    text-decoration: underline;
}

.markdown-preview :global(table) {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
}

.markdown-preview :global(th), 
.markdown-preview :global(td) {
    border: 1px solid #ddd;
    padding: 8px;
}

.markdown-preview :global(th) {
    background-color: #f2f2f2;
    text-align: left;
}
</style>
