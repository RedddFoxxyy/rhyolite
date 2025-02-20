use crate::app_state::AppState;
use crate::editor::io::{get_document_content, get_trove_dir, save_user_data, IOCommands};
use crate::editor::tabs::update_tabs_state;
use std::fs;
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
    ///TODO: The delete_document command needs to be worked on to support
    ///the new state management and remove all the legacy code.
    pub fn delete_document(app: AppHandle, _payload: Option<String>) {
        log::debug!("delete_document init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();
        let orig_state = &state;

        // let current_tab_id = {
        //     let tabswitcher = state.tab_switcher.read().unwrap();
        //     let current_tab_id = tabswitcher.current_tab_id.clone().unwrap();
        //     current_tab_id
        // };
        //TODO: Handle the case where the current_tab_id can be none!
        let current_tab_id = state
            .tab_switcher
            .read()
            .unwrap()
            .current_tab_id
            .clone()
            .unwrap();

        // Get the tab information in a separate scope
        let (tab_title, next_tab_info) = {
            let tabswitcher = state
                .tab_switcher
                .try_read()
                .unwrap_or_else(|err| panic!("Cannot read tab_switcher: {:?}", err));
            if !tabswitcher.tabs.contains_key(&current_tab_id) {
                log::debug!("Tab not found.");
                return;
            }

            let title = tabswitcher
                .tabs
                .get(&current_tab_id)
                .map(|tab| tab.title.clone())
                .unwrap_or_else(|| panic!("Tab title does not exist"));
            let next = tabswitcher
                .tabs
                .get_index(0)
                .or_else(|| tabswitcher.tabs.last())
                .map(|(next_id, next_tab)| (next_id.clone(), next_tab.title.clone()));

            (title, next)
        };

        // Update tab switcher in a separate scope
        {
            let mut tabswitcher = state.tab_switcher.write().unwrap();
            tabswitcher.tabs.shift_remove(&current_tab_id);
            if let Some((next_id, _)) = &next_tab_info {
                tabswitcher.current_tab_id = Some(next_id.clone());
            }
        }
        update_tabs_state(app.clone());

        // Handle file operations
        let trove_dir = get_trove_dir("Untitled_Trove");
        let filename = sanitize_filename::sanitize(format!("{}.md", tab_title));
        let file_path = trove_dir.join(&filename);

        if file_path.exists() {
            let _ = fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e));
        }

        // Update recent files in a separate scope to avoid deadlocks.
        {
            let mut workspace = state.workspace.write().unwrap();
            workspace
                .recent_files
                .retain(|doc| doc.id != current_tab_id);
        }

        let _ = save_user_data(orig_state);

        // Get the content for the next tab
        let next_tab = if let Some((next_id, next_title)) = next_tab_info {
            //TODO: Handle panic cases here when using unwrap.
            get_document_content(next_id, next_title).unwrap()
        } else {
            None
        };
        // TODO: Handle panic cases here when using unwrap.
        // update: for now I handled this by using an if let pattern
        // matcher to run the emit code only if tab is not none.
        if let Some(tab) = next_tab {
            let _ = app.emit("current_editor_content", tab.content);
        }
    }
}
