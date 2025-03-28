
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteRequest {
	/// Send an email to this address
	pub email: String,
	/// An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub projects: Option<Vec<InviteRequestProjectsItem>>,
	/// `owner` or `reader`
	pub role: InviteRequestRole,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteRequestProjectsItem {
	/// Project's public ID
	pub id: String,
	/// Project membership role
	pub role: InviteRequestProjectsItemRole,
}
/// Project membership role
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InviteRequestProjectsItemRole {
	Member,
	Owner,
}
/// `owner` or `reader`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InviteRequestRole {
	Reader,
	Owner,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectCreateRequest {
	/// The friendly name of the project, this name appears in reports.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ProjectRateLimitUpdateRequest {
	/// The maximum batch input tokens per day. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_1_day_max_input_tokens: Option<i64>,
	/// The maximum audio megabytes per minute. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_audio_megabytes_per_1_minute: Option<i64>,
	/// The maximum images per minute. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_images_per_1_minute: Option<i64>,
	/// The maximum requests per day. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_day: Option<i64>,
	/// The maximum requests per minute.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_minute: Option<i64>,
	/// The maximum tokens per minute.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens_per_1_minute: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountCreateRequest {
	/// The name of the service account being created.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUpdateRequest {
	/// The updated name of the project, this name appears in reports.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserCreateRequest {
	/// `owner` or `member`
	pub role: ProjectUserCreateRequestRole,
	/// The ID of the user.
	pub user_id: String,
}
/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectUserCreateRequestRole {
	Owner,
	Member,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserUpdateRequest {
	/// `owner` or `member`
	pub role: ProjectUserUpdateRequestRole,
}
/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectUserUpdateRequestRole {
	Owner,
	Member,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRoleUpdateRequest {
	/// `owner` or `reader`
	pub role: UserRoleUpdateRequestRole,
}
/// `owner` or `reader`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRoleUpdateRequestRole {
	Owner,
	Reader,
}
