import type { Tab } from "../types/tab";
import TabsStore, { type ITabsStates } from "../stores/tabs.store";
import { ApiProvider } from "./api.service";
import docservice from "./document.service";
import { invoke } from "@tauri-apps/api/core";

const apiProvider = new ApiProvider();

const switchTab = async (tabId: string) => {
  // const tab: Tab | undefined = TabsStore.getTabById(tabId);
  // if (!tab) return undefined;

  // Invoke the switch_tab command
  invoke("exec_command", { 
    cmd: "switch_tab", 
    payload: JSON.stringify({ tabId }) 
  });

  // Update local store
  // TabsStore.updateCurrentTabState(tab);

  // return tab;
};

const closeTab = async (tabId?: string) => {
  if (!tabId) return;
  try {
    invoke("exec_command", { cmd: "close_tab", payload: JSON.stringify({ tabId }) });
    // const tabToClose: Tab | undefined = TabsStore.getTabById(tabId);
    // if (!tabToClose) return;
    // const currentTab: Tab | null = TabsStore.getCurrentTabState();
    // if (!currentTab) return;
    // if (tabToClose.id === currentTab.id) {
    //   await apiProvider.CloseCurrentTab(currentTab.id);
    //   const tabs = await docservice.getAllDocumentTabs();
    //   if (tabs.length > 0) {
    //     const lastTab = tabs[tabs.length - 1];
    //     TabsStore.updateCurrentTabState(lastTab);
    //   } else {
    //     await docservice.addNewDocumentTab();
    //   }
    // } else {
    //   await apiProvider.CloseCurrentTab(tabToClose.id);
    //   const tabs = await docservice.getAllDocumentTabs();
    //   TabsStore.updateCurrentTabState(currentTab);
    // }
    //invoke("update_states");
  } catch (error) {
    console.error("Failed to delete document:", error);
  }
};

const gotoTab1 = async () => {
  const tabs: Tab[] = TabsStore.getTabsState();
  if (tabs.length > 0) {
    await switchTab(tabs[0].id);
  }
};

const gotoLastTab = async () => {
  const tabs: Tab[] = TabsStore.getTabsState();
  if (tabs.length > 0) {
    const lastTabIndex = tabs.length - 1;
    await switchTab(tabs[lastTabIndex].id);
  }
};

const cycleTabs = async () => {
  const tabs: Tab[] = TabsStore.getTabsState();
  const currentTab: Tab | null = TabsStore.getCurrentTabState();
  if (tabs.length > 0) {
    const currentTabIndex = tabs.findIndex((tab) => tab.id === currentTab?.id);
    const nextTabIndex = (currentTabIndex + 1) % tabs.length;
    const nextTab = tabs[nextTabIndex];
    await switchTab(nextTab.id);
  }
};

const updateTabTitleById = async (tabId: string, newTitle: string) => {
  //  TabsStore.states.update((data: ITabsStates) => {
  //    return {
  //      ...data,
  //      tabs: data.tabs.map((tab) =>
  //        tab.id === tabId ? { ...tab, title: newTitle } : tab,
  //      ),
  //    };
  //  });
  invoke("exec_command", {
    cmd: "update_tab_title",
    payload: JSON.stringify({ id: tabId, title: newTitle }),
  });
};

export default {
  switchTab,
  gotoTab1,
  gotoLastTab,
  cycleTabs,
  closeTab,
  updateTabTitleById,
};
