class ContentEditorStore {
	#isVisible: boolean = $state(false);

	isVisible(): boolean {
		return this.#isVisible;
	}

	toggleToolbarVisibility(): boolean {
		this.#isVisible = !this.#isVisible;
		return this.#isVisible;
	}
}

export const contentEditorStore = new ContentEditorStore();
