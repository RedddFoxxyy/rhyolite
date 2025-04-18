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
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import {
		editorTheme,
		markdownHighlightStyle
	} from "$lib/components/content-editor/cm-theme.svelte";
	import { contentEditorStore } from "$lib/stores/contentEditor.svelte";
	import DOMPurify from "dompurify";

	// HTMLElements for Codemirror container and marked preview container
	let editorContainer = $state<HTMLElement | null>(null);
	let markedPreviewContainer = $state<HTMLElement | null>(null);

	// Init Codemirror Editor View.
	let codemirrorEditorView: EditorView;

	// Init marked parsed content variable.
	let parsed_content: string | Promise<string> = $state("");

	// Create a Marked Instance.
	const marked: Marked = new Marked({ gfm: true });

	const { onContentChange = (update: ViewUpdate) => {} } = $props<{
		onContentChange?: (update: ViewUpdate) => void;
	}>();

	// Initialise the codemirror editor view with required extentions.
	function setupEditorView(): void {
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
				// wysiwygPlugin,
				syntaxHighlighting(markdownHighlightStyle),
				EditorView.updateListener.of(onContentChange),
				keymap.of([
					...closeBracketsKeymap,
					...defaultKeymap,
					...searchKeymap,
					...historyKeymap,
					...foldKeymap,
					...completionKeymap,
					...lintKeymap
				])
			],
			parent: editorContainer,
			doc: contentEditorStore.getDocumentContent()
		});
	}

	onMount(() => {
		setupEditorView();
		const docContentlisten: Promise<UnlistenFn> = listen<string>(
			"current_editor_content",
			(event) => {
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
					// If the editor is in preview mode then update the markdown preview with the content of new document.
					if (contentEditorStore.isPreviewMode()) {
						updateMarkdownPreview();
					}
				}
			}
		);
		return () => {
			docContentlisten.then((unsub) => unsub());
		};
	});

	$effect(() => {
		if (contentEditorStore.isPreviewMode()) {
			updateMarkdownPreview();
		}
	});

	async function updateMarkdownPreview(): Promise<void> {
		if (!codemirrorEditorView || !markedPreviewContainer) return;
		const currentContent: string = codemirrorEditorView.state.doc.toString();
		// Need to sanitize the HTML output (e.g., using DOMPurify) to prevent XSS attacks if the Markdown source is untrusted.
		parsed_content = DOMPurify.sanitize(await marked.parse(currentContent));
		markedPreviewContainer.innerHTML = parsed_content;
	}

	/**
	 * Updates the CodeMirror editor's content via a transaction.
	 * @param view The EditorView instance.
	 * @param newContent The new string content.
	 */
	function setEditorContent(view: EditorView, newContent: string): void {
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
	@import "./markedTheme.css";
</style>
