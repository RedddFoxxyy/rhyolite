use std::fs;

use crate::{
    app_state::{AppState, Tab, TROVE_DIR},
    editor::{
        io::get_trove_dir,
        tabs::{update_tabs_state, TabCommands},
    },
};
use tauri::{AppHandle, Manager};

impl TabCommands {
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
            let new_title = payload_object.title;
            let id = payload_object.id;

            let mut old_title = None;

            // Check if the new title already exists in other tabs
            if tabs
                .values()
                .any(|tab| tab.id != id && tab.title == new_title)
            {
                log::debug!("A tab with this title already exists.");
            } else {
                // Get the tab, update its title, and insert it back
                if let Some(mut tab) = tabs.get(&id).cloned() {
                    old_title = Some(tab.title.clone());
                    tab.title = new_title;
                    tabs.insert(id, tab.clone());
                } else {
                    log::debug!("Tab not found");
                }
            }

            // Delete the file with old title, if old title exists.
            if old_title.is_some() {
                let trove_dir = get_trove_dir(TROVE_DIR);
                let filename = sanitize_filename::sanitize(format!("{}.md", old_title.unwrap()));
                let file_path = trove_dir.join(&filename);

                if file_path.exists() {
                    let _ = fs::remove_file(&file_path).map_err(|e| {
                        format!("Failed to delete file {}: {}", file_path.display(), e)
                    });
                }
            }
        }

        update_tabs_state(app);
    }
}
