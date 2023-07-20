//! # Concourse
//!
//! `concourse` contains the structs for serialization to concourse outputs and deserialization from concourse inputs. Ordinarily more functionality is required here, but this crate leverages the concourse rust bindings to automatically provide functionality through trait implementations.

use serde::{Deserialize, Serialize};

use concourse_resource::IntoMetadataKV;

// standard concourse structs
// check input and (vec seralized to list) output, out output
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Version {
    state: String,
}

impl Version {
    /// Constructor
    ///
    /// # Examples
    ///
    /// ```
    /// let version = Version::new(String::from("Open"));
    /// ```
    pub(crate) fn new(state: String) -> Self {
        Version { state }
    }
}

// check and out input
#[derive(Deserialize, Debug)]
pub(crate) struct Source {
    // client and issues
    pat: Option<String>,
    owner: String,
    repo: String,
    // read and update
    number: Option<u64>,
}

impl Source {
    /// Readers
    pub(crate) fn pat(&self) -> Option<String> {
        return self.pat.clone();
    }
    pub(crate) fn owner(&self) -> String {
        return self.owner.clone();
    }
    pub(crate) fn repo(&self) -> String {
        return self.repo.clone();
    }
    pub(crate) fn number(&self) -> Option<u64> {
        return self.number;
    }
}

// out input
#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub(crate) struct OutParams {
    // title and body later converted to &str
    title: String,
    body: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
}

impl OutParams {
    /// Readers
    pub(crate) fn title(&self) -> String {
        return self.title.clone();
    }
    pub(crate) fn body(&self) -> Option<String> {
        return self.body.clone();
    }
    pub(crate) fn labels(&self) -> Option<Vec<String>> {
        return self.labels.clone();
    }
    pub(crate) fn assignees(&self) -> Option<Vec<String>> {
        return self.assignees.clone();
    }
}

// out output TODO ask for other desired information in metadata
#[derive(Serialize, Debug, IntoMetadataKV)]
pub(crate) struct OutMetadata {
    number: u64,
    labels: Vec<octocrab::models::Label>,
    assignees: Vec<octocrab::models::Author>,
}

impl OutMetadata {
    /// Constructor
    ///
    /// # Examples
    ///
    /// ```
    /// let metadata = OutMetadata::new(10, !vec[String::from("triage")], !vec[String::from("myuser")];
    /// ```
    pub(crate) fn new(
        number: u64,
        labels: Vec<octocrab::models::Label>,
        assignees: Vec<octocrab::models::Author>,
    ) -> Self {
        OutMetadata {
            number,
            labels,
            assignees,
        }
    }
}
