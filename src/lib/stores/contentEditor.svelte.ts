// A Markdown Document can either be in source(editing) mode or in preview(reading) mode.
export type DocumentMode = "source" | "preview";

// Share and store data related to the current open document.
class ContentEditorStore {
	#isVisible: boolean = $state(false);
	#documentMode: DocumentMode = $state("source");
	#documentContent: string = $state("Welcome to Rhyolite!");
	public wordCount: number = $state(0);
	public charCount: number = $state(0);

	isVisible(): boolean {
		return this.#isVisible;
	}

	toggleToolbarVisibility(): boolean {
		this.#isVisible = !this.#isVisible;
		return this.#isVisible;
	}

	toggleDocumentMode(): void {
		if (this.#documentMode == "source") {
			this.#documentMode = "preview";
		} else {
			this.#documentMode = "source";
		}
	}

	isPreviewMode(): boolean {
		if (this.#documentMode == "preview") {
			return true;
		} else {
			return false;
		}
	}

	setDocumentContent(content: string): void {
		this.#documentContent = content;
		// console.log(this.#documentContent) // Uncomment for debugging.
		// Update word and character counts
		this.calculateCounts();
	}

	getDocumentContent(): string {
		return this.#documentContent;
	}

	calculateCounts(): void {
		this.charCount = this.#documentContent.length;
		// Trim whitespace from start/end, split by any whitespace sequence,
		// filter out empty strings (handles multiple spaces), and count.
		this.wordCount =
			this.#documentContent.trim() === ""
				? 0
				: this.#documentContent.trim().split(/\s+/).length;
	}
}

export const contentEditorStore: ContentEditorStore = new ContentEditorStore();
