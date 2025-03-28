
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionObject {
	/// A description of what the function does, used by the model to choose when and how to call the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<FunctionParameters>,
	/// Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}
/// JSON Schema response format. Used to generate structured JSON responses.
/// Learn more about [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatJsonSchema {
	/// Structured Outputs configuration options, including a JSON Schema.
	pub json_schema: ResponseFormatJsonSchemaJsonSchema,
	/// The type of response format being defined. Always `json_schema`.
	pub r#type: ResponseFormatJsonSchemaType,
}
