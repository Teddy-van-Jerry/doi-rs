extern crate ureq;
use std::error::Error;

/// Digital Object Identifier (DOI) is a unique identifier for a digital object such as a document.
#[derive(Debug, Clone)]
pub struct Doi {
    /// A `String` representing the DOI number.
    pub doi: String,
}

impl Doi {
    /// Creates a new instance of [`Doi`].
    ///
    /// # Arguments
    ///
    /// * `doi` - A `String` representing the DOI.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282".to_string());
    /// ```
    pub fn new(doi: String) -> Self {
        Self { doi }
    }

    /// Returns the URL of the DOI.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282".to_string());
    /// assert_eq!(doi.https_url(), "https://doi.org/10.1109/TCSII.2024.3366282".to_string());
    /// ```
    pub fn https_url(&self) -> String {
        format!("https://doi.org/{}", self.doi)
    }

    /// Synchronously resolves the DOI and returns the resolved URL.
    ///
    /// This method sends a GET request to the DOI URL and returns the resolved URL.
    /// It expects a 418 response code from the server.
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if there is an error resolving the DOI.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    ///
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282".to_string());
    /// let resolved_link = doi.resolve();
    /// match resolved_link {
    ///     Ok(link) => {
    ///         println!("Resolved link: {}", link);
    ///         assert_eq!(link, "https://ieeexplore.ieee.org/document/10437992/".to_string());
    ///     },
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn resolve(&self) -> Result<String, Box<dyn Error>> {
        let url = self.https_url();
        match ureq::get(&url).call() {
            Ok(response) | Err(ureq::Error::Status(418, response)) => {
                let resolved_link = response.get_url().to_string();
                Ok(resolved_link)
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
