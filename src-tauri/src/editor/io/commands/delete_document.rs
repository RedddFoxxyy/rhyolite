use crate::app_state::{AppState, Tab, TROVE_DIR};
use crate::editor::io::{get_document_content, get_trove_dir, save_user_data, IOCommands};
use crate::editor::tabs::update_tabs_state;
use std::fs;
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
    pub fn delete_document(app: AppHandle, _payload: Option<String>) {
        log::debug!("delete_document init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();
        let orig_state = &state;

        // Initialise Next Tab.
        let next_tab: Tab;

        //TODO: Handle the case where the current_tab_id can be none!
        let current_tab_id = state
            .tab_switcher
            .try_read()
            .unwrap()
            .current_tab_id
            .clone()
            .unwrap();

        // Get current_tab title(tab to be deleted).
        let current_tab_title = {
            let tabswitcher = state
                .tab_switcher
                .try_read()
                .unwrap_or_else(|err| panic!("Cannot access tab_switcher: {:?}", err));
            if !tabswitcher.tabs.contains_key(&current_tab_id) {
                log::debug!("Tab not found.");
                return;
            }

            let title = tabswitcher
                .tabs
                .get(&current_tab_id)
                .map(|tab| tab.title.clone())
                .unwrap_or_else(|| panic!("Tab title does not exist"));

            title
        };

        // Update tab switcher in a separate scope to avoid deadlocks
        // and get the next tab.
        {
            // TODO: Handle unwrap statements safely.
            let mut tabswitcher = state.tab_switcher.write().unwrap();
            next_tab = tabswitcher
                .tabs
                .shift_remove(&current_tab_id)
                .unwrap()
                .clone();
        }
        update_tabs_state(app.clone());

        // Handle file operations
        let trove_dir = get_trove_dir(TROVE_DIR);
        let filename = sanitize_filename::sanitize(format!("{}.md", current_tab_title));
        let file_path = trove_dir.join(&filename);

        if file_path.exists() {
            let _ = fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e));
        }

        // Remove the file in current_tab from the Recent Files in
        // a seperate scope to avoid deadlock.
        {
            let mut workspace = state.workspace.try_write().unwrap();
            workspace
                .recent_files
                .retain(|doc| doc.id != current_tab_id);
        }

        let _ = save_user_data(orig_state);

        // Get the DocumentData for the next tab
        let next_tab_data = {
            //TODO: Handle panic cases here when using unwrap.
            get_document_content(next_tab.id, next_tab.title).unwrap()
        };
        // TODO: Handle panic cases here when using unwrap.
        // update: for now I handled this by using an if let pattern
        // matcher to run the emit code only if tab is not none.
        if let Some(tab) = next_tab_data {
            let _ = app.emit("current_editor_content", tab.content);
        }
    }
}
