/*
 * GA4GH Tool Discovery API
 *
 * Proposed API for GA4GH (Global Alliance for Genomics & Health) tool repositories. A tool consists of a set of container images that are paired with a set of documents. Examples of documents include CWL (Common Workflow Language), WDL (Workflow Description Language), NFL (Nextflow), GXFORMAT2 (Galaxy) or SMK (Snakemake) that describe how to use those images and a set of specifications for those images (examples are Dockerfiles or Singularity recipes) that describe how to reproduce those images in the future. We use the following terminology, a \"container image\" describes a container as stored at rest on a filesystem, a \"tool\" describes one of the triples as described above. In practice, examples of \"tools\" include CWL CommandLineTools, CWL Workflows, WDL workflows, and Nextflow workflows that reference containers in formats such as Docker or Singularity.
 *
 * The version of the OpenAPI document: 2.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::clients::trs::models;
use serde::{Deserialize, Serialize};

/// ToolClass : Describes a class (type) of tool allowing us to categorize workflows, tasks, and maybe even other entities (such as services) separately.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolClass {
    /// The unique identifier for the class.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A short friendly name for the class.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A longer explanation of what this class is and what it can accomplish.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ToolClass {
    /// Describes a class (type) of tool allowing us to categorize workflows, tasks, and maybe even other entities (such as services) separately.
    pub fn new() -> ToolClass {
        ToolClass {
            id: None,
            name: None,
            description: None,
        }
    }
}

