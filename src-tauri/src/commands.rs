use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn exec_command(cmd: String, payload: serde_json::Value, app: AppHandle) {
    log::debug!("exec command '{}' called with '{}'", cmd, payload);
    log::debug!("payload is object: '{}'", payload.is_object());
    if payload.is_object() {
        let value = payload.as_object().unwrap().get("hi");
        log::debug!("value of hi: '{:?}'", value);
    }
    app.emit("dummy-event", "hellllo");
}
