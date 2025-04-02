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

	// HTMLElements for Codemirror container and marked preview container
	let editorContainer: HTMLElement;
	let markedPreviewContainer: HTMLElement;

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

	async function logEditorContent() {
		const currentContent: string = codemirrorEditorView.state.doc.toString();
		parsed_content = await marked.parse(currentContent);
		// marked_parsed_container.innerHTML = parsed_content;
		console.log(currentContent);
	}

	// Initialise the codemirror editor view with required extentions.
	function setupEditorView() {
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
	<div bind:this={editorContainer} class="text-text"></div>
	<!-- <button onclick={logEditorContent}>Click me to print to console!</button> -->
	<!-- <div bind:this={marked_parsed_container}></div> -->
</main>
