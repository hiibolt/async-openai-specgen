
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateModerationRequest {
	/// Input (or inputs) to classify. Can be a single string, an array of strings, or
	/// an array of multi-modal input objects similar to other models.
	pub input: CreateModerationRequestInput,
	/// The content moderation model you would like to use. Learn more in
	/// [the moderation guide](https://platform.openai.com/docs/guides/moderation), and learn about
	/// available models [here](https://platform.openai.com/docs/models#moderation).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateModerationRequestModel>,
}
/// Input (or inputs) to classify. Can be a single string, an array of strings, or
/// an array of multi-modal input objects similar to other models.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateModerationRequestInput {
	String(String),
	CreateModerationRequestInputStringArray(CreateModerationRequestInputStringArray),
	CreateModerationRequestInputVariedArray(CreateModerationRequestInputVariedArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateModerationRequestInputItems {
	Object(serde_json::Value),
}
