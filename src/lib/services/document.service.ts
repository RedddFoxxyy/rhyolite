import type { RecentFileInfo } from "$lib/types/document";
import type { Tab } from "$lib/types/tab";
import { invoke } from "@tauri-apps/api/core";

// NOTE: This functioni will be soon changed or depreciated.
const getAllDocumentTabs = (): void => {
	invoke("exec_command", { cmd: "update_states" });;
};

const getDocumentContent = (Tab: Tab): void => {
	invoke("exec_command", {
		cmd: "get_document_content",
		payload: JSON.stringify({
			id: Tab.id,
			title: Tab.title
		})
	});
}

export const addNewDocumentTab = (): void => {
	try {
		invoke("exec_command", { cmd: "new_tab" });
	} catch (error) {
		console.error("Failed to create new document:", error);
	}
};

const deleteDocumentTab = (id: string): void => {
	invoke("exec_command", {
		cmd: "delete_document",
		payload: JSON.stringify(id)
	});
};

const initFrontendState = (): void => {
	invoke("exec_command", { cmd: "init_frontend_state" });
};

const saveDocument = ({
	documentId,
	documentTitle,
	documentContent
}: {
	documentId: string;
	documentTitle: string;
	documentContent: any;
}): void => {
	invoke("exec_command", {
		cmd: "save_document",
		payload: JSON.stringify({
			id: documentId,
			title: documentTitle,
			content: documentContent || ""
		})
	});
};

export const loadDocument = (file: RecentFileInfo): void => {
	invoke("exec_command", {
		cmd: "load_tab",
		payload: JSON.stringify({ id: file.id, title: file.title })
	});
}

export default {
	getAllDocumentTabs,
	getDocumentContent,
	addNewDocumentTab,
	deleteDocumentTab,
	initFrontendState,
	saveDocument,
	loadDocument
};
