use crate::{
    app_state::AppState,
    editor::tabs::{update_tabs_state, TabCommands},
};
use log::*;
use tauri::{AppHandle, Manager};

impl TabCommands {
    pub fn switch_tab(app: AppHandle, payload: Option<String>) {
        let Some(payload) = payload else {
            warn!("Invalid call to switch_tab");
            return;
        };
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
            if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();

                let tab_switcher_option = state.get_tab_switcher_mut();

                if tab_switcher_option.is_none() {
                    error!("Failed to switch tabs!");
                    return;
                }

                let mut tab_switcher = tab_switcher_option.unwrap();

                if tab_switcher.tabs.values().any(|tab| tab.id == tab_id) {
                    // Update current open tab if needed
                    tab_switcher.current_tab_id = Some(tab_id.to_string());
                }
            }
        }
        update_tabs_state(app);
    }
}
