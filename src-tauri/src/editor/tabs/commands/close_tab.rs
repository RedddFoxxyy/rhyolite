use crate::{
    app_state::{AppState, Tab},
    editor::tabs::{TabCommands, update_tabs_state},
};
use tauri::{AppHandle, Manager};

impl TabCommands {
    pub async fn close_tab(app: AppHandle, payload: Option<String>) {
        log::debug!("close_tab init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to close_tab");
            return;
        };

        // Parse the JSON payload
        let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) else {
            log::debug!("Failed to parse JSON payload");
            return;
        };

        let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) else {
            log::debug!("Invalid JSON payload format: missing or invalid tabId field");
            return;
        };

        let temp_app = app.clone();
        let state = temp_app.state::<AppState>();

        // Close the tab and switch to the previous tab in a seperate scope
        // to avoid deadlocks!
        {
            let maybe_tab_switcher = state.get_tab_switcher_mut();
            if maybe_tab_switcher.is_none() {
                log::error!("Failed to close tab!");
                return;
            }
            let mut tab_switcher = maybe_tab_switcher.unwrap();
            let tabs = &tab_switcher.tabs;

            // Do not close the only remaining tab. This will be removed in future..
            if tabs.len() == 1 {
                return;
            }

            let next_tab_index = tab_switcher.tabs.shift_remove_full(tab_id).unwrap().0;

            let next_tab: Tab;

            if let Some(next_tab_kv) = tab_switcher.tabs.get_index(next_tab_index) {
                next_tab = next_tab_kv.1.clone();
            } else {
                next_tab = tab_switcher
                    .tabs
                    .get_index(next_tab_index - 1)
                    .unwrap()
                    .1
                    .clone();
            }

            let next_tab_id = next_tab.id;

            tab_switcher.current_tab_id = Some(next_tab_id);
        }

        // Call event emitter after releasing the lock
        update_tabs_state(app);
    }
}
