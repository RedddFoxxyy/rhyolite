use crate::app_state::{AppState, Tab};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn exec_command(cmd: String, payload: serde_json::Value, app: AppHandle) {
    log::debug!("exec command '{}' called with '{}'", cmd, payload);
    log::debug!("payload is object: '{}'", payload.is_object());
    if payload.is_object() {
        let value = payload.as_object().unwrap().get("hi");
        log::debug!("value of hi: '{:?}'", value);
    }
    let _ = app.emit("dummy-event", "hellllo");
}

pub fn event_emitter(app: AppHandle, state: State<'_, AppState>) {
    let current_state = state.lock().unwrap();
    let current_tab_id;
    {
        let tabs: Vec<Tab> = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .tabs
            .values()
            .cloned()
            .collect();
        let _ = app.emit("Tabs", tabs);
    }
    {
        current_tab_id = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .current_tab_id
            .clone()
            .unwrap();
    }
    {
        let current_tab = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .tabs
            .get(&current_tab_id)
            .map(|tab| (*tab).clone())
            .unwrap();
        let _ = app.emit("Current_Tab", current_tab);
    }
}
