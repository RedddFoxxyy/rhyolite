class SettingsMenuStore {
	#isVisible: boolean = $state(false);

	isVisible(): boolean {
		return this.#isVisible;
	}

	toggleVisibility(): boolean {
		this.#isVisible = !this.#isVisible;
		return this.#isVisible;
	}
}

export const settingsMenuStore = new SettingsMenuStore();
