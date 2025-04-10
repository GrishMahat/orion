use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchResult {
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    pub action: Action,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    OpenFile(PathBuf),
    ExecuteCommand(String),
    OpenUrl(String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Command {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: String,
    pub action: Action,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchQuery {
    #[validate(length(min = 1))]
    pub text: String,
    #[validate(range(min = 1, max = 100))]
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub query: SearchQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    SearchQuery(SearchQuery),
    SearchResponse(SearchResponse),
    Command(Command),
    ConfigUpdate,
    Redirect(String),
    Error(String),
}
// this  json 
// #[derive(serde::Deserialize)]
// struct Bang {
//     c: String,  // category
//     d: String,  // domain
//     r: u32,     // rank
//     s: String,  // site name
//     sc: String, // subcategory
//     t: String,  // trigger (prefix)
//     u: String,  // url template
// }

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Bang {
    #[validate(length(min = 1))]
    pub category: String,
    #[validate(length(min = 1))]
    pub domain: String,
    pub score: i32,
    #[validate(length(min = 1))]
    pub display_name: String,
    #[validate(length(min = 1))]
    pub subcategory: String,
    #[validate(length(min = 1))]
    pub trigger: String,
    #[validate(length(min = 1))]
    pub url_template: String,
}

impl Bang {
    pub fn new(
        category: String,
        domain: String,
        score: i32,
        display_name: String,
        subcategory: String,
        trigger: String,
        url_template: String,
    ) -> Self {
        Bang {
            category,
            domain,
            score,
            display_name,
            subcategory,
            trigger,
            url_template,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.trigger.to_lowercase().contains(&query) ||
        self.display_name.to_lowercase().contains(&query) ||
        self.subcategory.to_lowercase().contains(&query)
    }
}

impl SearchResult {
    pub fn new(title: String, description: Option<String>, action: Action, score: f32) -> Self {
        SearchResult {
            title,
            description,
            action,
            score,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.title.to_lowercase().contains(&query) ||
        self.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query))
    }
}

impl Command {
    pub fn new(name: String, description: String, action: Action, keywords: Vec<String>) -> Self {
        Command {
            name,
            description,
            action,
            keywords,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query) ||
        self.description.to_lowercase().contains(&query) ||
        self.keywords.iter().any(|k| k.to_lowercase().contains(&query))
    }
}
