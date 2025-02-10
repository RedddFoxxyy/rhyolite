use tauri::{AppHandle, Manager};
use crate::app_state::{AppState, Tab};
use crate::editor::tabs::{update_tabs_state, TabCommands};

impl TabCommands{
    pub fn update_tab_title(app: AppHandle, payload: Option<String>) {
        log::debug!("update_tab_title init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to update_tab_title");
            return;
        };

        if let Ok(payload_object) = serde_json::from_str::<Tab>(&payload) {
            let temp_app = app.clone();
            let state = temp_app.state::<AppState>();

            let tabs = &mut state.tab_switcher.write().unwrap().tabs;
            let title = payload_object.title;
            let id = payload_object.id;

            // Check if the new title already exists in other tabs
            if tabs.values().any(|tab| tab.id != id && tab.title == title) {
                log::debug!("A tab with this title already exists.");
            } else {
                // Get the tab, update its title, and insert it back
                if let Some(mut tab) = tabs.get(&id).cloned() {
                    tab.title = title;
                    tabs.insert(id, tab.clone());
                } else {
                    log::debug!("Tab not found");
                }
            }
        }

        update_tabs_state(app);
    }
}