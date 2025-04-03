import { EditorView } from "@codemirror/view";
import type { Extension } from "@codemirror/state";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

export const editorTheme = EditorView.theme({
	"&.cm-focused": {
		outline: "none !important"
	},
	".cm-content": {
		color: "var(--color-text)",
		caretColor: "var(--color-text)",
		fontSize: "16px"
	},
	"&.cm-focused .cm-cursor": {
		borderLeftColor: "var(--color-text)"
	},
	// For focused editor selection
	"&.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground": {
		background: "rgb(var(--color-subtext0)) !important",
		opacity: 0.5
	},

	// For unfocused selection
	".cm-selectionBackground": {
		background: "rgb(var(--color-subtext0)) !important",
		backgroundColor: "rgb(var(--color-subtext0)) !important",
		opacity: 0.3
	},
	".cm-gutters": {
		background: "var(--color-surface0)",
		backgroundColor: "var(--color-surface0)",
		borderRight: "None"
	},
	".cm-foldPlaceholder": {
		background: "None",
		border: "None",
		color: "rgb(var(--color-subtext0))"
	}
});

export const markdownHighlightStyle = HighlightStyle.define([
	{
		tag: t.heading1,
		fontSize: "2.2em",
		fontWeight: "bold"
	},
	{
		tag: t.heading2,
		fontSize: "2em",
		fontWeight: "bold"
	},
	{
		tag: t.heading3,
		fontSize: "1.8em",
		fontWeight: "bold"
	},
	{
		tag: t.heading4,
		fontSize: "1.6em",
		fontWeight: "bold"
	},
	{
		tag: t.heading5,
		fontSize: "1.4em",
		fontWeight: "bold"
	},
	{
		tag: t.heading6,
		fontSize: "1.2em",
		fontWeight: "bold"
	},
	{
		tag: t.emphasis,
		// color: "#666666",
		fontStyle: "italic"
	},
	{
		tag: t.processingInstruction, // Handles #, >, etc.
		color: "rgb(var(--color-subtext0))"
	},
	{
		tag: t.strong,
		fontWeight: "bold"
	},
	{
		tag: t.strikethrough, // Strikethrough text
		textDecoration: "line-through",
		color: "rgb(var(--color-subtext0))"
	},
	{
		tag: t.quote // Blockquotes
		//   fontStyle: "italic",
	}
]);
