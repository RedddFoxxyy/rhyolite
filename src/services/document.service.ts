import type { Tab } from "../types/tab";
import tabsStore from "../stores/tabs.store";
import { ApiProvider } from "./api.service";
import TabService from "./tab.service";
import type { Document } from "../types/document";
import { isValidJSON } from "../helpers/common.helper";
import { invoke } from "@tauri-apps/api/core";

const apiProvider = new ApiProvider();

const getAllDocumentTabs = async (): Promise<Tab[]> => {
  const tabs: Tab[] = await apiProvider.getAllDocumentTabs();
  invoke("exec_command", { cmd: "update_states" });
  return tabsStore.updateTabsState(tabs);
};

export const addNewDocumentTab = (): void => {
  try {
    invoke("exec_command", { cmd: "new_tab" });
  } catch (error) {
    console.error("Failed to create new document:", error);
  }
};

const deleteDocumentTab = (id: string): void => {
  // invoke("exec_command", { cmd: "delete_document" });

  invoke("exec_command", {
    cmd: "delete_document",
    payload: JSON.stringify(id),
  });
};

const loadRecentDocuments = (): void => {
  invoke("exec_command", { cmd: "load_last_open_tabs" });
};

const saveDocument = async ({
  documentId,
  documentTitle,
  documentContent,
}: {
  documentId: string;
  documentTitle: string;
  documentContent: any;
}): Promise<void> => {
  invoke("exec_command", {
    cmd: "save_document",
    payload: JSON.stringify({
      id: documentId,
      title: documentTitle,
      content: documentContent || "",
    }),
  });
};

const loadDocument = async (
  documentId: string,
  documentTitle: string,
): Promise<Document | null> => {
  try {
    const doc = await apiProvider.getDocumentContent(documentId, documentTitle);
    if (!doc) return null;

    invoke("exec_command", { cmd: "update_states" });
    return doc;
  } catch (error) {
    console.error("Failed to load document:", error);
    return null;
  }
};

export const runDummyCommand = async (payload: Record<string, any>) => {
  invoke("exec_command", payload);
};

export default {
  getAllDocumentTabs,
  addNewDocumentTab,
  deleteDocumentTab,
  loadRecentDocuments,
  saveDocument,
  loadDocument,
};
