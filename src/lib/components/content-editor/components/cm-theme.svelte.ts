import { EditorView } from "@codemirror/view";

export const editorTheme = EditorView.theme({
	"&.cm-focused": {
		outline: "none !important"
	},
	"&.cm-focused .cm-cursor": {
		borderLeftColor: "var(--color-text)"
	},
	".cm-content": {
		color: "var(--color-text)",
		fontSize: "16px"
	}
});
