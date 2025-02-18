#[tauri::command]
pub fn parse_img() {
    println!("I was invoked from JavaScript!");
}