pub mod serper;

pub struct WebSearchOptions {
    pub query: String,
    pub language: String,
    pub region: String,
}

pub struct WebSearchResult {
    pub title: String,
    pub link: String,
    pub snippet: String,
}

#[async_trait::async_trait]
pub trait SearchClient: Send + Sync {
    async fn search(&self, options: WebSearchOptions) -> anyhow::Result<Vec<WebSearchResult>>;
}
