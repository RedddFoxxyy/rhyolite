class CommandPaletteStore {
  #flagCommandPaletteVisibility: boolean = $state(false);

  isVisible(): boolean {
    return this.#flagCommandPaletteVisibility;
  }

  toggleVisibility(): boolean {
    this.#flagCommandPaletteVisibility = !this.#flagCommandPaletteVisibility;
    return this.#flagCommandPaletteVisibility;
  }
}

export const commandPaletteStore = new CommandPaletteStore();
