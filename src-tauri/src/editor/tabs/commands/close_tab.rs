use crate::{
    app_state::AppState,
    editor::tabs::{update_tabs_state, TabCommands},
};
use tauri::{AppHandle, Manager};

impl TabCommands {
    pub fn close_tab(app: AppHandle, payload: Option<String>) {
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
            let tab_switcher_option = state.get_tab_switcher_mut();
            if tab_switcher_option.is_none() {
                log::error!("Failed to close tab!");
                return;
            }
            let mut tab_switcher = tab_switcher_option.unwrap();
            let tabs = &tab_switcher.tabs;

            // Do not delete the only remaining tab.
            if tabs.len() == 1 {
                return;
            }

            // Assuming that there is a next tab that exists!
            let next_tab = tab_switcher.tabs.shift_remove(tab_id).unwrap();

            let next_tab_id = next_tab.id;

            tab_switcher.current_tab_id = Some(next_tab_id);
        }

        // Call event emitter after releasing the lock
        update_tabs_state(app);
    }
}
