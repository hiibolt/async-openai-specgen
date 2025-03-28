
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompleteUploadRequest {
	/// The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub md5: Option<String>,
	/// The ordered list of Part IDs.
	pub part_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateUploadRequest {
	/// The number of bytes in the file you are uploading.
	pub bytes: i64,
	/// The name of the file to upload.
	pub filename: String,
	/// The MIME type of the file.
	/// 
	/// This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.
	pub mime_type: String,
	/// The intended purpose of the uploaded file.
	/// 
	/// See the [documentation on File purposes](https://platform.openai.com/docs/api-reference/files/create#files-create-purpose).
	pub purpose: CreateUploadRequestPurpose,
}
/// The intended purpose of the uploaded file.
/// 
/// See the [documentation on File purposes](https://platform.openai.com/docs/api-reference/files/create#files-create-purpose).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateUploadRequestPurpose {
	Assistants,
	Batch,
	#[serde(rename = "fine-tune")]
	FineTune,
	Vision,
}
