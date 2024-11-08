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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorizations {
    #[serde(rename = "drs_object_id", skip_serializing_if = "Option::is_none")]
    pub drs_object_id: Option<String>,
    /// An Optional list of support authorization types. More than one can be supported and tried in sequence. Defaults to `None` if empty or missing.
    #[serde(rename = "supported_types", skip_serializing_if = "Option::is_none")]
    pub supported_types: Option<Vec<SupportedTypes>>,
    /// If authorizations contain `PassportAuth` this is a required list of visa issuers (as found in a visa's `iss` claim) that may authorize access to this object. The caller must only provide passports that contain visas from this list. It is strongly recommended that the caller validate that it is appropriate to send the requested passport/visa to the DRS server to mitigate attacks by malicious DRS servers requesting credentials they should not have.
    #[serde(rename = "passport_auth_issuers", skip_serializing_if = "Option::is_none")]
    pub passport_auth_issuers: Option<Vec<String>>,
    /// If authorizations contain `BearerAuth` this is an optional list of issuers that may authorize access to this object. The caller must provide a token from one of these issuers. If this is empty or missing it assumed the caller knows which token to send via other means. It is strongly recommended that the caller validate that it is appropriate to send the requested token to the DRS server to mitigate attacks by malicious DRS servers requesting credentials they should not have.
    #[serde(rename = "bearer_auth_issuers", skip_serializing_if = "Option::is_none")]
    pub bearer_auth_issuers: Option<Vec<String>>,
}

impl Authorizations {
    pub fn new() -> Authorizations {
        Authorizations {
            drs_object_id: None,
            supported_types: None,
            passport_auth_issuers: None,
            bearer_auth_issuers: None,
        }
    }
}
/// An Optional list of support authorization types. More than one can be supported and tried in sequence. Defaults to `None` if empty or missing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedTypes {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "BasicAuth")]
    BasicAuth,
    #[serde(rename = "BearerAuth")]
    BearerAuth,
    #[serde(rename = "PassportAuth")]
    PassportAuth,
}

impl Default for SupportedTypes {
    fn default() -> SupportedTypes {
        Self::None
    }
}
