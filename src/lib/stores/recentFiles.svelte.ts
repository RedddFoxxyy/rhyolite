class RecentFilesStore {
	#flagFilesMenuVisibility: boolean = $state(false);

	isVisible(): boolean {
		return this.#flagFilesMenuVisibility;
	}

	toggleVisibility(): boolean {
		this.#flagFilesMenuVisibility = !this.#flagFilesMenuVisibility;
		return this.#flagFilesMenuVisibility;
	}
}

export const recentFilesStore = new RecentFilesStore();
