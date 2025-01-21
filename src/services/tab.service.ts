import type { Tab } from "../types/tab";
import {tabsStore} from "../stores/tabs.svelte";
import { ApiProvider } from "./api.service";
import docservice from "./document.service";

const apiProvider = new ApiProvider();

const switchTab = async (tabId: string): Promise<Tab | undefined> => {
    const tab: Tab | undefined = tabsStore.getTabById(tabId);
    if(!tab) return undefined;

    tabsStore.updateCurrentTab(tab ?? null);
    await apiProvider.sendCurrentOpenTab(tab.id);
    return tab;
};

const closeTab = async (tabId?: string) => {
    if (!tabId) return;
    try {
        const tabToClose: Tab | undefined = tabsStore.getTabById(tabId);
        if (!tabToClose) return;
        const currentTab: Tab | null = tabsStore.getCurrentTab();
        if (!currentTab) return;
        if (tabToClose.id === currentTab.id) {
            await apiProvider.CloseCurrentTab(currentTab.id);
            const tabs = await docservice.getAllDocumentTabs();
            if (tabs.length > 0) {
                const lastTab = tabs[tabs.length - 1];
                tabsStore.updateCurrentTab(lastTab); 
            } else {
                await docservice.addNewDocumentTab(); 
            }
        } else {
            await apiProvider.CloseCurrentTab(tabToClose.id);
            const tabs = await docservice.getAllDocumentTabs();
            tabsStore.updateCurrentTab(currentTab);
        }
    } catch (error) {
        console.error("Failed to delete document:", error);
    }
};


const gotoTab1 = async () => {
    const tabs: Tab[] = tabsStore.getTabs();
    if (tabs.length > 0) {
        await switchTab(tabs[0].id);
    }
};

const gotoLastTab = async ()=> {
    const tabs: Tab[] = tabsStore.getTabs();
    if (tabs.length > 0) {
        const lastTabIndex = tabs.length - 1;
        await switchTab(tabs[lastTabIndex].id);
    }
};

const cycleTabs = async () => {
    const tabs: Tab[] = tabsStore.getTabs();
    const currentTab: Tab | null = tabsStore.getCurrentTab();
    if (tabs.length > 0) {
        const currentTabIndex = tabs.findIndex(tab => tab.id === currentTab?.id);
        const nextTabIndex = (currentTabIndex + 1) % tabs.length;
        const nextTab = tabs[nextTabIndex];
        await switchTab(nextTab.id);
    }
}

const updateTabTitleById = async (tabId: string, newTitle: string) => {
    const tabs = tabsStore.getTabs();
    tabs.map(tab =>tab.id === tabId ? { ...tab, title: newTitle } : tab);
    tabsStore.updateTabs(tabs);
}

export default {
    switchTab,
    gotoTab1,
    gotoLastTab,
    cycleTabs,
    closeTab,
    updateTabTitleById,
}
