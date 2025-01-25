import { apiProvider } from '../services/api.service'; 
import type { Tab } from '../types/tab';


class TabsStore {
    #tabs: Tab[] = $state([]);
    #currentTab: Tab | null = $state(null);

    constructor() {
        (async() => {
            this.#tabs = await apiProvider.getAllDocumentTabs();
        })();

        this.#currentTab = this.#tabs.length > 0 ? this.#tabs[0] : null;
    }

    resetCurrentTab() {
        this.#tabs = this.getTabs();
        this.#currentTab = this.#tabs.length > 0 ? this.#tabs[0] : null;
    }

    updateTabs(tabs: Tab[]): Tab[] {
        this.#tabs = tabs;
        return this.#tabs;
    }

    updateCurrentTab(currentTab: Tab | null): Tab | null {
        this.#currentTab = currentTab;
        return this.#currentTab;
    }

    getTabById(tabId: string): Tab | undefined {
        return this.#tabs.find(tab => tab.id === tabId); // Replace 'id' with the actual property name if it's different
    }

    getCurrentTab(): Tab | null {
        return this.#currentTab;
    }

    _markDeleting(tabId: string) {
      const tab = this.getTabById(tabId)
      if (!tab) {
        throw new Error(`Tab doesn't exist: "${tabId}"`)
      }
      tab.deleting = true
    }

    getTabs(): Tab[] {
        return this.#tabs;
    }
}

export const tabsStore = new TabsStore();
