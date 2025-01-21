class RecentFilesStore {
  #isVisible = $state(false);

  isVisible(): boolean {
    return this.#isVisible;
  }

  toggleVisibility() {
    this.#isVisible = !this.#isVisible;
  }
}

export const recentFilesStore = new RecentFilesStore();
