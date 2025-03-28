use super::assistants_chat::ResponseFormatJsonSchema;
use super::assistants_chat_responses::ResponseFormatJsonObject;
use super::assistants_chat_responses::ResponseFormatText;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssistantSupportedModels {
	#[serde(rename = "o3-mini")]
	O3Mini,
	#[serde(rename = "o3-mini-2025-01-31")]
	O3Mini20250131,
	O1,
	#[serde(rename = "o1-2024-12-17")]
	O120241217,
	#[serde(rename = "gpt-4o")]
	Gpt4O,
	#[serde(rename = "gpt-4o-2024-11-20")]
	Gpt4O20241120,
	#[serde(rename = "gpt-4o-2024-08-06")]
	Gpt4O20240806,
	#[serde(rename = "gpt-4o-2024-05-13")]
	Gpt4O20240513,
	#[serde(rename = "gpt-4o-mini")]
	Gpt4OMini,
	#[serde(rename = "gpt-4o-mini-2024-07-18")]
	Gpt4OMini20240718,
	#[serde(rename = "gpt-4.5-preview")]
	Gpt45Preview,
	#[serde(rename = "gpt-4.5-preview-2025-02-27")]
	Gpt45Preview20250227,
	#[serde(rename = "gpt-4-turbo")]
	Gpt4Turbo,
	#[serde(rename = "gpt-4-turbo-2024-04-09")]
	Gpt4Turbo20240409,
	#[serde(rename = "gpt-4-0125-preview")]
	Gpt40125Preview,
	#[serde(rename = "gpt-4-turbo-preview")]
	Gpt4TurboPreview,
	#[serde(rename = "gpt-4-1106-preview")]
	Gpt41106Preview,
	#[serde(rename = "gpt-4-vision-preview")]
	Gpt4VisionPreview,
	#[serde(rename = "gpt-4")]
	Gpt4,
	#[serde(rename = "gpt-4-0314")]
	Gpt40314,
	#[serde(rename = "gpt-4-0613")]
	Gpt40613,
	#[serde(rename = "gpt-4-32k")]
	Gpt432K,
	#[serde(rename = "gpt-4-32k-0314")]
	Gpt432K0314,
	#[serde(rename = "gpt-4-32k-0613")]
	Gpt432K0613,
	#[serde(rename = "gpt-3.5-turbo")]
	Gpt35Turbo,
	#[serde(rename = "gpt-3.5-turbo-16k")]
	Gpt35Turbo16K,
	#[serde(rename = "gpt-3.5-turbo-0613")]
	Gpt35Turbo0613,
	#[serde(rename = "gpt-3.5-turbo-1106")]
	Gpt35Turbo1106,
	#[serde(rename = "gpt-3.5-turbo-0125")]
	Gpt35Turbo0125,
	#[serde(rename = "gpt-3.5-turbo-16k-0613")]
	Gpt35Turbo16K0613,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsCode {
	/// The type of tool being defined: `code_interpreter`
	pub r#type: AssistantToolsCodeType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsFileSearch {
	/// Overrides for the file search tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<AssistantToolsFileSearchFileSearch>,
	/// The type of tool being defined: `file_search`
	pub r#type: AssistantToolsFileSearchType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsFunction {
	pub function: FunctionObject,
	/// The type of tool being defined: `function`
	pub r#type: AssistantToolsFunctionType,
}
/// Specifies the format that the model must output. Compatible with [GPT-4o](https://platform.openai.com/docs/models#gpt-4o), [GPT-4 Turbo](https://platform.openai.com/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.
/// 
/// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
/// 
/// Setting to `{ "type": "json_object" }` enables JSON mode, which ensures the message the model generates is valid JSON.
/// 
/// **Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if `finish_reason="length"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
	Auto(String),
	ResponseFormatText(ResponseFormatText),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
	ResponseFormatJsonSchema(ResponseFormatJsonSchema),
}
