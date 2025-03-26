use serde::{Serialize, Deserialize};

/// **o-series models only**
/// 
/// Configuration options for 
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Reasoning {
	/// **computer_use_preview only**
	/// 
	/// A summary of the reasoning performed by the model. This can be
	/// useful for debugging and understanding the model's reasoning process.
	/// One of `concise` or `detailed`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub generate_summary: Option<ReasoningGenerateSummary>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub effort: Option<ReasoningEffort>,
}
/// The type of the input item. Always `input_text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputTextType {
	#[serde(rename = "input_text")]
	InputText,
}
/// The detail level of the image to be sent to the model. One of `high`,
/// `low`, or `auto`. Defaults to `auto`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputImageDetail {
	High,
	Low,
	Auto,
}
/// The type of the input item. Always `input_image`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputImageType {
	#[serde(rename = "input_image")]
	InputImage,
}
/// An image input to the model. Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImage {
	/// The ID of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The URL of the image to be sent to the model. A fully qualified URL or
	/// base64 encoded image in a data URL.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/// The detail level of the image to be sent to the model. One of `high`,
	/// `low`, or `auto`. Defaults to `auto`.
	pub detail: InputImageDetail,
	/// The type of the input item. Always `input_image`.
	pub r#type: InputImageType,
}
/// A text input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputText {
	/// The type of the input item. Always `input_text`.
	pub r#type: InputTextType,
	/// The text input to the model.
	pub text: String,
}
/// The type of the input item. Always `input_file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputFileType {
	#[serde(rename = "input_file")]
	InputFile,
}
/// **computer_use_preview only**
/// 
/// A summary of the reasoning performed by the model. This can be
/// useful for debugging and understanding the model's reasoning process.
/// One of `concise` or `detailed`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningGenerateSummary {
	Concise,
	Detailed,
}
/// A file input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFile {
	/// The type of the input item. Always `input_file`.
	pub r#type: InputFileType,
	/// The name of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	/// The content of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_data: Option<String>,
	/// The ID of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}
