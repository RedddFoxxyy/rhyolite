import { Marked } from "marked";
import DOMPurify from "dompurify";
import {
	WidgetType,
	ViewPlugin,
	Decoration,
	EditorView,
	ViewUpdate,
	type DecorationSet
} from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";
import { RangeSetBuilder } from "@codemirror/state";

const marked = new Marked({
	// Configure marked options if needed (e.g., gfm: true, breaks: false)
	gfm: true,
	pedantic: false
});

// Function to safely parse and sanitize
function safeMarkedParse(markdownFragment: string): string {
	const rawHTML = marked.parse(markdownFragment);
	// Important: Sanitize the HTML!
	return DOMPurify.sanitize(rawHTML as string); // Cast needed as marked types might be Promise<string> sometimes
}

class HtmlWidget extends WidgetType {
	constructor(readonly html: string) {
		super();
	}

	eq(other: HtmlWidget) {
		return other.html === this.html; // Only redraw if HTML content changes
	}

	toDOM() {
		// Create the wrapper element that will be displayed in the editor
		// It replaces the source text visually
		const wrap = document.createElement("div");
		wrap.setAttribute("aria-hidden", "true"); // Hide from screen readers (source is still there)
		wrap.className = "cm-rendered-html-block"; // For styling
		wrap.style.display = "inline-block"; // Prevent it taking full width unnecessarily
		wrap.innerHTML = this.html;
		return wrap;
	}

	// Ignore events within the widget to allow clicking through
	// to position the cursor in the underlying source text.
	// You might need more complex logic here later for interaction
	// within the widget itself if required.
	ignoreEvent() {
		return true;
	}
}

export const wysiwygPlugin = ViewPlugin.fromClass(
	class {
		decorations: DecorationSet;

		constructor(view: EditorView) {
			this.decorations = this.buildDecorations(view.state);
		}

		update(update: ViewUpdate) {
			// Only rebuild decorations if the document, selection, or viewport changed
			if (update.docChanged || update.selectionSet || update.viewportChanged) {
				this.decorations = this.buildDecorations(update.state);
			}
		}

		buildDecorations(state: EditorState): DecorationSet {
			const builder = new RangeSetBuilder<Decoration>();
			const cursorPos = state.selection.main.head; // Use the primary cursor position

			// We'll iterate through the top-level syntax nodes (blocks)
			syntaxTree(state).iterate({
				enter: (node) => {
					// Check if it's a relevant top-level block node
					// Adjust these node types based on lezer-parser-markdown grammar
					const isBlock = [
						"Paragraph",
						"Heading1",
						"Heading2",
						"Heading3",
						"Heading4",
						"Heading5",
						"Heading6",
						"Blockquote",
						"ListItem",
						"BulletList",
						"OrderedList", // List items are tricky, might need parent check
						"FencedCode",
						"CodeBlock",
						"HTMLBlock",
						"HorizontalRule"
					].includes(node.name); // Check node.type.name in newer Lezer versions

					if (!isBlock) {
						// Don't process nodes that aren't top-level blocks we want to render
						// (e.g., don't render *just* Bold inside a paragraph)
						// Only return true to descend into children if necessary (e.g., lists)
						// Careful here: You need to identify the actual blocks. Sometimes iterating topNode.cursor() is better.
						// Let's refine: Iterate top node children instead.
						return false; // Stop descending into this node for this simple example
					}

					// Check if the main cursor is INSIDE this block's range
					const cursorInside = cursorPos >= node.from && cursorPos <= node.to;

					// If cursor is NOT inside, render the block
					if (!cursorInside) {
						const markdownSlice = state.doc.sliceString(node.from, node.to);
						if (markdownSlice.trim()) {
							// Don't render empty blocks
							try {
								const renderedHtml = safeMarkedParse(markdownSlice);
								const widget = Decoration.replace({
									widget: new HtmlWidget(renderedHtml),
									block: true // Treat this decoration as a block element
								});
								builder.add(node.from, node.to, widget);
							} catch (e) {
								console.error("Error parsing Markdown fragment:", e, markdownSlice);
								// Optional: Add an error decoration?
							}
						}
					}
					// Don't descend further into this block from the top-level iteration
					return false;
				},
				// We only care about top-level blocks for replacement
				from: state.viewport.from, // Optimize: only iterate nodes in viewport
				to: state.viewport.to
			});

			// --- Refined Block Iteration (Alternative using topNode.cursor) ---
			// This might be more reliable for getting actual top-level blocks
			const tree = syntaxTree(state);
			let cursor = tree.topNode.cursor(); // Get cursor starting at the top
			do {
				const node = cursor.node; // Current node reference
				// Check node.name or node.type.name for block types
				const isBlock = [
					/* ... block names ... */
				].includes(node.name); // Check node.type.name if needed

				if (isBlock && node.from >= state.viewport.from && node.to <= state.viewport.to) {
					// Only process blocks fully in viewport
					const cursorInside = cursorPos >= node.from && cursorPos <= node.to;

					if (!cursorInside) {
						const markdownSlice = state.doc.sliceString(node.from, node.to);
						if (markdownSlice.trim()) {
							try {
								const renderedHtml = safeMarkedParse(markdownSlice);
								const widget = Decoration.replace({
									widget: new HtmlWidget(renderedHtml),
									block: true
								});
								builder.add(node.from, node.to, widget);
							} catch (e) {
								console.error("Error parsing Markdown fragment:", e, markdownSlice);
							}
						}
					}
				}
			} while (cursor.nextSibling()); // Move to the next sibling block

			// --- End Refined Iteration ---

			return builder.finish();
		}
	},
	{
		decorations: (v) => v.decorations
	}
);
