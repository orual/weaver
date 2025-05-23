// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.video.getUploadLimits` namespace.
pub const NSID: &str = "app.bsky.video.getUploadLimits";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub can_upload: bool,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub error: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub message: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub remaining_daily_bytes: core::option::Option<i64>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub remaining_daily_videos: core::option::Option<i64>,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
