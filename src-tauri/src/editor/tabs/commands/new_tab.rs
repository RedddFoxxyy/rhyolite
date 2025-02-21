use crate::app_state::{AppState, FileInfo, Tab, DEFAULT_NOTE_TITLE, TROVE_DIR};
use crate::editor::io::{get_trove_dir, save_document, save_user_data};
use crate::editor::tabs::{cleanup_deleted_files_workaround, update_tabs_state, TabCommands};
use crate::utils::generate_available_path;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

impl TabCommands {
    pub fn new_tab(app: AppHandle, _payload: Option<String>) {
        log::debug!("new_tab init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();

        let new_id = Uuid::new_v4().to_string();

        let trove_dir = get_trove_dir(TROVE_DIR);

        let new_path =
            generate_available_path(trove_dir.join(String::from(DEFAULT_NOTE_TITLE) + ".md"));
        let title = new_path.file_stem().unwrap().to_string_lossy().to_string();

        cleanup_deleted_files_workaround(state, trove_dir, &title);

        // Create new tab
        let new_tab = Tab {
            id: new_id.clone(),
            title: title.clone(),
        };

        // Insert into IndexMap
        {
            let mut tab_switcher = state.tab_switcher.write().unwrap();
            tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
            tab_switcher.current_tab_id = Some(new_id.clone());
        }

        {
            state
                .workspace
                .write()
                .unwrap()
                .recent_files
                .push(FileInfo {
                    id: new_id.clone(),
                    title: title.clone(),
                });
        }

        let _ = save_user_data(state);
        let _ = save_document(new_id, title, String::new(), state.to_owned());
        update_tabs_state(app);
    }
}
