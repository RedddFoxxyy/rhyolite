class SettingsMenuStore {
  #flagSettingsMenuVisibility: boolean = $state(false);

  isVisible(): boolean {
    return this.#flagSettingsMenuVisibility;
  }

  toggleVisibility(): boolean {
    this.#flagSettingsMenuVisibility = !this.#flagSettingsMenuVisibility;
    return this.#flagSettingsMenuVisibility;
  }
}

export const settingsMenuStore = new SettingsMenuStore();
