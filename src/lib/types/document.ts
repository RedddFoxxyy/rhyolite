export interface MarkdownFileData {
	id: string;
	title: string;
	content: string | any;
}

export interface TabDocument {
	path: string;
	title: string;
	content: string;
}

export interface RecentFileInfo {
	id: string;
	title: string;
}
