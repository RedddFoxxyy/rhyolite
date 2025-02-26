import type { Tab } from "../types/tab";
import { invoke } from "@tauri-apps/api/core";
import type { Document, RecentFileInfo } from "../types/document";
import type { IApiServiceProvider } from "./api.interface";

export class TauriInvokeServiceProvider implements IApiServiceProvider {
  async addNewDocumentTab(): Promise<Tab> {
    return await invoke<Tab>("new_tab");
  }

  async getAllDocumentTabs(): Promise<Tab[]> {
    return await invoke<Tab[]>("get_tabs");
  }

  async sendCurrentOpenTab(tabId: string) {
    await invoke("send_current_open_tab", { id: tabId });
  }

  async getDocumentContent(
    tabId: string,
    tabTitle: string,
  ): Promise<Document | null> {
    return await invoke<Document | null>("get_document_content", {
      id: tabId,
      title: tabTitle,
    });
  }

  async saveDocument({
    documentId,
    documentTitle,
    documentContent,
  }: {
    documentId: string;
    documentTitle: string;
    documentContent: string;
  }): Promise<void> {
    await invoke("save_document", {
      id: documentId,
      title: documentTitle,
      content: documentContent,
    });
    await invoke("update_tab_title", {
      id: documentId,
      title: documentTitle,
    });
  }

  async getLastOpenedTabs(): Promise<Document[]> {
    return await invoke<Document[]>("load_last_open_tabs");
  }

  getRecentlyOpenedFiles() {
    invoke("exec_command", { cmd: "get_recent_files_metadata" });
    //return await invoke<RecentFileInfo[]>("get_recent_files_metadata");
  }

  async loadTab({
    documentId,
    documentTitle,
  }: {
    documentId: string;
    documentTitle: string;
  }): Promise<void> {
    await invoke("load_tab", {
      id: documentId,
      title: documentTitle,
    });
  }

  async CloseCurrentTab(documentId: string) {
    await invoke("close_tab", { id: documentId });
  }

  async deleteDocument(documentId: string) {
    await invoke("delete_document", { id: documentId });
  }

  async getCurrentOpenTab(): Promise<string> {
    return await invoke("get_current_open_tab");
  }
}
