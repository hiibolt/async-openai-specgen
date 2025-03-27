use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A filter used to compare a specified attribute key to a given value using a defined comparison operation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComparisonFilter {
	/// The key to compare against the value.
	pub key: String,
	/// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
	/// - `eq`: equals
	/// - `ne`: not equal
	/// - `gt`: greater than
	/// - `gte`: greater than or equal
	/// - `lt`: less than
	/// - `lte`: less than or equal
	pub r#type: ComparisonFilterType,
	/// The value to compare against the attribute key; supports string, number, or boolean types.
	pub value: ComparisonFilterValue,
}
/// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
/// - `eq`: equals
/// - `ne`: not equal
/// - `gt`: greater than
/// - `gte`: greater than or equal
/// - `lt`: less than
/// - `lte`: less than or equal
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComparisonFilterType {
	Eq,
	Ne,
	Gt,
	Gte,
	Lt,
	Lte,
}
/// The value to compare against the attribute key; supports string, number, or boolean types.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComparisonFilterValue {
	String(String),
	Number(f64),
	Boolean(bool),
}
/// Combine multiple filters using `and` or `or`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompoundFilter {
	/// Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`.
	pub filters: Vec<CompoundFilterFilters>,
	/// Type of operation: `and` or `or`.
	pub r#type: CompoundFilterType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CompoundFilterFilters {
	ComparisonFilter(ComparisonFilter),
	Object(serde_json::Value),
}
/// Type of operation: `and` or `or`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CompoundFilterType {
	And,
	Or,
}
/// A tool that controls a virtual computer. Learn more about the 
/// [computer tool](https://platform.openai.com/docs/guides/tools-computer-use).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerTool {
	/// The height of the computer display.
	pub display_height: f64,
	/// The width of the computer display.
	pub display_width: f64,
	/// The type of computer environment to control.
	pub environment: ComputerToolEnvironment,
	/// The type of the computer use tool. Always `computer_use_preview`.
	pub r#type: ComputerToolType,
}
/// The type of computer environment to control.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComputerToolEnvironment {
	Mac,
	Windows,
	Ubuntu,
	Browser,
}
/// The type of the computer use tool. Always `computer_use_preview`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComputerToolType {
	#[serde(rename = "computer_use_preview")]
	ComputerUsePreview,
}
/// A tool that searches for relevant content from uploaded files.
/// Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchTool {
	/// A filter to apply based on file attributes.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filters: Option<FileSearchToolFilters>,
	/// The maximum number of results to return. This number should be between 1 
	/// and 50 inclusive.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<i64>,
	/// Ranking options for search.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<FileSearchToolRankingOptions>,
	/// The type of the file search tool. Always `file_search`.
	pub r#type: FileSearchToolType,
	/// The IDs of the vector stores to search.
	pub vector_store_ids: Vec<String>,
}
/// A filter to apply based on file attributes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FileSearchToolFilters {
	ComparisonFilter(ComparisonFilter),
	CompoundFilter(CompoundFilter),
}
/// Ranking options for search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchToolRankingOptions {
	/// The ranker to use for the file search.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<FileSearchToolRankingOptionsRanker>,
	/// The score threshold for the file search, a number between 0 and 1.
	/// Numbers closer to 1 will attempt to return only the most relevant
	/// results, but may return fewer results.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score_threshold: Option<f64>,
}
/// The ranker to use for the file search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FileSearchToolRankingOptionsRanker {
	Auto,
	#[serde(rename = "default-2024-11-15")]
	Default20241115,
}
/// The type of the file search tool. Always `file_search`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FileSearchToolType {
	#[serde(rename = "file_search")]
	FileSearch,
}
/// Defines a function in your own code the model can choose to call. Learn more
/// about [function calling](https://platform.openai.com/docs/guides/function-calling).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionTool {
	/// A description of the function. Used by the model to determine whether
	/// or not to call the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function to call.
	pub name: String,
	/// A JSON schema object describing the parameters of the function.
	pub parameters: FunctionToolParameters,
	/// Whether to enforce strict parameter validation. Default `true`.
	pub strict: bool,
	/// The type of the function tool. Always `function`.
	pub r#type: FunctionToolType,
}
/// The type of the function tool. Always `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionToolType {
	Function,
}
/// **o-series models only**
/// 
/// Configuration options for 
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Reasoning {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub effort: Option<ReasoningEffort>,
	/// **computer_use_preview only**
	/// 
	/// A summary of the reasoning performed by the model. This can be
	/// useful for debugging and understanding the model's reasoning process.
	/// One of `concise` or `detailed`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub generate_summary: Option<ReasoningGenerateSummary>,
}
/// **o-series models only** 
/// 
/// Constrains effort on reasoning for 
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
/// Currently supported values are `low`, `medium`, and `high`. Reducing
/// reasoning effort can result in faster responses and fewer tokens used
/// on reasoning in a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ReasoningEffort {
	Low,
	Medium,
	High,
}
/// **computer_use_preview only**
/// 
/// A summary of the reasoning performed by the model. This can be
/// useful for debugging and understanding the model's reasoning process.
/// One of `concise` or `detailed`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ReasoningGenerateSummary {
	Concise,
	Detailed,
}
/// JSON object response format. An older method of generating JSON responses.
/// Using `json_schema` is recommended for models that support it. Note that the
/// model will not generate JSON without a system or user message instructing it
/// to do so.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatJsonObject {
	/// The type of response format being defined. Always `json_object`.
	pub r#type: ResponseFormatJsonObjectType,
}
/// The type of response format being defined. Always `json_object`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseFormatJsonObjectType {
	#[serde(rename = "json_object")]
	JsonObject,
}
/// Default response format. Used to generate text responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatText {
	/// The type of response format being defined. Always `text`.
	pub r#type: ResponseFormatTextType,
}
/// The type of response format being defined. Always `text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseFormatTextType {
	Text,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseProperties {
	/// Inserts a system (or developer) message as the first item in the model's context.
	/// 
	/// When using along with `previous_response_id`, the instructions from a previous
	/// response will be not be carried over to the next response. This makes it simple
	/// to swap out system (or developer) messages in new responses.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_output_tokens: Option<i64>,
	/// Model ID used to generate the response, like `gpt-4o` or `o1`. OpenAI
	/// offers a wide range of models with different capabilities, performance
	/// characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models)
	/// to browse and compare available models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModelIdsResponses>,
	/// The unique ID of the previous response to the model. Use this to
	/// create multi-turn conversations. Learn more about 
	/// [conversation state](https://platform.openai.com/docs/guides/conversation-state).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_response_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning: Option<Reasoning>,
	/// Configuration options for a text response from the model. Can be plain
	/// text or structured JSON data. Learn more:
	/// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
	/// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<ResponsePropertiesText>,
	/// How the model should select which tool (or tools) to use when generating
	/// a response. See the `tools` parameter to see how to specify which tools
	/// the model can call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<ResponsePropertiesToolChoice>,
	/// An array of tools the model may call while generating a response. You 
	/// can specify which tool to use by setting the `tool_choice` parameter.
	/// 
	/// The two categories of tools you can provide the model are:
	/// 
	/// - **Built-in tools**: Tools that are provided by OpenAI that extend the
	///   model's capabilities, like [web search](https://platform.openai.com/docs/guides/tools-web-search)
	///   or [file search](https://platform.openai.com/docs/guides/tools-file-search). Learn more about
	///   [built-in tools](https://platform.openai.com/docs/guides/tools).
	/// - **Function calls (custom tools)**: Functions that are defined by you,
	///   enabling the model to call your own code. Learn more about
	///   [function calling](https://platform.openai.com/docs/guides/function-calling).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<Tool>>,
	/// The truncation strategy to use for the model response.
	/// - `auto`: If the context of this response and previous ones exceeds
	///   the model's context window size, the model will truncate the 
	///   response to fit the context window by dropping input items in the
	///   middle of the conversation. 
	/// - `disabled` (default): If a model response will exceed the context window 
	///   size for a model, the request will fail with a 400 error.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation: Option<ResponsePropertiesTruncation>,
}
/// Configuration options for a text response from the model. Can be plain
/// text or structured JSON data. Learn more:
/// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
/// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponsePropertiesText {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<TextResponseFormatConfiguration>,
}
/// How the model should select which tool (or tools) to use when generating
/// a response. See the `tools` parameter to see how to specify which tools
/// the model can call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponsePropertiesToolChoice {
	ToolChoiceOptions(ToolChoiceOptions),
	ToolChoiceTypes(ToolChoiceTypes),
	ToolChoiceFunction(ToolChoiceFunction),
}
/// The truncation strategy to use for the model response.
/// - `auto`: If the context of this response and previous ones exceeds
///   the model's context window size, the model will truncate the 
///   response to fit the context window by dropping input items in the
///   middle of the conversation. 
/// - `disabled` (default): If a model response will exceed the context window 
///   size for a model, the request will fail with a 400 error.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponsePropertiesTruncation {
	Auto,
	Disabled,
}
/// An object specifying the format that the model must output.
/// 
/// Configuring `{ "type": "json_schema" }` enables Structured Outputs, 
/// which ensures the model will match your supplied JSON schema. Learn more in the 
/// [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
/// 
/// The default format is `{ "type": "text" }` with no additional options.
/// 
/// **Not recommended for gpt-4o and newer models:**
/// 
/// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
/// ensures the message the model generates is valid JSON. Using `json_schema`
/// is preferred for models that support it.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum TextResponseFormatConfiguration {
	ResponseFormatText(ResponseFormatText),
	TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
}
/// JSON Schema response format. Used to generate structured JSON responses.
/// Learn more about [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextResponseFormatJsonSchema {
	/// A description of what the response format is for, used by the model to
	/// determine how to respond in the format.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the response format. Must be a-z, A-Z, 0-9, or contain
	/// underscores and dashes, with a maximum length of 64.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	pub schema: ResponseFormatJsonSchemaSchema,
	/// Whether to enable strict schema adherence when generating the output.
	/// If set to true, the model will always follow the exact schema defined
	/// in the `schema` field. Only a subset of JSON Schema is supported when
	/// `strict` is `true`. To learn more, read the [Structured Outputs
	/// guide](https://platform.openai.com/docs/guides/structured-outputs).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
	/// The type of response format being defined. Always `json_schema`.
	pub r#type: TextResponseFormatJsonSchemaType,
}
/// The type of response format being defined. Always `json_schema`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum TextResponseFormatJsonSchemaType {
	#[serde(rename = "json_schema")]
	JsonSchema,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Tool {
	FileSearchTool(FileSearchTool),
	FunctionTool(FunctionTool),
	ComputerTool(ComputerTool),
	WebSearchTool(WebSearchTool),
}
/// Use this option to force the model to call a specific function.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceFunction {
	/// The name of the function to call.
	pub name: String,
	/// For function calling, the type is always `function`.
	pub r#type: ToolChoiceFunctionType,
}
/// For function calling, the type is always `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ToolChoiceFunctionType {
	Function,
}
/// Controls which (if any) tool is called by the model.
/// 
/// `none` means the model will not call any tool and instead generates a message.
/// 
/// `auto` means the model can pick between generating a message or calling one or
/// more tools.
/// 
/// `required` means the model must call one or more tools.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ToolChoiceOptions {
	None,
	Auto,
	Required,
}
/// Indicates that the model should use a built-in tool to generate a response.
/// [Learn more about built-in tools](https://platform.openai.com/docs/guides/tools).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceTypes {
	/// The type of hosted tool the model should to use. Learn more about
	/// [built-in tools](https://platform.openai.com/docs/guides/tools).
	/// 
	/// Allowed values are:
	/// - `file_search`
	/// - `web_search_preview`
	/// - `computer_use_preview`
	pub r#type: ToolChoiceTypesType,
}
/// The type of hosted tool the model should to use. Learn more about
/// [built-in tools](https://platform.openai.com/docs/guides/tools).
/// 
/// Allowed values are:
/// - `file_search`
/// - `web_search_preview`
/// - `computer_use_preview`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ToolChoiceTypesType {
	#[serde(rename = "file_search")]
	FileSearch,
	#[serde(rename = "web_search_preview")]
	WebSearchPreview,
	#[serde(rename = "computer_use_preview")]
	ComputerUsePreview,
	#[serde(rename = "web_search_preview_2025_03_11")]
	WebSearchPreview20250311,
}
/// High level guidance for the amount of context window space to use for the 
/// search. One of `low`, `medium`, or `high`. `medium` is the default.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum WebSearchContextSize {
	Low,
	Medium,
	High,
}
/// Approximate location parameters for the search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchLocation {
	/// Free text input for the city of the user, e.g. `San Francisco`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	/// The two-letter 
	/// [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,
	/// e.g. `US`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	/// Free text input for the region of the user, e.g. `California`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
	/// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) 
	/// of the user, e.g. `America/Los_Angeles`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timezone: Option<String>,
}
/// This tool searches the web for relevant results to use in a response.
/// Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchTool {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_context_size: Option<WebSearchContextSize>,
	/// The type of the web search tool. One of:
	/// - `web_search_preview`
	/// - `web_search_preview_2025_03_11`
	pub r#type: WebSearchToolType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_location: Option<WebSearchToolUserLocation>,
}
/// The type of the web search tool. One of:
/// - `web_search_preview`
/// - `web_search_preview_2025_03_11`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum WebSearchToolType {
	#[serde(rename = "web_search_preview")]
	WebSearchPreview,
	#[serde(rename = "web_search_preview_2025_03_11")]
	WebSearchPreview20250311,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchToolUserLocation {
	/// Free text input for the city of the user, e.g. `San Francisco`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	/// The two-letter 
	/// [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,
	/// e.g. `US`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	/// Free text input for the region of the user, e.g. `California`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
	/// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) 
	/// of the user, e.g. `America/Los_Angeles`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timezone: Option<String>,
	/// The type of location approximation. Always `approximate`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<WebSearchToolUserLocationType>,
}
/// The type of location approximation. Always `approximate`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum WebSearchToolUserLocationType {
	Approximate,
}
pub type FunctionToolParameters = serde_json::Value;
pub type Metadata = HashMap<String, String>;
pub type ModelIdsResponses = serde_json::Value;
pub type ModelIdsShared = serde_json::Value;
pub type ResponseFormatJsonSchemaSchema = serde_json::Value;
