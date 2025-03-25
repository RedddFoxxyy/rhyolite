use crate::{
	app_state::AppState,
	editor::{
		io::commands::get_document_content::send_document_content,
		tabs::{TabCommands, update_tabs_state},
	},
};
use log::*;
use tauri::{AppHandle, Manager};

impl TabCommands {
	pub async fn cycle_tabs(app: AppHandle, _payload: Option<String>) {
		debug!("Init Cycle Tabs");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let mut tab_switcher = state.tab_switcher.write().await;

		let current_tab_id = tab_switcher.current_tab_id.clone();

		if current_tab_id.is_none() {
			error!("No tab open!");
			return;
		}

		let current_tab_index = tab_switcher.tabs.get_index_of(&current_tab_id.unwrap());
		if current_tab_index.is_none() {
			error!("Invalid current tab, cannot cycle through tabs!!");
			return;
		}
		let next_tab_index = current_tab_index.unwrap() + 1;
		let next_tab_id: String = {
			let maybe_tab = tab_switcher.tabs.get_index(next_tab_index);
			if let Some(tab) = maybe_tab {
				tab.0.clone()
			} else {
				// TODO: Handle the last tab logic, that is no next tab.
				// It will cycle to the first tab in this case.
				tab_switcher.tabs.get_index(0).unwrap().0.clone()
				// return;
			}
		};

		if tab_switcher.tabs.values().any(|tab| tab.id == next_tab_id) {
			tab_switcher.current_tab_id = Some(next_tab_id.clone());
		}
		let current_tab_data = tab_switcher.tabs.get(&next_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app);
	}

	pub async fn goto_tab_1(app: AppHandle, _payload: Option<String>) {
		debug!("Init goto_tab_1");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let mut tab_switcher = state.tab_switcher.write().await;

		if tab_switcher.tabs.is_empty() {
			info!("No tab open in the workspace.");
			return;
		}
		let first_tab_id: String = tab_switcher.tabs.get_index(0).unwrap().0.clone();

		if tab_switcher.tabs.values().any(|tab| tab.id == first_tab_id) {
			// Update current open tab if needed
			tab_switcher.current_tab_id = Some(first_tab_id.clone());
		}

		let current_tab_data = tab_switcher.tabs.get(&first_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app);
	}

	pub async fn goto_last_tab(app: AppHandle, _payload: Option<String>) {
		debug!("Init goto_last_tab");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let mut tab_switcher = state.tab_switcher.write().await;

		if tab_switcher.tabs.is_empty() {
			info!("No tab open in the workspace.");
			return;
		}

		// NOTE: The getting of last tab may be optimised.
		let last_tab_entry = tab_switcher.tabs.last_entry();
		if last_tab_entry.is_none() {
			error!("Failed to get last tab entry!");
			return;
		}

		let last_tab_id = last_tab_entry.unwrap().get().id.clone();

		if tab_switcher.tabs.values().any(|tab| tab.id == last_tab_id) {
			// Update current open tab if needed
			tab_switcher.current_tab_id = Some(last_tab_id.clone());
		}

		let current_tab_data = tab_switcher.tabs.get(&last_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app);
	}
}
