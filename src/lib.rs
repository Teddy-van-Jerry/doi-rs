//! Digital Object Identifier (DOI) resolver for Rust
//!
//! This crate provides a simple way to resolve DOIs and retrieve metadata
//! using the [DOI](https://www.doi.org) APIs.
//!
//! ## Basic Usage
//! The following is a basic example of using this library:
//! ```rust
//! use doi::Doi;
//! let doi = Doi::new("10.1109/TCSII.2024.3366282");
//! match doi.resolve() {
//!     Ok(resolved) => println!("Resolved Link: {}", resolved),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! Please refer to the API documentation for more information.
//! More complicated constructions can be done using the [`DoiBuilder`] struct.
//!
//! ## Metadata
//! This library also provides a way to retrieve metadata for a DOI.
//! The `metadata` feature is required to use this functionality (enabled by default).
//!
//! ### Structured Metadata
//! The structured metadata can be retrieved using the `metadata` method.
//! ```rust
//! use doi::Doi;
//! let doi = Doi::new("10.1109/TCSII.2024.3366282");
//! match doi.metadata() {
//!     Ok(metadata) => println!("Paper Title: {}", metadata.title.unwrap_or("<unknown>".to_string())),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! Supported metadata fields:
//! | Field | Type | Description |
//! | --- | --- | --- |
//! | `title` | `Option<String>` | Title of the document |
//! | `authors` | `Option<Vec<DoiMetadataPerson>>` | Author(s) of the document |
//! | `r#type` | `Option<DoiMetadataType>` | Type of the document (e.g., journal, conference) |
//!
//! The [`DoiMetadataPerson`] struct has the fields `given`, `family`, and `suffix`, which are all `Option<String>`.
//! The [`DoiMetadataType`] enum has the [`DoiMetadataType::as_str`] method to get the string representation.
//!
//! ### Raw JSON Metadata
//! The raw JSON metadata can be retrieved using the [`Doi::metadata_json`] method,
//! powered by `serde_json`.
//! ```rust
//! use doi::Doi;
//! let doi = Doi::new("10.1109/TCSII.2024.3366282");
//! match doi.metadata_json() {
//!     Ok(metadata) => println!("Metadata: {}", metadata),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! The raw JSON string can be obtained via the [`Doi::metadata_json_string`] method.
//!
//! ## Blocking Requests
//! This library is designed to use blocking I/O,
//! depending on the [`ureq` library](https://docs.rs/ureq) for HTTP requests.
//!
//! ## License
//! This project is licensed under the [MIT license](https://github.com/Teddy-van-Jerry/doi-rs/blob/master/LICENSE).

extern crate ureq;
use std::error::Error;
use ureq::config::Config;
use ureq::{Agent, ResponseExt};

/// Digital Object Identifier (DOI) is a unique identifier for a digital object such as a document.
#[derive(Debug, Clone)]
pub struct Doi {
    /// A `String` representing the DOI number.
    pub doi: Option<String>,
    /// A `ureq::Agent` for making HTTP requests.
    agent: Agent,
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
    /// let doi1 = Doi::new("10.1109/TCSII.2024.3366282");
    /// let doi2 = Doi::new("10.1145/3643832.3661865".to_string());
    /// let mut doi3 = Doi::new("10.1109/TCSII.2024.3366282");
    /// doi3.set_doi("10.1145/3643832.3661865");
    /// assert_eq!(doi2, doi3);
    /// ```
    pub fn new<S: Into<String>>(doi: S) -> Self {
        Self {
            doi: Some(doi.into()),
            agent: DoiBuilder::default_agent(),
        }
    }

    /// Checks if the DOI is set.
    pub fn is_set(&self) -> bool {
        self.doi.is_some()
    }

    /// Validates the DOI format according to ISO 26324.
    ///
    /// A valid DOI must:
    /// - Have a prefix starting with `10.`
    /// - Have a registrant code (number ≥ 1000) after `10.`
    /// - Have a `/` separating the prefix and suffix
    /// - Have a non-empty suffix
    ///
    /// Returns `false` if the DOI is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    ///
    /// // Valid DOIs
    /// assert!(Doi::new("10.1000/182").is_valid());
    /// assert!(Doi::new("10.1109/TCSII.2024.3366282").is_valid());
    /// assert!(Doi::new("10.1000.1/test").is_valid()); // Sub-divided prefix
    ///
    /// // Invalid DOIs
    /// assert!(!Doi::new("not-a-doi").is_valid());
    /// assert!(!Doi::new("10.999/test").is_valid()); // Registrant < 1000
    /// assert!(!Doi::new("10.1000").is_valid()); // No suffix
    /// assert!(!Doi::new("11.1000/test").is_valid()); // Wrong directory indicator
    /// ```
    pub fn is_valid(&self) -> bool {
        let Some(doi) = &self.doi else {
            return false;
        };

        // Must start with "10."
        if !doi.starts_with("10.") {
            return false;
        }

        // Find the separator between prefix and suffix
        let Some(slash_pos) = doi.find('/') else {
            return false;
        };

        // Extract registrant code (everything between "10." and the first "/" or ".")
        let after_10 = &doi[3..slash_pos];
        if after_10.is_empty() {
            return false;
        }

        // The registrant code is the first segment (before any sub-element ".")
        let registrant_code = after_10.split('.').next().unwrap_or("");

        // Registrant code must be numeric and >= 1000
        match registrant_code.parse::<u32>() {
            Ok(code) if code >= 1000 => {}
            _ => return false,
        }

        // Suffix must be non-empty
        let suffix = &doi[slash_pos + 1..];
        !suffix.is_empty()
    }

    /// Returns the DOI number.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the DOI is not set, i.e., `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::Doi;
    /// let doi = Doi::new("10.1109/TCSII.2024.3366282");
    /// assert_eq!(doi.get_doi().unwrap(), "10.1109/TCSII.2024.3366282".to_string());
    /// ```
    pub fn get_doi(&self) -> Result<String, Box<dyn Error>> {
        match &self.doi {
            Some(doi) => Ok(doi.clone()),
            None => Err("DOI is not set".into()),
        }
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
    /// The URL is in the format `https://doi.org/<DOI_NUMBER>`.
    /// The `doi` field must be set.
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
        // Use a request builder with http_status_as_error=false to get the response
        // even on 4xx/5xx status codes, allowing us to extract the final URI
        let response = self
            .agent
            .head(&url)
            .config()
            .http_status_as_error(false)
            .build()
            .call()?;
        let resolved_link = response.get_uri().to_string();
        Ok(resolved_link)
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
            agent: DoiBuilder::default_agent(),
        }
    }
}

impl PartialEq for Doi {
    /// Compares two [`Doi`] instances.
    ///
    /// The two DOIs are the same when their lowercase values are equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use doi::Doi;
    /// let doi1 = Doi::new("10.1109/TCSII.2024.3366282");
    /// let mut doi2 = Doi::new("10.1109/TCSII.2024.3366282");
    /// let doi3 = Doi::new("10.1109/tcsii.2024.3366282");
    /// let doi4 = Doi::new("10.1145/3643832.3661865");
    /// let doi5 = Doi::default();
    /// let doi6 = Doi::default();
    /// assert_eq!(doi1, doi2);
    /// assert_eq!(doi1, doi3);
    /// assert_ne!(doi1, doi4);
    /// assert_ne!(doi1, doi5);
    /// assert_eq!(doi5, doi6);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (&self.doi, &other.doi) {
            (Some(doi1), Some(doi2)) => doi1.to_lowercase() == doi2.to_lowercase(),
            (None, None) => true,
            _ => false,
        }
    }
}

/// Builder for the [`Doi`] struct.
#[derive(Debug, Clone, Default)]
pub struct DoiBuilder {
    /// An `Option<String>` representing the DOI number.
    doi: Option<String>,
    /// A `bool` for trying to use the system's proxy settings (default as `true`).
    env_proxy: bool,
    /// An `Option<ureq::Proxy>` representing the proxy.
    proxy: Option<ureq::Proxy>,
}

impl DoiBuilder {
    /// Creates a new instance of [`DoiBuilder`].
    ///
    /// The `doi` field is set to `None` and the `env_proxy` field is set to `true`.
    pub fn new() -> Self {
        Self {
            doi: None,
            env_proxy: true,
            proxy: None,
        }
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
    /// use doi::{Doi, DoiBuilder};
    /// let doi = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").build();
    /// assert_eq!(doi.doi, Some("10.1109/TCSII.2024.3366282".to_string()));
    /// ```
    pub fn doi<S: Into<String>>(&mut self, doi: S) -> &mut Self {
        self.doi = Some(doi.into());
        self
    }

    /// Sets whether to use the system's proxy settings.
    ///
    /// This will be overridden by the [`Self::proxy`] method.
    ///
    /// # Arguments
    ///
    /// * `env_proxy` - A `bool` representing whether to use the system's proxy settings.
    ///                 It is `true` by default if the `proxy` feature is enabled.
    ///                 (The `proxy` feature is enabled by default.)
    ///
    /// # Example
    ///
    /// ```
    /// use doi::{Doi, DoiBuilder};
    /// let doi = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").env_proxy(false).build();
    /// ```
    pub fn env_proxy(&mut self, env_proxy: bool) -> &mut Self {
        self.env_proxy = env_proxy;
        self
    }

    /// Sets the proxy URL explicitly.
    ///
    /// # Arguments
    ///
    /// * `proxy` - A `String` or `&str` representing the proxy URL.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if there is an error creating the proxy.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::{Doi, DoiBuilder};
    /// let doi = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").proxy("http://127.0.0.1:7890").unwrap().build();
    /// ```
    pub fn proxy<S: AsRef<str>>(&mut self, proxy: S) -> Result<&mut Self, Box<dyn Error>> {
        self.proxy = Some(ureq::Proxy::new(proxy.as_ref())?);
        Ok(self)
    }

    /// Returns the default `ureq::Agent`.
    ///
    /// Uses `Config::default()` which automatically picks up proxy from environment.
    pub fn default_agent() -> Agent {
        Agent::new_with_defaults()
    }

    /// Builds the [`Doi`] instance.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::{Doi, DoiBuilder};
    /// let doi = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").build();
    /// ```
    pub fn build(&self) -> Doi {
        let agent = if let Some(proxy) = &self.proxy {
            let config = Config::builder().proxy(Some(proxy.clone())).build();
            Agent::new_with_config(config)
        } else if self.env_proxy {
            // Default config picks up proxy from environment
            Agent::new_with_defaults()
        } else {
            // No proxy
            let config = Config::builder().proxy(None).build();
            Agent::new_with_config(config)
        };
        Doi {
            doi: self.doi.clone(),
            agent,
        }
    }
}

#[cfg(feature = "metadata")]
mod metadata;
#[cfg(feature = "metadata")]
pub use metadata::{DoiMetadata, DoiMetadataPerson, DoiMetadataType, JsonValue};
