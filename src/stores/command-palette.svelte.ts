class CommandPaletteStore {
    #isVisible = $state(false);

    toggleVisibility() {
        this.#isVisible = !this.#isVisible;
    }

    isVisible() {
        return this.#isVisible;
    }
}

export const commandPaletteStore = new CommandPaletteStore();
