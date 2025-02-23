use crate::{
    app_state::{AppState, Tab},
    editor::tabs::{update_tabs_state, TabCommands},
};
use tauri::{AppHandle, Manager};

impl TabCommands {
    pub fn load_tab(app: AppHandle, payload: Option<String>) {
        log::debug!("load_tab init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to load_tab");
            return;
        };
        if let Ok(json_value) = serde_json::from_str::<Tab>(&payload) {
            {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();
                let id = json_value.id;
                let title = json_value.title;
                let new_tab = Tab {
                    id: id.clone(),
                    title: title.clone(),
                };

                let tab_switcher_option = state.get_tab_switcher_mut();
                if tab_switcher_option.is_none() {
                    log::debug!("Failed to load Tab with documet {}", title);
                    return;
                }
                let mut tab_switcher = tab_switcher_option.unwrap();
                tab_switcher.tabs.insert(id, new_tab.clone());
            }

            update_tabs_state(app);
        }
    }
}
