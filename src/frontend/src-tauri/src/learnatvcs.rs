use learnatvcs;
use serde_json::json;
#[tauri::command]
pub async fn scrape_plans(
    username: String,
    password: String,
    quarter: learnatvcs::TargetQuarter,
    dates: learnatvcs::TargetDate,
) -> Result<learnatvcs::Output, String> {
    return learnatvcs::scrape(username, password, quarter, dates)
        .await
        .map_err(|err| err.to_string());
}
#[tauri::command]
pub async fn get_credentials() -> Option<String> {
    let entry = keyring::Entry::new("skootils", "learnatvcs");
    return entry.get_password().ok();
}
#[tauri::command]
pub async fn set_credentials(username: String, password: String) -> Result<(), String> {
    let entry = keyring::Entry::new("skootils", "learnatvcs");
    return entry
        .set_password(&json!({"username": username, "password": password}).to_string())
        .or(Err(String::from("Could not set username and password")));
}
