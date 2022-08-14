use super::city::{City, CitySalaries, CityScores};
use super::weather::WeatherData;
use serde_json::Value;
pub struct CityFacts<'a> {
    pub population: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub geohash: String,
    pub country: String,
    pub timezone: String,
    pub full_name: String,
    pub geonameid: u64,
    pub scores: CityScores<'a>,
    pub salaries: CitySalaries<'a>,
    pub weather: WeatherData,
}

impl<'a> CityFacts<'a> {
    pub async fn new(city: &City) -> CityFacts<'a> {
        let url = city.urls.geo_name_id_url.as_str();
        let body = reqwest::get(url).await.unwrap();
        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();
        let lan = json["location"]["latlon"]["latitude"].as_f64().unwrap();
        let lon = json["location"]["latlon"]["longitude"].as_f64().unwrap();
        Self {
            weather: WeatherData::new(&lan, &lon).await,
            population: json["population"].as_u64().unwrap(),
            latitude: lan,
            longitude: lon,
            geohash: json["location"]["geohash"].as_str().unwrap().to_owned(),
            full_name: json["full_name"].as_str().unwrap().to_owned(),
            geonameid: json["geoname_id"].as_u64().unwrap(),
            country: json["_links"]["city:country"]["name"]
                .as_str()
                .unwrap()
                .to_owned(),
            timezone: json["_links"]["city:timezone"]["name"]
                .as_str()
                .unwrap()
                .to_owned(),
            scores: CityScores::new(&city.urls.score_url).await,
            salaries: CitySalaries::new(&city.urls.salary_url).await,
        }
    }
}
