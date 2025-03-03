use crate::{
    app_state::{AppState, DocumentData, Tab, DEFAULT_NOTE_TITLE, TROVE_DIR},
    editor::{
        io::{commands::save_document::save_document_helper, get_trove_dir, save_user_data},
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
            let maybe_tab_switcher = state.get_tab_switcher_mut();
            if maybe_tab_switcher.is_none() {
                log::error!("Failed to create a new tab!");
                return;
            }
            let mut tab_switcher = maybe_tab_switcher.unwrap();
            tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
            tab_switcher.current_tab_id = Some(new_id.clone());
        }

        // NOTE: I commented this block of code as save_document_helper
        // already handles updating the recent files with the proper
        // document path.
        // {
        //     let maybe_workspace = state.get_workspace_mut();
        //     if maybe_workspace.is_none() {
        //         log::error!("Failed to add new tab to recent_files!");
        //         return;
        //     }
        //     let mut workspace = maybe_workspace.unwrap();

        //     workspace.recent_files.push(FileInfo {
        //         id: new_id.clone(),
        //         title: title.clone(),
        //     });
        // }

        let _ = save_user_data(state);
        let save_document_data = DocumentData {
            id: new_id,
            title,
            content: String::new(),
        };
        save_document_helper(state, save_document_data);
        update_tabs_state(app);
    }
}
