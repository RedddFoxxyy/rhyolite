use crate::{
    app_state::{AppState, TROVE_DIR},
    editor::{
        io::{get_document_content, get_trove_dir, save_user_data, IOCommands},
        tabs::update_tabs_state,
    },
};
use std::fs;
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
    pub fn delete_document(app: AppHandle, _payload: Option<String>) {
        log::debug!("delete_document init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();
        let orig_state = &state;

        let (next_tab, current_tab_id, current_tab_title) = {
            let tabswitcher_option = state.get_tab_switcher_mut();
            if tabswitcher_option.is_some() {
                let mut tabswitcher = tabswitcher_option.unwrap();

                //TODO: Handle the case where the current_tab_id can be none!
                let current_tab_id = tabswitcher.current_tab_id.clone().unwrap();
                if !tabswitcher.tabs.contains_key(&current_tab_id) {
                    log::debug!("Tab not found.");
                    return;
                }

                let current_tab_title = tabswitcher
                    .tabs
                    .get(&current_tab_id)
                    .map(|tab| tab.title.clone())
                    .unwrap_or_else(|| panic!("Tab title does not exist"));

                let next_tab = tabswitcher
                    .tabs
                    .shift_remove(&current_tab_id)
                    .unwrap()
                    .clone();

                (next_tab, current_tab_id, current_tab_title)
            } else {
                return;
            }
        };

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
