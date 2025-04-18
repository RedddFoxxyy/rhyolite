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
