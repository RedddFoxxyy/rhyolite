import { invoke } from "@tauri-apps/api/core";
import type { IApiServiceProvider } from "./api.interface";

export class TauriInvokeServiceProvider implements IApiServiceProvider {

	async saveDocument({
		documentId,
		documentTitle,
		documentContent
	}: {
		documentId: string;
		documentTitle: string;
		documentContent: string;
	}): Promise<void> {
		await invoke("save_document", {
			id: documentId,
			title: documentTitle,
			content: documentContent
		});
		await invoke("update_tab_title", {
			id: documentId,
			title: documentTitle
		});
	}

	getRecentlyOpenedFiles() {
		invoke("exec_command", { cmd: "get_recent_files_metadata" });
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
