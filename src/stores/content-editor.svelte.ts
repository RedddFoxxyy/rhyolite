class ContentEditorStore {
    #isVisible = $state(false);

    isToolbarVisible(): boolean {
        return this.#isVisible;
    }

    toggleToolbarVisibility() {
        this.#isVisible = !this.#isVisible;
    }

}

export const contentEditorStore = new ContentEditorStore();
