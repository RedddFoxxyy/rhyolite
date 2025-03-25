export interface IApiServiceProvider {
	saveDocument({
		documentId,
		documentTitle,
		documentContent
	}: {
		documentId: string;
		documentTitle: string;
		documentContent: any;
	}): void;

	getRecentlyOpenedFiles(): void;

	deleteDocument(documentId: string): Promise<void>;
}
