// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn ma_se_poes() -> String {
    format!("You are a big ma se poes")
}
#[tauri::command]
fn read_file(path: &str) -> String {
  let data = std::fs::read(path).unwrap();
    String::from_utf8(data).unwrap()
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, ma_se_poes, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
