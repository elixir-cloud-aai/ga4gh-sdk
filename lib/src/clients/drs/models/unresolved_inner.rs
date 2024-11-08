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
pub struct UnresolvedInner {
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "object_ids", skip_serializing_if = "Option::is_none")]
    pub object_ids: Option<Vec<String>>,
}

impl UnresolvedInner {
    pub fn new() -> UnresolvedInner {
        UnresolvedInner {
            error_code: None,
            object_ids: None,
        }
    }
}
