use crate::Doi;
use std::error::Error;
pub use ureq::serde_json::Value as JsonValue;

/// Metadata for a DOI.
#[derive(Debug, Clone, Default)]
pub struct DoiMetadata {
    /// Digital Object Identifier (DOI) number.
    pub doi: String,
    /// Title of the document.
    pub title: Option<String>,
}

impl DoiMetadata {
    /// Creates a new instance of [`DoiMetadata`].
    pub fn new(doi: String) -> Self {
        Self { doi, title: None }
    }
}

impl Doi {
    /// Fetches metadata for the DOI.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the DOI is not set, i.e., `None`.
    /// Returns a `Box<dyn Error>` if there is an error fetching metadata from doi.org.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// match doi.metadata() {
    ///     Ok(metadata) => {
    ///         let title = metadata.title.unwrap_or("<unknown>".to_string());
    ///         println!("Paper Title: {}", title);
    ///         assert_eq!(title, "Flexible High-Level Synthesis Library for Linear Transformations".to_string());
    ///     },
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn metadata(&self) -> Result<DoiMetadata, Box<dyn Error>> {
        let doi = self.get_doi()?;
        let mut metadata = DoiMetadata::new(doi);
        let json = self.metadata_json()?;
        if let Some(title) = json["title"].as_str() {
            metadata.title = Some(title.to_string());
        }
        Ok(metadata)
    }

    /// Fetches metadata for the DOI (with `.call()?`).
    fn metadata_call(&self, accept: &str) -> Result<ureq::Response, Box<dyn Error>> {
        self.get_doi()?; // Check if DOI is set.
        Ok(self
            .agent
            .get(&self.https_url())
            .set("Accept", accept)
            .call()?)
    }

    /// Fetches metadata for the DOI in JSON format.
    ///
    /// Serde JSON is used to parse the JSON response, which is returned as a `serde_json::Value`.
    /// This type is aliased as [`JsonValue`] in the `doi` crate.
    ///
    /// # DOI API
    ///
    /// Internally, this method calls the doi.org API with the `Accept: application/json` header.
    /// With `curl`, this is equivalent to:
    /// ```sh
    /// curl -LH "Accept: application/json" https://doi.org/<DOI>
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the DOI is not set, i.e., `None`.
    /// Returns a `Box<dyn Error>` if there is an error fetching metadata from doi.org.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// match doi.metadata_json() {
    ///     Ok(json) => {
    ///         let title = json["title"].as_str().unwrap();
    ///         println!("Paper Title: {}", title);
    ///         assert_eq!(title, "Flexible High-Level Synthesis Library for Linear Transformations");
    ///     },
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn metadata_json(&self) -> Result<JsonValue, Box<dyn Error>> {
        self.metadata_call("application/json")?
            .into_json()
            .map_err(|e| format!("Error parsing JSON: {}", e).into())
    }

    /// Fetches metadata for the DOI in JSON format (as a string).
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the DOI is not set, i.e., `None`.
    /// Returns a `Box<dyn Error>` if there is an error fetching metadata from doi.org.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// match doi.metadata_json_string() {
    ///     Ok(json) => println!("JSON: {}", json),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn metadata_json_string(&self) -> Result<String, Box<dyn Error>> {
        self.metadata_call("application/json")?
            .into_string()
            .map_err(|e| format!("Error parsing JSON: {}", e).into())
    }

    /// Fetches metadata for the DOI in BibTeX format.
    ///
    /// # DOI API
    ///
    /// Internally, this method calls the doi.org API with the `Accept: application/x-bibtex` header.
    /// With `curl`, this is equivalent to:
    /// ```sh
    /// curl -LH "Accept: application/x-bibtex" https://doi.org/<DOI>
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the DOI is not set, i.e., `None`.
    /// Returns a `Box<dyn Error>` if there is an error fetching metadata from doi.org.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// match doi.metadata_bibtex() {
    ///     Ok(bibtex) => println!("BibTeX: {}", bibtex),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn metadata_bibtex(&self) -> Result<String, Box<dyn Error>> {
        self.metadata_call("application/x-bibtex")?
            .into_string()
            .map_err(|e| format!("Error fetching BibTeX: {}", e).into())
    }
}
