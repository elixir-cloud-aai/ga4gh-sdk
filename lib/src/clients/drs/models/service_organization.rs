/*
 * Data Repository Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: ga4gh-cloud@ga4gh.org
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::clients::drs::models;
use serde::{Deserialize, Serialize};

/// ServiceOrganization : Organization providing the service
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceOrganization {
    /// Name of the organization responsible for the service
    #[serde(rename = "name")]
    pub name: String,
    /// URL of the website of the organization (RFC 3986 format)
    #[serde(rename = "url")]
    pub url: String,
}

impl ServiceOrganization {
    /// Organization providing the service
    pub fn new(name: String, url: String) -> ServiceOrganization {
        ServiceOrganization {
            name,
            url,
        }
    }
}
