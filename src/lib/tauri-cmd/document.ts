import type { RecentFileInfo } from "$lib/types/document";
import type { Tab } from "$lib/types/tab";
import { invoke } from "@tauri-apps/api/core";

// NOTE: This function will be soon changed or depreciated.
function getAllDocumentTabs() {
	invoke("exec_command", { cmd: "update_states" });
}

function getDocumentContent(Tab: Tab) {
	invoke("exec_command", {
		cmd: "get_document_content",
		payload: JSON.stringify({
			id: Tab.id,
			title: Tab.title
		})
	});
}

export function addNewDocumentTab() {
	try {
		invoke("exec_command", { cmd: "new_tab" });
	} catch (error) {
		console.error("Failed to create new document:", error);
	}
}

function deleteDocumentTab(id: string) {
	invoke("exec_command", {
		cmd: "delete_document",
		payload: JSON.stringify(id)
	});
}

function initFrontendState() {
	invoke("exec_command", { cmd: "init_frontend_state" });
}

function updateTabTitle(id: string, title: string) {
	invoke("exec_command", {
		cmd: "update_tab_title",
		payload: JSON.stringify({ id: id, title: title })
	});
}

function saveDocument(documentId: string, documentTitle: string, documentContent: any) {
	invoke("exec_command", {
		cmd: "save_document",
		payload: JSON.stringify({
			id: documentId,
			title: documentTitle,
			content: documentContent || ""
		})
	});
}

export function loadDocument(file: RecentFileInfo) {
	invoke("exec_command", {
		cmd: "load_tab",
		payload: JSON.stringify({ id: file.id, title: file.title })
	});
}

export function getRecentlyOpenedFiles() {
	invoke("exec_command", { cmd: "get_recent_files_metadata" });
}

export default {
	getAllDocumentTabs,
	getDocumentContent,
	addNewDocumentTab,
	deleteDocumentTab,
	initFrontendState,
	updateTabTitle,
	saveDocument,
	loadDocument,
	getRecentlyOpenedFiles
};
