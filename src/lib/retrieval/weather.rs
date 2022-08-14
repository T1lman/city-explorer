use reqwest;
use serde_json::Value;
pub struct WeatherData {
    pub langitude: f64,
    pub longitude: f64,
    pub avg_24_h: f64,
}

impl WeatherData {
    pub async fn new(lan: &f64, lon: &f64) -> Self {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
            &lan, &lon
        );
        let body = reqwest::get(url).await.unwrap();
        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(text.as_str()).unwrap();
        let tempvec = json["hourly"]["temperature_2m"].as_array().unwrap();
        let mut temp: f64 = 0.0;
        for i in 0..24 {
            temp += tempvec[i].as_f64().unwrap();
        }
        temp = temp / 24.0;
        Self {
            langitude: lan.clone(),
            longitude: lon.clone(),
            avg_24_h: temp,
        }
    }
}
