/// Digital Object Identifier (DOI) resolver for Rust
extern crate ureq;
use std::error::Error;
use ureq::Agent;

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
        match self.agent.head(&url).call() {
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

#[derive(Debug, Clone, Default)]
pub struct DoiBuilder {
    /// An `Option<String>` representing the DOI number.
    doi: Option<String>,
    /// A `bool` for trying to use the system's proxy settings (default as `true`).
    env_proxy: bool,
    /// An `Option<String>` representing the proxy URL.
    proxy: Option<ureq::Proxy>,
}

/// Builder for the [`Doi`] struct.
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
    pub fn proxy<S: Into<String>>(&mut self, proxy: S) -> Result<&mut Self, Box<dyn Error>> {
        // self.proxy = Some(proxy.into());
        self.proxy = Some(ureq::Proxy::new(proxy.into())?);
        Ok(self)
    }

    /// Returns the default `ureq::Agent`.
    #[cfg(feature = "proxy")]
    pub fn default_agent() -> Agent {
        ureq::AgentBuilder::new().try_proxy_from_env(true).build()
    }

    /// Returns the default `ureq::Agent` (with no proxy).
    #[cfg(not(feature = "proxy"))]
    pub fn default_agent() -> Agent {
        ureq::AgentBuilder::new().build()
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
        #[cfg(feature = "proxy")]
        let build_agent = || -> Agent {
            if let Some(proxy) = &self.proxy {
                ureq::AgentBuilder::new().proxy(proxy.clone()).build()
            } else {
                ureq::AgentBuilder::new()
                    .try_proxy_from_env(self.env_proxy)
                    .build()
            }
        };
        #[cfg(not(feature = "proxy"))]
        let build_agent = || -> Agent { ureq::AgentBuilder::new().build() };
        Doi {
            doi: self.doi.clone(),
            agent: build_agent(),
        }
    }
}
