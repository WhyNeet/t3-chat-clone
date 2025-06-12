use serde::{Deserialize, Serialize};

pub struct WebSearch {
    key: String,
}

impl WebSearch {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub async fn search(&self, query: String) -> anyhow::Result<SerperResult> {
        let client = reqwest::Client::builder().build()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-KEY", self.key.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        let data = SerperRequest {
            q: query,
            gl: "us".to_string(),
            hl: "en".to_string(),
        };

        let request = client
            .request(reqwest::Method::POST, "https://google.serper.dev/search")
            .headers(headers)
            .json(&data);

        let response = request.send().await?;
        let body: SerperResult = response.json().await?;

        Ok(body)
    }
}

#[derive(Debug, Serialize)]
pub struct SerperRequest {
    q: String,
    gl: String,
    hl: String,
}

#[derive(Debug, Deserialize)]
pub struct SerperResult {
    pub organic: Vec<OrganicResult>,
    // #[serde(rename = "topStories")]
    // pub top_stories: Option<Vec<Story>>,
    // #[serde(rename = "peopleAlsoAsk")]
    // pub people_ask: Vec<PeopleQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct OrganicResult {
    pub title: String,
    pub snippet: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct Story {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct PeopleQuestion {
    pub question: String,
    pub snippet: String,
    pub title: String,
    pub link: String,
}
