use serde::{Deserialize, Serialize};

use crate::{SearchClient, WebSearchOptions, WebSearchResult};

pub struct SerperSearchClient {
    key: String,
}

impl SerperSearchClient {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[async_trait::async_trait]
impl SearchClient for SerperSearchClient {
    async fn search(&self, options: WebSearchOptions) -> anyhow::Result<Vec<WebSearchResult>> {
        let client = reqwest::Client::builder().build()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-KEY", self.key.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        let data = SerperRequest {
            q: options.query,
            gl: options.region,
            hl: options.language,
        };

        let request = client
            .request(reqwest::Method::POST, "https://google.serper.dev/search")
            .headers(headers)
            .json(&data);

        let response = request.send().await?;
        let body: SerperResult = response.json().await?;

        Ok(body
            .organic
            .into_iter()
            .map(|result| WebSearchResult {
                title: result.title,
                link: result.link,
                snippet: result.snippet,
            })
            .collect())
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
