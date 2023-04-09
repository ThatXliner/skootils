use learnatvcs;
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
