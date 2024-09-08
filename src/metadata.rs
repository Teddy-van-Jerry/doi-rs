use crate::Doi;
use std::error::Error;
pub use ureq::serde_json::Value as JsonValue;

/// Metadata for a DOI.
///
/// The metadata schema is roughly [`citation-style-language`](https://github.com/citation-style-language/schema).
#[derive(Debug, Clone, Default)]
pub struct DoiMetadata {
    /// Digital Object Identifier (DOI) number.
    pub doi: String,
    /// Title of the document.
    pub title: Option<String>,
    /// Author(s) of the document.
    pub authors: Option<Vec<DoiMetadataPerson>>,
    /// Type of the document (e.g., journal, conference).
    pub r#type: Option<DoiMetadataType>,
}

/// Metadata for a person.
#[derive(Debug, Clone, Default)]
pub struct DoiMetadataPerson {
    pub given: Option<String>,
    pub family: Option<String>,
    pub suffix: Option<String>,
}

impl DoiMetadataPerson {
    /// Returns the full name of the person.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the person's name is not set (all fields empty).
    ///
    /// # Example
    ///
    /// ```
    /// use doi::DoiMetadataPerson;
    /// let person = DoiMetadataPerson {
    ///      given: Some("Teddy".to_string()),
    ///      family: Some("Jerry".to_string()),
    ///      suffix: Some("Jr.".to_string()),
    /// };
    /// assert_eq!(person.full_name().unwrap(), "Teddy Jerry Jr.".to_string());
    /// let no_name = DoiMetadataPerson::default();
    /// assert_eq!(no_name.full_name().is_ok(), false);
    /// ```
    pub fn full_name(&self) -> Result<String, ()> {
        match (&self.given, &self.family, &self.suffix) {
            (Some(given), Some(family), Some(suffix)) => {
                Ok(format!("{} {} {}", given, family, suffix))
            }
            (Some(given), Some(family), None) => Ok(format!("{} {}", given, family)),
            (Some(given), None, Some(suffix)) => Ok(format!("{} {}", given, suffix)),
            (Some(given), None, None) => Ok(given.to_string()),
            (None, Some(family), Some(suffix)) => Ok(format!("{} {}", family, suffix)),
            (None, Some(family), None) => Ok(family.to_string()),
            (None, None, Some(suffix)) => Ok(suffix.to_string()),
            _ => Err(()),
        }
    }
}

/// Metadata type for a DOI.
///
/// Reference: [`csl-data.json`](https://github.com/citation-style-language/schema/blob/e3ce254a72c4470a5ed3b9d23b428017d25674e9/schemas/input/csl-data.json#L9-L58)
#[derive(Debug, Clone, PartialEq)]
pub enum DoiMetadataType {
    Article,
    ArticleJournal,
    ArticleMagazine,
    ArticleNewspaper,
    Bill,
    Book,
    Broadcast,
    Chapter,
    Classic,
    Collection,
    Dataset,
    Document,
    Entry,
    EntryDictionary,
    EntryEncyclopedia,
    Event,
    Figure,
    Graphic,
    Hearing,
    Interview,
    LegalCase,
    Legislation,
    Manuscript,
    Map,
    MotionPicture,
    MusicalScore,
    Pamphlet,
    PaperConference,
    Patent,
    Performance,
    Periodical,
    PersonalCommunication,
    Post,
    PostWeblog,
    Regulation,
    Report,
    Review,
    ReviewBook,
    Software,
    Song,
    Speech,
    Standard,
    Thesis,
    Treaty,
    Webpage,
    MISC(String),
}

impl DoiMetadataType {
    /// Creates a new instance of [`DoiMetadataType`] from a string.
    ///
    /// # Example
    ///
    /// ```
    /// use doi::DoiMetadataType;
    /// let article = DoiMetadataType::new("article");
    /// assert_eq!(article, DoiMetadataType::Article);
    /// let misc = DoiMetadataType::new("unknown");
    /// assert_eq!(misc, DoiMetadataType::MISC("unknown".to_string()));
    /// ```
    pub fn new(s: &str) -> Self {
        match s {
            "article" => Self::Article,
            "article-journal" => Self::ArticleJournal,
            "article-magazine" => Self::ArticleMagazine,
            "article-newspaper" => Self::ArticleNewspaper,
            "bill" => Self::Bill,
            "book" => Self::Book,
            "broadcast" => Self::Broadcast,
            "chapter" => Self::Chapter,
            "classic" => Self::Classic,
            "collection" => Self::Collection,
            "dataset" => Self::Dataset,
            "document" => Self::Document,
            "entry" => Self::Entry,
            "entry-dictionary" => Self::EntryDictionary,
            "entry-encyclopedia" => Self::EntryEncyclopedia,
            "event" => Self::Event,
            "figure" => Self::Figure,
            "graphic" => Self::Graphic,
            "hearing" => Self::Hearing,
            "interview" => Self::Interview,
            "legal_case" => Self::LegalCase,
            "legislation" => Self::Legislation,
            "manuscript" => Self::Manuscript,
            "map" => Self::Map,
            "motion_picture" => Self::MotionPicture,
            "musical_score" => Self::MusicalScore,
            "pamphlet" => Self::Pamphlet,
            "paper-conference" => Self::PaperConference,
            "patent" => Self::Patent,
            "performance" => Self::Performance,
            "periodical" => Self::Periodical,
            "personal_communication" => Self::PersonalCommunication,
            "post" => Self::Post,
            "post-weblog" => Self::PostWeblog,
            "regulation" => Self::Regulation,
            "report" => Self::Report,
            "review" => Self::Review,
            "review-book" => Self::ReviewBook,
            "software" => Self::Software,
            "song" => Self::Song,
            "speech" => Self::Speech,
            "standard" => Self::Standard,
            "thesis" => Self::Thesis,
            "treaty" => Self::Treaty,
            "webpage" => Self::Webpage,
            s => Self::MISC(s.to_string()),
        }
    }

    /// Returns the DOI metadata type as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Article => "article",
            Self::ArticleJournal => "article-journal",
            Self::ArticleMagazine => "article-magazine",
            Self::ArticleNewspaper => "article-newspaper",
            Self::Bill => "bill",
            Self::Book => "book",
            Self::Broadcast => "broadcast",
            Self::Chapter => "chapter",
            Self::Classic => "classic",
            Self::Collection => "collection",
            Self::Dataset => "dataset",
            Self::Document => "document",
            Self::Entry => "entry",
            Self::EntryDictionary => "entry-dictionary",
            Self::EntryEncyclopedia => "entry-encyclopedia",
            Self::Event => "event",
            Self::Figure => "figure",
            Self::Graphic => "graphic",
            Self::Hearing => "hearing",
            Self::Interview => "interview",
            Self::LegalCase => "legal_case",
            Self::Legislation => "legislation",
            Self::Manuscript => "manuscript",
            Self::Map => "map",
            Self::MotionPicture => "motion_picture",
            Self::MusicalScore => "musical_score",
            Self::Pamphlet => "pamphlet",
            Self::PaperConference => "paper-conference",
            Self::Patent => "patent",
            Self::Performance => "performance",
            Self::Periodical => "periodical",
            Self::PersonalCommunication => "personal_communication",
            Self::Post => "post",
            Self::PostWeblog => "post-weblog",
            Self::Regulation => "regulation",
            Self::Report => "report",
            Self::Review => "review",
            Self::ReviewBook => "review-book",
            Self::Software => "software",
            Self::Song => "song",
            Self::Speech => "speech",
            Self::Standard => "standard",
            Self::Thesis => "thesis",
            Self::Treaty => "treaty",
            Self::Webpage => "webpage",
            Self::MISC(s) => s,
        }
    }
}

impl DoiMetadata {
    /// Creates a new instance of [`DoiMetadata`].
    pub fn new(doi: String) -> Self {
        Self {
            doi,
            title: None,
            authors: None,
            r#type: None,
        }
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
    ///         let authors = metadata.authors.unwrap_or_default();
    ///         for author in authors {
    ///             let name = author.full_name().unwrap_or("<unknown>".to_string());
    ///             println!("Author: {}", name);
    ///         }
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
        if let Some(authors) = json["author"].as_array() {
            let mut author_list = Vec::new();
            for author in authors {
                let given = author["given"].as_str().map(|s| s.to_string());
                let family = author["family"].as_str().map(|s| s.to_string());
                let suffix = author["suffix"].as_str().map(|s| s.to_string());
                author_list.push(DoiMetadataPerson {
                    given,
                    family,
                    suffix,
                });
            }
            metadata.authors = Some(author_list);
        }
        if let Some(r#type) = json["type"].as_str() {
            metadata.r#type = Some(DoiMetadataType::new(r#type));
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
