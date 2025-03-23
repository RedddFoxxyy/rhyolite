import type { Tab } from "$lib/types/tab";
import type { Document } from "$lib/types/document";

export interface IApiServiceProvider {
	addNewDocumentTab(): Promise<Tab>;

	getAllDocumentTabs(): Promise<Tab[]>;

	sendCurrentOpenTab(tabId: string): Promise<void>;

	getDocumentContent(tabId: string, tabTitle: string): Promise<Document | null>;

	saveDocument({
		documentId,
		documentTitle,
		documentContent
	}: {
		documentId: string;
		documentTitle: string;
		documentContent: any;
	}): void;

	getLastOpenedTabs(): Promise<Document[]>;

	getRecentlyOpenedFiles(): void;

	loadTab({ documentId, documentTitle }: { documentId: string; documentTitle: string }): void;

	deleteDocument(documentId: string): Promise<void>;
}
