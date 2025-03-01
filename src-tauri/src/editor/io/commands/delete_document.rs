use crate::{
    app_state::{AppState, Tab, TROVE_DIR},
    editor::{
        io::{get_document_content, get_trove_dir, save_user_data, IOCommands},
        tabs::update_tabs_state,
    },
};
use std::fs;
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
    /// # Delete Document!
    ///
    /// Get's the current open tab and the next tab and deletes the file
    /// in the current tab and also removes it`s tab from app state.
    ///
    /// The next tab is the tab after the current tab(if it exists) or
    /// the tab before it if the above condition is not true.
    ///
    /// ___Example:(frontend)___
    /// ```
    /// invoke("exec_command", { cmd: "delete_document" });
    /// ```
    /// TODO: Delete the tab that is passed as payload rather than deleting
    /// the current open tab.
    pub fn delete_document(app: AppHandle, _payload: Option<String>) {
        log::debug!("delete_document init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();
        let orig_state = &state;

        let (next_tab, current_tab_id, current_tab_title): (Tab, String, String) = {
            let maybe_tab_switcher = state.get_tab_switcher_mut();
            if maybe_tab_switcher.is_none() {
                return;
            }
            let mut tab_switcher = maybe_tab_switcher.unwrap();

            //TODO: Handle the case where the current_tab_id can be none!
            let current_tab_id = tab_switcher.current_tab_id.clone().unwrap();
            if !tab_switcher.tabs.contains_key(&current_tab_id) {
                log::debug!("Tab not found.");
                return;
            }

            let current_tab_title = tab_switcher
                .tabs
                .get(&current_tab_id)
                .map(|tab| tab.title.clone())
                .unwrap_or_else(|| panic!("Tab title does not exist"));

            let next_tab_index = tab_switcher
                .tabs
                .shift_remove_full(&current_tab_id)
                .unwrap()
                .0;

            let next_tab: Tab;

            // TODO: Allow deletion of only remaining tab too, the editor should also be
            // able to handle no open tabs.
            if let Some(next_tab_kv) = tab_switcher.tabs.get_index(next_tab_index) {
                next_tab = next_tab_kv.1.clone();
            } else {
                if tab_switcher.tabs.is_empty() {
                    log::info!("Cannot delete the only open tab(this will be fixed in future).");
                    return;
                }
                next_tab = tab_switcher
                    .tabs
                    .get_index(next_tab_index - 1)
                    .unwrap()
                    .1
                    .clone();
            }

            tab_switcher.current_tab_id = Some(next_tab.id.clone());

            (next_tab, current_tab_id, current_tab_title)
        };

        update_tabs_state(app.clone());

        // Remove the file in current_tab from the Recent Files in
        // a seperate scope to avoid deadlock.
        {
            let maybe_workspace = state.get_workspace_mut();
            if maybe_workspace.is_none() {
                return;
            }
            maybe_workspace
                .unwrap()
                .recent_files
                .retain(|doc| doc.id != current_tab_id);
        }

        // Handle file operations
        let trove_dir = get_trove_dir(TROVE_DIR);
        let filename = sanitize_filename::sanitize(format!("{}.md", current_tab_title));
        let file_path = trove_dir.join(&filename);

        if file_path.exists() {
            let _ = fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e));
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
