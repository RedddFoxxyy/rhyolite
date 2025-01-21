import DocumentService from "../services/document.service";
import type { Tab } from '../types/tab';

class TabsStore {
    #tabs: Tab[] = $state([]);
    #currentTab: Tab | null = $state(null);

    constructor() {
        (async() => {
            this.#tabs = await DocumentService.getAllDocumentTabs();
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

        for (let tab of this.#tabs) {
           if (tab.id === currentTab?.id) tab = currentTab;
        }
        return this.#currentTab;
    }

    getTabById(tabId: string): Tab | undefined {
        return this.#tabs.find(tab => tab.id === tabId); // Replace 'id' with the actual property name if it's different
    }

    getCurrentTab(): Tab | null {
        return this.#currentTab;
    }

    getTabs(): Tab[] {
        return this.#tabs;
    }
}

export const tabsStore = new TabsStore();
