
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// JSON object response format. An older method of generating JSON responses.
/// Using `json_schema` is recommended for models that support it. Note that the
/// model will not generate JSON without a system or user message instructing it
/// to do so.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatJsonObject {
	/// The type of response format being defined. Always `json_object`.
	pub r#type: ResponseFormatJsonObjectType,
}
/// Default response format. Used to generate text responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatText {
	/// The type of response format being defined. Always `text`.
	pub r#type: ResponseFormatTextType,
}
