import { type Writable, writable, get } from "svelte/store";
import type { Tab } from "$lib/types/tab";
import DocumentService from "$lib/services/document.service";

export interface ITabsStates {
  tabs: Tab[];
  currentTab: Tab | null;
}

const states: Writable<ITabsStates> = writable<ITabsStates>({
  tabs: [],
  currentTab: null,
});

const initTabsStore = async () => {
  const tabs = await DocumentService.getAllDocumentTabs();
  const currentTab = tabs.length > 0 ? tabs[0] : null;
  states.update(() => ({ tabs, currentTab }));
};

const resetCurrentTab = () => {
  const tabs: Tab[] = getTabsState();
  const currentTab: Tab | null = tabs.length > 0 ? tabs[0] : null;
  updateCurrentTabState(currentTab);
};

const updateTabsState = (tabs: Tab[]): Tab[] => {
  states.update((data) => ({
    tabs: tabs,
    currentTab: data.currentTab,
  }));
  return tabs;
};

const updateCurrentTabState = (currentTab: Tab | null): Tab | null => {
  states.update((data) => ({
    tabs: data.tabs,
    currentTab: currentTab,
  }));
  return currentTab;
};

const getTabById = (tabId: string): Tab | undefined => {
  const { tabs }: { tabs: Tab[] } = get(states); // Access the current state of states
  return tabs.find((tab) => tab.id === tabId); // Replace 'id' with the actual property name if it's different
};

const getCurrentTabState = (): Tab | null => {
  const { currentTab }: { currentTab: Tab | null } = get(states);
  return currentTab;
};

const getTabsState = (): Tab[] => {
  const { tabs }: { tabs: Tab[] } = get(states);
  return tabs;
};

export default {
  states,
  initTabsStore,
  resetCurrentTab,
  updateTabsState,
  updateCurrentTabState,
  getTabById,
  getCurrentTabState,
  getTabsState,
};
