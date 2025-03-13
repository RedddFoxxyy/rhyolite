use crate::{
    app_state::{AppState, UserData},
    editor::{
        io::{IOCommands, get_documents_dir},
        tabs::update_tabs_state,
    },
};
use std::fs;
use tauri::{AppHandle, Manager};

impl IOCommands {
    pub async fn load_last_open_tabs(app: AppHandle, _payload: Option<String>) {
        log::debug!("load_last_open_tabs init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();

        let appdata_dir = get_documents_dir().join("appdata");
        let userdata_path = appdata_dir.join("userdata.json");

        if userdata_path.exists() {
            match fs::read_to_string(&userdata_path) {
                Ok(content) => match serde_json::from_str::<UserData>(&content) {
                    Ok(user_data) => {
                        // Update workspace in a separate scope
                        {
                            let mut tab_switcher = state.tab_switcher.write().await;
                            let mut workspace = state.workspace.write().await;

                            workspace.recent_files = user_data.recent_files.clone();

                            tab_switcher.current_tab_id = Some(user_data.last_open_tab.clone());

                            // Clear existing tabs and load from user_data
                            let tabs = &mut tab_switcher.tabs;
                            tabs.clear();
                            //tabswitcher.tabs = user_data.tabs.clone();
                            for tab in user_data.tabs {
                                tab_switcher.tabs.insert(tab.id.clone(), tab.clone());
                            }
                        }
                    }
                    Err(e) => log::debug!("{}", format!("Failed to deserialize userdata: {}", e)),
                },
                Err(e) => log::debug!("{}", format!("Failed to read userdata file: {}", e)),
            }
        }
        update_tabs_state(app.clone()).await;
    }
}
