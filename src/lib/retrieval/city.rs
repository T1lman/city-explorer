use reqwest;
use serde_json::Value;
use tui::style::Color;
use tui::{style::Style, text::Text};
#[derive(Debug, Clone)]
pub struct City {
    pub query: String,
    pub urls: CityUrls,
}

impl City {
    pub async fn new(query: String) -> Self {
        let urls = CityUrls::new(&query).await;
        Self { query, urls }
    }
}

#[derive(Debug, Clone)]
pub struct CityUrls {
    pub geo_name_id_url: String,
    pub base_urban_url: String,
    pub salary_url: String,
    pub score_url: String,
}

impl CityUrls {
    async fn new(query: &String) -> Self {
        //get geonameid
        let url = format!("https://api.teleport.org/api/cities/?search={}", query);
        let body = reqwest::get(url).await.unwrap();
        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();

        let base_json =
            json["_embedded"]["city:search-results"][0]["_links"]["city:item"]["href"].clone();
        let base = base_json.as_str().unwrap();

        //get urbanarea url
        let body = reqwest::get(base).await.unwrap();
        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();

        let urban_json = json["_links"]["city:urban_area"]["href"].clone();
        let urban = urban_json.as_str().unwrap();
        let salaries = format!("{urban}salaries/");
        let scores = format!("{urban}scores/");
        CityUrls {
            geo_name_id_url: base.to_owned(),
            base_urban_url: urban.to_owned(),
            salary_url: salaries,
            score_url: scores,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CityScores<'a> {
    pub data: Text<'a>,
}

#[derive(Debug, Clone)]
pub struct CitySalaries<'a> {
    pub data: Text<'a>,
}

impl<'a> CityScores<'a> {
    pub async fn new(url: &String) -> CityScores<'a> {
        let body = reqwest::get(url).await.unwrap();

        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();

        let categories = json["categories"].as_array().unwrap().clone();
        let mut result = Text::raw("");
        for scores in categories {
            let name = scores["name"].as_str().unwrap();
            let score = scores["score_out_of_10"].as_f64().unwrap();
            let scorecolor = if score <= 4.0 {
                Color::Red
            } else if score <= 7.0 {
                Color::Yellow
            } else {
                Color::Green
            };
            let scoreformat = format!("{score}/10");
            let scoretext = Text::styled(format!("{scoreformat}"), Style::default().fg(scorecolor));
            let mut restext = Text::raw(format!("{}: ", name));
            restext.extend(scoretext);

            result.extend(restext)
        }

        Self { data: result }
    }
}

impl<'a> CitySalaries<'a> {
    pub async fn new(url: &String) -> CitySalaries<'a> {
        let body = reqwest::get(url).await.unwrap();

        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();

        let categories = json["salaries"].as_array().unwrap().clone();
        let mut result = Text::raw("");
        for job in categories {
            let salary = format!(
                "{:.1}$",
                job["salary_percentiles"]["percentile_50"].as_f64().unwrap()
            );
            let salarycolor = Color::Green;
            let salarytext = Text::styled(format!("{}", salary), Style::default().fg(salarycolor));
            let name = job["job"]["title"].as_str().unwrap();
            let mut restext = Text::raw(format!("{}: ", name));
            restext.extend(salarytext);
            result.extend(restext)
        }
        Self { data: result }
    }
}
