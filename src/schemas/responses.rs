use super::aliases::InputMessageContentList;
use super::assistants_chat_responses::ResponseFormatJsonObject;
use super::assistants_chat_responses::ResponseFormatText;
use super::chat_responses::CreateModelResponseProperties;
use super::chat_responses::WebSearchLocation;
use super::responses_vector_stores::ComparisonFilter;
use super::responses_vector_stores::CompoundFilter;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Annotation {
	FileCitation(FileCitation),
	UrlCitation(UrlCitation),
	FilePath(FilePath),
}
/// A click action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Click {
	/// Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.
	pub button: ClickButton,
	/// Specifies the event type. For a click action, this property is 
	/// always set to `click`.
	pub r#type: ClickType,
	/// The x-coordinate where the click occurred.
	pub x: i64,
	/// The y-coordinate where the click occurred.
	pub y: i64,
}
/// Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClickButton {
	Left,
	Right,
	Wheel,
	Back,
	Forward,
}
/// Specifies the event type. For a click action, this property is 
/// always set to `click`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClickType {
	Click,
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CompoundFilterItems {
	ComparisonFilter(ComparisonFilter),
	Object(serde_json::Value),
}
/// Type of operation: `and` or `or`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompoundFilterType {
	And,
	Or,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComputerAction {
	Click(Click),
	DoubleClick(DoubleClick),
	Drag(Drag),
	KeyPress(KeyPress),
	Move(Move),
	Screenshot(Screenshot),
	Scroll(Scroll),
	Type(Type),
	Wait(Wait),
}
/// A computer screenshot image used with the computer use tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerScreenshotImage {
	/// The identifier of an uploaded file that contains the screenshot.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The URL of the screenshot image.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/// Specifies the event type. For a computer screenshot, this property is 
	/// always set to `computer_screenshot`.
	pub r#type: ComputerScreenshotImageType,
}
/// Specifies the event type. For a computer screenshot, this property is 
/// always set to `computer_screenshot`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerScreenshotImageType {
	#[serde(rename = "computer_screenshot")]
	ComputerScreenshot,
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
/// A tool call to a computer use tool. See the 
/// [computer use guide](https://platform.openai.com/docs/guides/tools-computer-use) for more information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerToolCall {
	pub action: ComputerAction,
	/// An identifier used when responding to the tool call with output.
	pub call_id: String,
	/// The unique ID of the computer call.
	pub id: String,
	/// The pending safety checks for the computer call.
	pub pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
	/// The status of the item. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when items are returned via API.
	pub status: ComputerToolCallStatus,
	/// The type of the computer call. Always `computer_call`.
	pub r#type: ComputerToolCallType,
}
/// The output of a computer tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerToolCallOutput {
	/// The safety checks reported by the API that have been acknowledged by the 
	/// developer.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,
	/// The ID of the computer tool call that produced the output.
	pub call_id: String,
	/// The ID of the computer tool call output.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	pub output: ComputerScreenshotImage,
	/// The status of the message input. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when input items are returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ComputerToolCallOutputStatus>,
	/// The type of the computer tool call output. Always `computer_call_output`.
	pub r#type: ComputerToolCallOutputType,
}
/// The status of the message input. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when input items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolCallOutputStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the computer tool call output. Always `computer_call_output`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolCallOutputType {
	#[serde(rename = "computer_call_output")]
	ComputerCallOutput,
}
/// A pending safety check for the computer call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerToolCallSafetyCheck {
	/// The type of the pending safety check.
	pub code: String,
	/// The ID of the pending safety check.
	pub id: String,
	/// Details about the pending safety check.
	pub message: String,
}
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the computer call. Always `computer_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolCallType {
	#[serde(rename = "computer_call")]
	ComputerCall,
}
/// The type of computer environment to control.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolEnvironment {
	Mac,
	Windows,
	Ubuntu,
	Browser,
}
/// The type of the computer use tool. Always `computer_use_preview`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComputerToolType {
	#[serde(rename = "computer_use_preview")]
	ComputerUsePreview,
}
/// An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Coordinate {
	/// The x-coordinate.
	pub x: i64,
	/// The y-coordinate.
	pub y: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CreateResponse {
	/// Specify additional output data to include in the model response. Currently
	/// supported values are:
	/// - `file_search_call.results`: Include the search results of
	///   the file search tool call.
	/// - `message.input_image.image_url`: Include image urls from the input message.
	/// - `computer_call_output.output.image_url`: Include image urls from the computer call output.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<Includable>>,
	/// Text, image, or file inputs to the model, used to generate a response.
	/// 
	/// Learn more:
	/// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
	/// - [Image inputs](https://platform.openai.com/docs/guides/images)
	/// - [File inputs](https://platform.openai.com/docs/guides/pdf-files)
	/// - [Conversation state](https://platform.openai.com/docs/guides/conversation-state)
	/// - [Function calling](https://platform.openai.com/docs/guides/function-calling)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<CreateResponseInput>,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// Model ID used to generate the response, like `gpt-4o` or `o1`. OpenAI
	/// offers a wide range of models with different capabilities, performance
	/// characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models)
	/// to browse and compare available models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModelIdsResponses>,
	/// Whether to allow the model to run tool calls in parallel.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<bool>,
	/// The unique ID of the previous response to the model. Use this to
	/// create multi-turn conversations. Learn more about 
	/// [conversation state](https://platform.openai.com/docs/guides/conversation-state).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_response_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning: Option<Reasoning>,
	/// Whether to store the generated model response for later retrieval via
	/// API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store: Option<bool>,
	/// If set to true, the model response data will be streamed to the client
	/// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
	/// See the [Streaming section below](https://platform.openai.com/docs/api-reference/responses-streaming)
	/// for more information.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	/// We generally recommend altering this or `top_p` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
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
	/// An alternative to sampling with temperature, called nucleus sampling,
	/// where the model considers the results of the tokens with top_p probability
	/// mass. So 0.1 means only the tokens comprising the top 10% probability mass
	/// are considered.
	/// 
	/// We generally recommend altering this or `temperature` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	/// The truncation strategy to use for the model response.
	/// - `auto`: If the context of this response and previous ones exceeds
	///   the model's context window size, the model will truncate the 
	///   response to fit the context window by dropping input items in the
	///   middle of the conversation. 
	/// - `disabled` (default): If a model response will exceed the context window 
	///   size for a model, the request will fail with a 400 error.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation: Option<ResponsePropertiesTruncation>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// Text, image, or file inputs to the model, used to generate a response.
/// 
/// Learn more:
/// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
/// - [Image inputs](https://platform.openai.com/docs/guides/images)
/// - [File inputs](https://platform.openai.com/docs/guides/pdf-files)
/// - [Conversation state](https://platform.openai.com/docs/guides/conversation-state)
/// - [Function calling](https://platform.openai.com/docs/guides/function-calling)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateResponseInput {
	String(String),
	CreateResponseInputInputItemArray(CreateResponseInputInputItemArray),
}
/// A double click action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DoubleClick {
	/// Specifies the event type. For a double click action, this property is 
	/// always set to `double_click`.
	pub r#type: DoubleClickType,
	/// The x-coordinate where the double click occurred.
	pub x: i64,
	/// The y-coordinate where the double click occurred.
	pub y: i64,
}
/// Specifies the event type. For a double click action, this property is 
/// always set to `double_click`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DoubleClickType {
	#[serde(rename = "double_click")]
	DoubleClick,
}
/// A drag action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Drag {
	/// An array of coordinates representing the path of the drag action. Coordinates will appear as an array
	/// of objects, eg
	/// ```
	/// [
	///   { x: 100, y: 200 },
	///   { x: 200, y: 300 }
	/// ]
	/// ```
	pub path: Vec<Coordinate>,
	/// Specifies the event type. For a drag action, this property is 
	/// always set to `drag`.
	pub r#type: DragType,
}
/// Specifies the event type. For a drag action, this property is 
/// always set to `drag`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DragType {
	Drag,
}
/// A message input to the model with a role indicating instruction following
/// hierarchy. Instructions given with the `developer` or `system` role take
/// precedence over instructions given with the `user` role. Messages with the
/// `assistant` role are presumed to have been generated by the model in previous
/// interactions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EasyInputMessage {
	/// Text, image, or audio input to the model, used to generate a response.
	/// Can also contain previous assistant responses.
	pub content: EasyInputMessageContent,
	/// The role of the message input. One of `user`, `assistant`, `system`, or
	/// `developer`.
	pub role: EasyInputMessageRole,
	/// The type of the message input. Always `message`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<EasyInputMessageType>,
}
/// Text, image, or audio input to the model, used to generate a response.
/// Can also contain previous assistant responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum EasyInputMessageContent {
	String(String),
	InputMessageContentList(InputMessageContentList),
}
/// The role of the message input. One of `user`, `assistant`, `system`, or
/// `developer`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EasyInputMessageRole {
	User,
	Assistant,
	System,
	Developer,
}
/// The type of the message input. Always `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EasyInputMessageType {
	Message,
}
/// A citation to a file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileCitation {
	/// The ID of the file.
	pub file_id: String,
	/// The index of the file in the list of files.
	pub index: i64,
	/// The type of the file citation. Always `file_citation`.
	pub r#type: FileCitationType,
}
/// The type of the file citation. Always `file_citation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileCitationType {
	#[serde(rename = "file_citation")]
	FileCitation,
}
/// A path to a file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FilePath {
	/// The ID of the file.
	pub file_id: String,
	/// The index of the file in the list of files.
	pub index: i64,
	/// The type of the file path. Always `file_path`.
	pub r#type: FilePathType,
}
/// The type of the file path. Always `file_path`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilePathType {
	#[serde(rename = "file_path")]
	FilePath,
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
	pub ranking_options: Option<HashMap<String, String>>,
	/// The type of the file search tool. Always `file_search`.
	pub r#type: FileSearchToolType,
	/// The IDs of the vector stores to search.
	pub vector_store_ids: Vec<String>,
}
/// The results of a file search tool call. See the 
/// [file search guide](https://platform.openai.com/docs/guides/tools-file-search) for more information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchToolCall {
	/// The unique ID of the file search tool call.
	pub id: String,
	/// The queries used to search for files.
	pub queries: Vec<String>,
	/// The results of the file search tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<Vec<FileSearchToolCallResultsItem>>,
	/// The status of the file search tool call. One of `in_progress`, 
	/// `searching`, `incomplete` or `failed`,
	pub status: FileSearchToolCallStatus,
	/// The type of the file search tool call. Always `file_search_call`.
	pub r#type: FileSearchToolCallType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FileSearchToolCallResultsItem {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
	/// The unique ID of the file.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The name of the file.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	/// The relevance score of the file - a value between 0 and 1.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score: Option<f64>,
	/// The text that was retrieved from the file.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
}
/// The status of the file search tool call. One of `in_progress`, 
/// `searching`, `incomplete` or `failed`,
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileSearchToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Searching,
	Completed,
	Incomplete,
	Failed,
}
/// The type of the file search tool call. Always `file_search_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileSearchToolCallType {
	#[serde(rename = "file_search_call")]
	FileSearchCall,
}
/// A filter to apply based on file attributes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FileSearchToolFilters {
	ComparisonFilter(ComparisonFilter),
	CompoundFilter(CompoundFilter),
}
/// The type of the file search tool. Always `file_search`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
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
	pub parameters: serde_json::Value,
	/// Whether to enforce strict parameter validation. Default `true`.
	pub strict: bool,
	/// The type of the function tool. Always `function`.
	pub r#type: FunctionToolType,
}
/// A tool call to run a function. See the 
/// [function calling guide](https://platform.openai.com/docs/guides/function-calling) for more information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCall {
	/// A JSON string of the arguments to pass to the function.
	pub arguments: String,
	/// The unique ID of the function tool call generated by the model.
	pub call_id: String,
	/// The unique ID of the function tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The name of the function to run.
	pub name: String,
	/// The status of the item. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when items are returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<FunctionToolCallStatus>,
	/// The type of the function tool call. Always `function_call`.
	pub r#type: FunctionToolCallType,
}
/// The output of a function tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCallOutput {
	/// The unique ID of the function tool call generated by the model.
	pub call_id: String,
	/// The unique ID of the function tool call output. Populated when this item
	/// is returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// A JSON string of the output of the function tool call.
	pub output: String,
	/// The status of the item. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when items are returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<FunctionToolCallOutputStatus>,
	/// The type of the function tool call output. Always `function_call_output`.
	pub r#type: FunctionToolCallOutputType,
}
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionToolCallOutputStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the function tool call output. Always `function_call_output`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionToolCallOutputType {
	#[serde(rename = "function_call_output")]
	FunctionCallOutput,
}
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the function tool call. Always `function_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionToolCallType {
	#[serde(rename = "function_call")]
	FunctionCall,
}
/// The type of the function tool. Always `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionToolType {
	Function,
}
/// Specify additional output data to include in the model response. Currently
/// supported values are:
/// - `file_search_call.results`: Include the search results of
///   the file search tool call.
/// - `message.input_image.image_url`: Include image urls from the input message.
/// - `computer_call_output.output.image_url`: Include image urls from the computer call output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Includable {
	#[serde(rename = "file_search_call.results")]
	FileSearchCallResults,
	#[serde(rename = "message.input_image.image_url")]
	MessageInputImageImageUrl,
	#[serde(rename = "computer_call_output.output.image_url")]
	ComputerCallOutputOutputImageUrl,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InputContent {
	InputText(InputText),
	InputImage(InputImage),
	InputFile(InputFile),
}
/// A file input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFile {
	/// The content of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_data: Option<String>,
	/// The ID of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The name of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	/// The type of the input item. Always `input_file`.
	pub r#type: InputFileType,
}
/// The type of the input item. Always `input_file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputFileType {
	#[serde(rename = "input_file")]
	InputFile,
}
/// An image input to the model. Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImage {
	/// The detail level of the image to be sent to the model. One of `high`,
	/// `low`, or `auto`. Defaults to `auto`.
	pub detail: InputImageDetail,
	/// The ID of the file to be sent to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The URL of the image to be sent to the model. A fully qualified URL or
	/// base64 encoded image in a data URL.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/// The type of the input item. Always `input_image`.
	pub r#type: InputImageType,
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InputItem {
	EasyInputMessage(EasyInputMessage),
	Item(Item),
	ItemReference(ItemReference),
}
/// A message input to the model with a role indicating instruction following
/// hierarchy. Instructions given with the `developer` or `system` role take
/// precedence over instructions given with the `user` role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputMessage {
	pub content: InputMessageContentList,
	/// The role of the message input. One of `user`, `system`, or `developer`.
	pub role: InputMessageRole,
	/// The status of item. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when items are returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<InputMessageStatus>,
	/// The type of the message input. Always set to `message`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<InputMessageType>,
}
/// The role of the message input. One of `user`, `system`, or `developer`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputMessageRole {
	User,
	System,
	Developer,
}
/// The status of item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputMessageStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the message input. Always set to `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputMessageType {
	Message,
}
/// A text input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputText {
	/// The text input to the model.
	pub text: String,
	/// The type of the input item. Always `input_text`.
	pub r#type: InputTextType,
}
/// The type of the input item. Always `input_text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputTextType {
	#[serde(rename = "input_text")]
	InputText,
}
/// Content item used to generate a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Item {
	InputMessage(InputMessage),
	OutputMessage(OutputMessage),
	FileSearchToolCall(FileSearchToolCall),
	ComputerToolCall(ComputerToolCall),
	ComputerToolCallOutput(ComputerToolCallOutput),
	WebSearchToolCall(WebSearchToolCall),
	FunctionToolCall(FunctionToolCall),
	FunctionToolCallOutput(FunctionToolCallOutput),
	ReasoningItem(ReasoningItem),
}
/// An internal identifier for an item to reference.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemReference {
	/// The ID of the item to reference.
	pub id: String,
	/// The type of item to reference. Always `item_reference`.
	pub r#type: ItemReferenceType,
}
/// The type of item to reference. Always `item_reference`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemReferenceType {
	#[serde(rename = "item_reference")]
	ItemReference,
}
/// A collection of keypresses the model would like to perform.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeyPress {
	/// The combination of keys the model is requesting to be pressed. This is an
	/// array of strings, each representing a key.
	pub keys: Vec<String>,
	/// Specifies the event type. For a keypress action, this property is 
	/// always set to `keypress`.
	pub r#type: KeyPressType,
}
/// Specifies the event type. For a keypress action, this property is 
/// always set to `keypress`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KeyPressType {
	Keypress,
}
/// A mouse move action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Move {
	/// Specifies the event type. For a move action, this property is 
	/// always set to `move`.
	pub r#type: MoveType,
	/// The x-coordinate to move to.
	pub x: i64,
	/// The y-coordinate to move to.
	pub y: i64,
}
/// Specifies the event type. For a move action, this property is 
/// always set to `move`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MoveType {
	Move,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OutputContent {
	OutputText(OutputText),
	Refusal(Refusal),
}
/// An output message from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputMessage {
	/// The content of the output message.
	pub content: Vec<OutputContent>,
	/// The unique ID of the output message.
	pub id: String,
	/// The role of the output message. Always `assistant`.
	pub role: OutputMessageRole,
	/// The status of the message input. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when input items are returned via API.
	pub status: OutputMessageStatus,
	/// The type of the output message. Always `message`.
	pub r#type: OutputMessageType,
}
/// The role of the output message. Always `assistant`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMessageRole {
	Assistant,
}
/// The status of the message input. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when input items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMessageStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the output message. Always `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMessageType {
	Message,
}
/// A text output from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputText {
	/// The annotations of the text output.
	pub annotations: Vec<Annotation>,
	/// The text output from the model.
	pub text: String,
	/// The type of the output text. Always `output_text`.
	pub r#type: OutputTextType,
}
/// The type of the output text. Always `output_text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputTextType {
	#[serde(rename = "output_text")]
	OutputText,
}
/// **o-series models only**
/// 
/// Configuration options for 
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
/// A description of the chain of thought used by a reasoning model while generating
/// a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItem {
	/// The unique identifier of the reasoning content.
	pub id: String,
	/// The status of the item. One of `in_progress`, `completed`, or
	/// `incomplete`. Populated when items are returned via API.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ReasoningItemStatus>,
	/// Reasoning text contents.
	pub summary: Vec<ReasoningItemSummaryItem>,
	/// The type of the object. Always `reasoning`.
	pub r#type: ReasoningItemType,
}
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningItemStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItemSummaryItem {
	/// A short summary of the reasoning used by the model when generating
	/// the response.
	pub text: String,
	/// The type of the object. Always `summary_text`.
	pub r#type: ReasoningItemSummaryItemType,
}
/// The type of the object. Always `summary_text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningItemSummaryItemType {
	#[serde(rename = "summary_text")]
	SummaryText,
}
/// The type of the object. Always `reasoning`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningItemType {
	Reasoning,
}
/// A refusal from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Refusal {
	/// The refusal explanationfrom the model.
	pub refusal: String,
	/// The type of the refusal. Always `refusal`.
	pub r#type: RefusalType,
}
/// The type of the refusal. Always `refusal`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RefusalType {
	Refusal,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
pub enum ResponsePropertiesTruncation {
	Auto,
	Disabled,
}
/// A screenshot action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Screenshot {
	/// Specifies the event type. For a screenshot action, this property is 
	/// always set to `screenshot`.
	pub r#type: ScreenshotType,
}
/// Specifies the event type. For a screenshot action, this property is 
/// always set to `screenshot`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScreenshotType {
	Screenshot,
}
/// A scroll action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Scroll {
	/// The horizontal scroll distance.
	pub scroll_x: i64,
	/// The vertical scroll distance.
	pub scroll_y: i64,
	/// Specifies the event type. For a scroll action, this property is 
	/// always set to `scroll`.
	pub r#type: ScrollType,
	/// The x-coordinate where the scroll occurred.
	pub x: i64,
	/// The y-coordinate where the scroll occurred.
	pub y: i64,
}
/// Specifies the event type. For a scroll action, this property is 
/// always set to `scroll`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScrollType {
	Scroll,
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
/// An action to type in text.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Type {
	/// The text to type.
	pub text: String,
	/// Specifies the event type. For a type action, this property is 
	/// always set to `type`.
	pub r#type: TypeType,
}
/// Specifies the event type. For a type action, this property is 
/// always set to `type`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TypeType {
	Type,
}
/// A citation for a web resource used to generate a model response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UrlCitation {
	/// The index of the last character of the URL citation in the message.
	pub end_index: i64,
	/// The index of the first character of the URL citation in the message.
	pub start_index: i64,
	/// The title of the web resource.
	pub title: String,
	/// The type of the URL citation. Always `url_citation`.
	pub r#type: UrlCitationType,
	/// The URL of the web resource.
	pub url: String,
}
/// The type of the URL citation. Always `url_citation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UrlCitationType {
	#[serde(rename = "url_citation")]
	UrlCitation,
}
/// A wait action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Wait {
	/// Specifies the event type. For a wait action, this property is 
	/// always set to `wait`.
	pub r#type: WaitType,
}
/// Specifies the event type. For a wait action, this property is 
/// always set to `wait`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WaitType {
	Wait,
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
/// The results of a web search tool call. See the 
/// [web search guide](https://platform.openai.com/docs/guides/tools-web-search) for more information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchToolCall {
	/// The unique ID of the web search tool call.
	pub id: String,
	/// The status of the web search tool call.
	pub status: WebSearchToolCallStatus,
	/// The type of the web search tool call. Always `web_search_call`.
	pub r#type: WebSearchToolCallType,
}
/// The status of the web search tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Searching,
	Completed,
	Failed,
}
/// The type of the web search tool call. Always `web_search_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchToolCallType {
	#[serde(rename = "web_search_call")]
	WebSearchCall,
}
/// The type of the web search tool. One of:
/// - `web_search_preview`
/// - `web_search_preview_2025_03_11`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchToolType {
	#[serde(rename = "web_search_preview")]
	WebSearchPreview,
	#[serde(rename = "web_search_preview_2025_03_11")]
	WebSearchPreview20250311,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
pub enum WebSearchToolUserLocationType {
	Approximate,
}
