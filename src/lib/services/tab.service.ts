import type { Tab } from "$lib/types/tab";
import { ApiProvider } from "./api.service";
import docservice from "./document.service";
import { invoke } from "@tauri-apps/api/core";

const apiProvider = new ApiProvider();

// TODO: Let the backend exec loading of document too!
const switchTab = (tab: Tab) => {
	// Invoke the switch_tab command
	invoke("exec_command", {
		cmd: "switch_tab",
		payload: JSON.stringify({ tabId: tab.id })
	});
	// invoke("exec_command", {
	//   cmd: "get_document_content",
	//   payload: JSON.stringify({ id: tab.id, title: tab.title }),
	// });
};

const closeTab = (tabId?: string) => {
	if (!tabId) return;
	try {
		invoke("exec_command", {
			cmd: "close_tab",
			payload: JSON.stringify({ tabId })
		});
	} catch (error) {
		console.error("Failed to delete document:", error);
	}
};

const gotoTab1 = async () => {
	invoke("exec_command", { cmd: "goto_tab_1" });
};

const gotoLastTab = async () => {
	invoke("exec_command", { cmd: "goto_last_tab" });
};

const cycleTabs = async () => {
	invoke("exec_command", { cmd: "cycle_tabs" });
};

const updateTabTitleById = (tabId: string, newTitle: string) => {
	invoke("exec_command", {
		cmd: "update_tab_title",
		payload: JSON.stringify({ id: tabId, title: newTitle })
	});
};

export default {
	switchTab,
	gotoTab1,
	gotoLastTab,
	cycleTabs,
	closeTab,
	updateTabTitleById
};
