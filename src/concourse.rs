//! # Concourse
//!
//! `concourse` contains the structs for serialization to concourse outputs and deserialization from concourse inputs. Ordinarily more functionality is required here, but this crate leverages the concourse rust bindings to automatically provide functionality through trait implementations.

use serde::{Deserialize, Serialize};

use concourse_resource::IntoMetadataKV;

// standard concourse structs

// check input and (list) output, out output
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Version {
    state: String,
}

impl Version{
    /// Constructor
    pub(crate) fn new(state: String) -> Self {
        Version { state }
    }
}

// check and out input
#[derive(Deserialize, Debug)]
pub(crate) struct Source {
    // client (later converted to &str)
    pat: Option<String>,
    owner: String,
    repo: String,
    // read and update
    number: Option<u64>,
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

// out output
#[derive(Serialize, Debug, IntoMetadataKV)]
pub(crate) struct OutMetadata {
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

impl OutMetadata {
    /// Constructor
    pub(crate) fn new(number: u64, labels: Option<Vec<String>>, assignees: Option<Vec<String>>) -> Self {
        OutMetadata{ number, labels, assignees }
    }
}
