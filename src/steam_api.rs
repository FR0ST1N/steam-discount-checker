use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub title: String,
    pub savings: String,
    pub is_on_sale: String,
    #[serde(rename(deserialize = "steamAppID"))]
    pub steam_app_id: String,
}

pub fn get_data(steam_app_id: String) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let api_url = "https://www.cheapshark.com/api/1.0/deals?storeID=1&&steamAppID=".to_string()
        + &steam_app_id;
    let resp: Vec<Game> = reqwest::blocking::get(api_url)?.json()?;
    Ok(resp)
}
