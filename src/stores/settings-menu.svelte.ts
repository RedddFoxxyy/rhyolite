class SettingsMenuStore {
  #isVisible = $state(false);

  isVisible(): boolean {
    return this.#isVisible;
  }

  toggleVisibility() {
    this.#isVisible = !this.#isVisible;
  }
}

export const settingsMenuStore = new SettingsMenuStore();
