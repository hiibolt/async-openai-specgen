use super::assistants_threads::AssistantToolsCode;
use super::assistants_threads::AssistantToolsFileSearch;
use super::assistants_threads::AssistantToolsFunction;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// The type of tool being defined: `code_interpreter`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssistantToolsCodeType {
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
}
/// Overrides for the file search tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AssistantToolsFileSearchFileSearch {
	/// The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.
	/// 
	/// Note that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](https://platform.openai.com/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<FileSearchRankingOptions>,
}
/// The type of tool being defined: `file_search`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssistantToolsFileSearchType {
	#[serde(rename = "file_search")]
	FileSearch,
}
/// The type of tool being defined: `function`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssistantToolsFunctionType {
	Function,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateAssistantRequest {
	/// The description of the assistant. The maximum length is 512 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The system instructions that the assistant uses. The maximum length is 256,000 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models) for descriptions of them.
	pub model: CreateAssistantRequestModel,
	/// The name of the assistant. The maximum length is 256 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateAssistantRequestToolResources>,
	/// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateAssistantRequestItems>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateAssistantRequestItems {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CreateAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateAssistantRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
	/// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateAssistantRequestToolResourcesFileSearch {
}
/// The ranker to use for the file search. If not specified will use the `auto` ranker.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileSearchRanker {
	Auto,
	#[serde(rename = "default_2024_08_21")]
	Default20240821,
}
/// The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.
/// 
/// See the [file search tool documentation](https://platform.openai.com/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchRankingOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<FileSearchRanker>,
	/// The score threshold for the file search. All values must be a floating point number between 0 and 1.
	pub score_threshold: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ModifyAssistantRequest {
	/// The description of the assistant. The maximum length is 512 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The system instructions that the assistant uses. The maximum length is 256,000 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models) for descriptions of them.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModifyAssistantRequestModel>,
	/// The name of the assistant. The maximum length is 256 characters.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<ModifyAssistantRequestToolResources>,
	/// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ModifyAssistantRequestItems>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ModifyAssistantRequestItems {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ModifyAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ModifyAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ModifyAssistantRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
	/// Overrides the list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ModifyAssistantRequestToolResourcesFileSearch {
	/// Overrides the [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}
/// The type of response format being defined. Always `json_object`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatJsonObjectType {
	#[serde(rename = "json_object")]
	JsonObject,
}
/// Structured Outputs configuration options, including a JSON Schema.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatJsonSchemaJsonSchema {
	/// A description of what the response format is for, used by the model to
	/// determine how to respond in the format.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the response format. Must be a-z, A-Z, 0-9, or contain
	/// underscores and dashes, with a maximum length of 64.
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<ResponseFormatJsonSchemaSchema>,
	/// Whether to enable strict schema adherence when generating the output.
	/// If set to true, the model will always follow the exact schema defined
	/// in the `schema` field. Only a subset of JSON Schema is supported when
	/// `strict` is `true`. To learn more, read the [Structured Outputs
	/// guide](https://platform.openai.com/docs/guides/structured-outputs).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}
/// The type of response format being defined. Always `json_schema`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatJsonSchemaType {
	#[serde(rename = "json_schema")]
	JsonSchema,
}
/// The type of response format being defined. Always `text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatTextType {
	Text,
}
