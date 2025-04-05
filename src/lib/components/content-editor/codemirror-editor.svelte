<script lang="ts">
	import { EditorView, minimalSetup } from "codemirror";
	import {
		ViewPlugin,
		keymap,
		drawSelection,
		highlightActiveLine,
		dropCursor,
		rectangularSelection,
		crosshairCursor,
		lineNumbers
	} from "@codemirror/view";
	import type { ViewUpdate } from "@codemirror/view";
	import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
	import { EditorState, StateEffect } from "@codemirror/state";
	import {
		defaultHighlightStyle,
		syntaxHighlighting,
		indentOnInput,
		bracketMatching,
		foldGutter,
		foldKeymap
	} from "@codemirror/language";
	import { languages } from "@codemirror/language-data";
	import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
	import { lintKeymap } from "@codemirror/lint";
	import {
		autocompletion,
		completionKeymap,
		closeBrackets,
		closeBracketsKeymap
	} from "@codemirror/autocomplete";
	import { markdown, commonmarkLanguage, markdownLanguage } from "@codemirror/lang-markdown";
	import { Marked } from "marked";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import {
		editorTheme,
		markdownHighlightStyle
	} from "$lib/components/content-editor/cm-theme.svelte";
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
	const marked: Marked = new Marked({});

	const { onContentChange = (update: ViewUpdate) => {} } = $props<{
		onContentChange?: (update: ViewUpdate) => void;
	}>();

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
				drawSelection(),
				dropCursor(),
				foldGutter(),
				autocompletion(),
				rectangularSelection(),
				crosshairCursor(),
				highlightSelectionMatches(),
				bracketMatching(),
				closeBrackets(),
				indentOnInput(),
				syntaxHighlighting(markdownHighlightStyle),
				EditorView.updateListener.of(onContentChange)
			],
			parent: editorContainer,
			doc: initial_editor_content
		});
	}

	onMount(() => {
		setupEditorView();
		const docContentlisten = listen<string>("current_editor_content", (event) => {
			const documentContent = event.payload;
			// console.log("Setting editor content from event");
			if (codemirrorEditorView) {
				setEditorContent(codemirrorEditorView, documentContent);
				// If the editor is in preview mode then update the markdown preview with the content of new document.
				if (contentEditorStore.isPreviewMode()) {
					updateMarkdownPreview();
				}
			} else {
				// Fallback if event arrives before editor is ready (less likely but possible)
				console.warn(
					"EditorView not ready when receiving content, updating initial content instead."
				);
				initial_editor_content = documentContent;
				// If the editor is in preview mode then update the markdown preview with the content of new document.
				if (contentEditorStore.isPreviewMode()) {
					updateMarkdownPreview();
				}
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

<main class=" overflow-auto mb-15 p-2 min-h-96 w-[90%] h-full min-w-[400px] mx-auto">
	<div
		bind:this={editorContainer}
		class="text-text"
		style={contentEditorStore.isPreviewMode() ? "display: none;" : ""}
	></div>

	{#if contentEditorStore.isPreviewMode()}
		{@const _ = updateMarkdownPreview()}
		<div bind:this={markedPreviewContainer} class="markdown-preview text-text"></div>
	{/if}
</main>

<!-- This styling needs to be moved to a css file, also can this be replaced with tailwind class? -->
<style>
	.markdown-preview :global(h1) {
		font-size: 2em;
		font-weight: bold;
		margin-top: 0.67em;
		margin-bottom: 0.67em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(h2) {
		font-size: 1.8em;
		font-weight: bold;
		margin-top: 0.83em;
		margin-bottom: 0.83em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(h3) {
		font-size: 1.6em;
		font-weight: bold;
		margin-top: 1em;
		margin-bottom: 1em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(h4) {
		font-size: 1.4em;
		font-weight: bold;
		margin-top: 1.33em;
		margin-bottom: 1.33em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(h5) {
		font-size: 1.2em;
		font-weight: bold;
		margin-top: 1.67em;
		margin-bottom: 1.67em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(h6) {
		font-size: 1em;
		font-weight: bold;
		margin-top: 2.33em;
		margin-bottom: 2.33em;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(p) {
		margin-top: 1em;
		margin-bottom: 1em;
		line-height: 1.6;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(ul) {
		padding-left: 2em;
		margin-top: 1em;
		margin-bottom: 1em;
		color: rgb(var(--color-text));
		list-style-type: disc;
	}

	.markdown-preview :global(ol) {
		padding-left: 2em;
		margin-top: 1em;
		margin-bottom: 1em;
		color: rgb(var(--color-text));
		list-style-type: decimal;
	}

	.markdown-preview :global(blockquote) {
		background-color: rgb(var(--color-base));
		border-left: 4px solid rgb(var(--color-subtext0));
		padding-left: 1em;
		margin-left: 0;
		color: rgb(var(--color-text));
	}

	.markdown-preview :global(code) {
		background-color: rgb(var(--color-base));
		padding: 0.2em 0.4em;
		border-radius: 3px;
		font-family: monospace;
	}

	.markdown-preview :global(pre) {
		background-color: rgb(var(--color-mantle));
		padding: 1em;
		border-radius: 5px;
		overflow-x: auto;
		width: fit-content;
		min-width: 200px;
	}

	.markdown-preview :global(pre code) {
		background-color: transparent;
		padding: 0;
	}

	.markdown-preview :global(a) {
		color: rgb(var(--color-subtext0));
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
		border: 1px solid rgb(var(--color-subtext0));
		padding: 8px;
	}

	.markdown-preview :global(th) {
		background-color: rgb(var(--color-crust));
		text-align: left;
	}
</style>
