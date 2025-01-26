import type { Tab } from "../types/tab";
import type { Document, RecentFileInfo } from "../types/document";
import type { Settings } from "../types/settings";
import type { get } from "svelte/store";

export interface IApiServiceProvider {
  addNewDocumentTab(): Promise<Tab>;

  getAllDocumentTabs(): Promise<Tab[]>;

  sendCurrentOpenTab(tabId: string): Promise<void>;

  getDocumentContent(tabId: string, tabTitle: string): Promise<Document | null>;

  saveDocument({
    documentId,
    documentTitle,
    documentContent,
  }: {
    documentId: string;
    documentTitle: string;
    documentContent: any;
  }): Promise<void>;

  getLastOpenedTabs(): Promise<Document[]>;

  getRecentlyOpenedFiles(): Promise<RecentFileInfo[]>;

  loadTab({
    documentId,
    documentTitle,
  }: {
    documentId: string;
    documentTitle: string;
  }): Promise<void>;

  deleteDocument(documentId: string): Promise<void>;

  get_all_settings(): Promise<Settings>;  
}
