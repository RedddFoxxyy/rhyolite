export type DocumentMode = 'source' | 'preview';

class ContentEditorStore {
	#isVisible: boolean = $state(false);
	#documentMode: DocumentMode = $state('source');

	isVisible(): boolean {
		return this.#isVisible;
	}

	toggleToolbarVisibility(): boolean {
		this.#isVisible = !this.#isVisible;
		return this.#isVisible;
	}

	toggleDocumentMode() {
		if (this.#documentMode == 'source') {
			this.#documentMode = 'preview';
		} else {
			this.#documentMode = 'source';
		}
	}

	isPreviewMode(): boolean {
		if (this.#documentMode == 'preview') {
			return true;
		} else {
			return false;
		}
	}
}

export const contentEditorStore = new ContentEditorStore();
