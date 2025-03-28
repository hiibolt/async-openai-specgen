
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateImageRequest {
	/// The model to use for image generation.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageRequestModel>,
	/// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i64>,
	/// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
	pub prompt: String,
	/// The quality of the image that will be generated. `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quality: Option<CreateImageRequestQuality>,
	/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageRequestResponseFormat>,
	/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`. Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageRequestSize>,
	/// The style of the generated images. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for `dall-e-3`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub style: Option<CreateImageRequestStyle>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// The quality of the image that will be generated. `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestQuality {
	Standard,
	Hd,
}
/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestResponseFormat {
	Url,
	#[serde(rename = "b64_json")]
	B64Json,
}
/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`. Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestSize {
	#[serde(rename = "256x256")]
	Type256x256,
	#[serde(rename = "512x512")]
	Type512x512,
	#[serde(rename = "1024x1024")]
	Type1024x1024,
	#[serde(rename = "1792x1024")]
	Type1792x1024,
	#[serde(rename = "1024x1792")]
	Type1024x1792,
}
/// The style of the generated images. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for `dall-e-3`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateImageRequestStyle {
	Vivid,
	Natural,
}
