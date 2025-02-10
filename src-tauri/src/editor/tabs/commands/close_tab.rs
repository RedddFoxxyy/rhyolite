use tauri::{AppHandle, Manager};
use crate::app_state::AppState;
use crate::editor::tabs::{update_tabs_state, TabCommands};

impl TabCommands{
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

        // Get next tab ID in a separate scope to minimize lock time
        let next_tab_id = {
            let tab_switcher = state.tab_switcher.read().unwrap();
            let tabs = &tab_switcher.tabs;

            if tabs.is_empty() {
                return; // Don't close the last tab
            }

            if let Some((index, _, _)) = tabs.get_full(tab_id) {
                // Get the next tab ID (either at same index or last tab)
                tabs.get_index(index + 1)
                    .or_else(|| tabs.last())
                    .map(|(id, _)| id.clone())
            } else {
                None
            }
        };

        // Update tabs in a separate lock scope
        {
            let mut tab_switcher = state.tab_switcher.write().unwrap();
            tab_switcher.tabs.shift_remove(tab_id);

            // Update current open tab if needed
            if let Some(next_id) = next_tab_id {
                tab_switcher.current_tab_id = Some(next_id);
            }
        }

        // Call event emitter after releasing the lock
        update_tabs_state(app);
    }
}