use crate::Doi;
use std::error::Error;
pub use ureq::serde_json::Value as JsonValue;

#[derive(Debug, Clone, Default)]
pub struct DoiMetadata {
    pub doi: String,
    pub title: Option<String>,
}

impl DoiMetadata {
    pub fn new(doi: String) -> Self {
        Self { doi, title: None }
    }
}

impl Doi {
    pub fn metadata(&self) -> Result<DoiMetadata, Box<dyn Error>> {
        let doi = self.get_doi()?;
        let mut metadata = DoiMetadata::new(doi);
        let json = self.metadata_json()?;
        if let Some(title) = json["title"].as_str() {
            metadata.title = Some(title.to_string());
        }
        Ok(metadata)
    }

    fn metadata_call(&self, accept: &str) -> Result<ureq::Response, Box<dyn Error>> {
        self.get_doi()?; // Check if DOI is set.
        Ok(self
            .agent
            .get(&self.https_url())
            .set("Accept", accept)
            .call()?)
    }

    pub fn metadata_json(&self) -> Result<JsonValue, Box<dyn Error>> {
        self.metadata_call("application/json")?
            .into_json()
            .map_err(|e| format!("Error parsing JSON: {}", e).into())
    }

    pub fn metadata_json_string(&self) -> Result<String, Box<dyn Error>> {
        self.metadata_call("application/json")?
            .into_string()
            .map_err(|e| format!("Error parsing JSON: {}", e).into())
    }

    pub fn metadata_bibtex(&self) -> Result<String, Box<dyn Error>> {
        self.metadata_call("application/x-bibtex")?
            .into_string()
            .map_err(|e| format!("Error fetching BibTeX: {}", e).into())
    }
}
