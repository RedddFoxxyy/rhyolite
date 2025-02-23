use crate::{
    app_state::{AppState, FileInfo, Tab, DEFAULT_NOTE_TITLE, TROVE_DIR},
    editor::{
        io::{get_trove_dir, save_document, save_user_data},
        tabs::{cleanup_deleted_files_workaround, update_tabs_state, TabCommands},
    },
    utils::generate_available_path,
};
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
            let tab_switcher_option = state.get_tab_switcher_mut();
            if tab_switcher_option.is_none() {
                log::error!("Failed to create a new tab!");
                return;
            }
            let mut tab_switcher = tab_switcher_option.unwrap();
            tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
            tab_switcher.current_tab_id = Some(new_id.clone());
        }

        {
            let workspace_option = state.get_workspace_mut();
            if workspace_option.is_none() {
                log::error!("Failed to add new tab to recent_files!");
                return;
            }
            let mut workspace = workspace_option.unwrap();

            workspace.recent_files.push(FileInfo {
                id: new_id.clone(),
                title: title.clone(),
            });
        }

        let _ = save_user_data(state);
        let _ = save_document(new_id, title, String::new(), state.to_owned());
        update_tabs_state(app);
    }
}
