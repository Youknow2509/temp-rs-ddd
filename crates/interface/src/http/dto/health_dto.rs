use serde::Deserialize;

// Query parameters for GET /health
// Example: GET /health?verbose=true
#[derive(Debug, Deserialize)]
pub struct HealthQuery {
    // Return extended info (db status, version, etc.) when true.
    // Defaults to false when omitted from the query string.
    pub verbose: Option<bool>,
}

// Template: query params for a paginated list endpoint.
// Move to the relevant dto file when implementing that group.
//
// #[derive(Debug, Deserialize)]
// pub struct ListQuery {
//     pub page:   Option<u32>,
//     pub limit:  Option<u32>,
//     pub search: Option<String>,
//     pub sort:   Option<String>, // e.g. "created_at:desc"
// }
