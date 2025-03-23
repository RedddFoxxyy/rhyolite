class ContentEditorStore {
	#flagToolbarVisibility: boolean = $state(false);

	isVisible(): boolean {
		return this.#flagToolbarVisibility;
	}

	toggleToolbarVisibility(): boolean {
		this.#flagToolbarVisibility = !this.#flagToolbarVisibility;
		return this.#flagToolbarVisibility;
	}
}

export const contentEditorStore = new ContentEditorStore();
