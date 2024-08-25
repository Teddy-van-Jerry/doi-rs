/// Digital Object Identifier (DOI) resolver for Rust
extern crate ureq;
use std::error::Error;

/// Digital Object Identifier (DOI) is a unique identifier for a digital object such as a document.
#[derive(Debug, Clone)]
pub struct Doi {
    /// A `String` representing the DOI number.
    pub doi: Option<String>,
    /// A `bool` for trying to use the system's proxy settings (default as `true`).
    env_proxy: bool,
}

impl Doi {
    /// Creates a new instance of [`Doi`].
    ///
    /// # Arguments
    ///
    /// * `doi` - A `String` or `&str` representing the DOI number.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// ```
    pub fn new<S: Into<String>>(doi: S) -> Self {
        Self {
            doi: Some(doi.into()),
            env_proxy: true,
        }
    }

    /// Checks if the DOI is set.
    pub fn is_set(&self) -> bool {
        self.doi.is_some()
    }

    /// Sets the DOI number.
    ///
    /// # Arguments
    ///
    /// * `doi` - A `String` or `&str` representing the DOI number.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let mut doi = Doi::default();
    /// doi.set_doi("10.1109/TCSII.2024.3366282");
    /// assert_eq!(doi.doi, Some("10.1109/TCSII.2024.3366282".to_string()));
    /// ```
    pub fn set_doi<S: Into<String>>(&mut self, doi: S) {
        self.doi = Some(doi.into());
    }

    /// Returns the URL of the DOI.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// assert_eq!(doi.https_url(), "https://doi.org/10.1109/TCSII.2024.3366282".to_string());
    /// ```
    pub fn https_url(&self) -> String {
        format!("https://doi.org/{}", self.doi.as_ref().unwrap())
    }

    /// Synchronously resolves the DOI and returns the resolved URL.
    ///
    /// This method sends a GET request to the DOI URL and returns the resolved URL.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if there is an error resolving the DOI.
    /// A 418 response code from the server does not count as an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    ///
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
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
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl Default for Doi {
    /// The default implementation of [`Doi`] returns a `None` value.
    ///
    /// # Examples
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::default();
    /// assert_eq!(doi.doi, None);
    /// assert_eq!(doi.is_set(), false);
    /// ```
    fn default() -> Self {
        Self {
            doi: None,
            env_proxy: true,
        }
    }
}
