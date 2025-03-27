use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddUploadPartRequest {
	/// The chunk of bytes for this Part.
	pub data: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminApiKey {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_at: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner: Option<AdminApiKeyOwner>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redacted_value: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminApiKeyOwner {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_at: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Annotation {
	FileCitation(FileCitation),
	UrlCitation(UrlCitation),
	FilePath(FilePath),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApiKeyList {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<AdminApiKey>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_more: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
}
/// Represents an event emitted when streaming a Run.
/// 
/// Each event in a server-sent events stream has an `event` and `data` property:
/// 
/// ```
/// event: thread.created
/// data: {"id": "thread_123", "object": "thread", ...}
/// ```
/// 
/// We emit events whenever a new object is created, transitions to a new state, or is being
/// streamed in parts (deltas). For example, we emit `thread.run.created` when a new run
/// is created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses
/// to create a message during a run, we emit a `thread.message.created event`, a
/// `thread.message.in_progress` event, many `thread.message.delta` events, and finally a
/// `thread.message.completed` event.
/// 
/// We may add additional events over time, so we recommend handling unknown events gracefully
/// in your code. See the [Assistants API quickstart](https://platform.openai.com/docs/assistants/overview) to learn how to
/// integrate the Assistants API with streaming.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantStreamEvent {
	ThreadStreamEvent(ThreadStreamEvent),
	RunStreamEvent(RunStreamEvent),
	RunStepStreamEvent(RunStepStreamEvent),
	MessageStreamEvent(MessageStreamEvent),
	ErrorEvent(ErrorEvent),
	DoneEvent(DoneEvent),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
/// The type of tool being defined: `code_interpreter`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantToolsCodeType {
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsFileSearch {
	/// Overrides for the file search tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<AssistantToolsFileSearchFileSearch>,
	/// The type of tool being defined: `file_search`
	pub r#type: AssistantToolsFileSearchType,
}
/// Overrides for the file search tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
#[serde(untagged)]
pub enum AssistantToolsFileSearchType {
	#[serde(rename = "file_search")]
	FileSearch,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsFileSearchTypeOnly {
	/// The type of tool being defined: `file_search`
	pub r#type: AssistantToolsFileSearchTypeOnlyType,
}
/// The type of tool being defined: `file_search`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantToolsFileSearchTypeOnlyType {
	#[serde(rename = "file_search")]
	FileSearch,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantToolsFunction {
	pub function: FunctionObject,
	/// The type of tool being defined: `function`
	pub r#type: AssistantToolsFunctionType,
}
/// The type of tool being defined: `function`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantToolsFunctionType {
	Function,
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
/// Controls which (if any) tool is called by the model.
/// `none` means the model will not call any tools and instead generates a message.
/// `auto` is the default value and means the model can pick between generating a message or calling one or more tools.
/// `required` means the model must call one or more tools before responding to the user.
/// Specifying a particular tool like `{"type": "file_search"}` or `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
	None(String),
	Auto(String),
	Required(String),
	AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}
/// Specifies a tool the model should use. Use to force the model to call a specific tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantsNamedToolChoice {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<AssistantsNamedToolChoiceFunction>,
	/// The type of the tool. If type is `function`, the function name must be set
	pub r#type: AssistantsNamedToolChoiceType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantsNamedToolChoiceFunction {
	/// The name of the function to call.
	pub name: String,
}
/// The type of the tool. If type is `function`, the function name must be set
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AssistantsNamedToolChoiceType {
	Function,
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
	#[serde(rename = "file_search")]
	FileSearch,
}
/// The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is `json`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AudioResponseFormat {
	Json,
	Text,
	Srt,
	#[serde(rename = "verbose_json")]
	VerboseJson,
	Vtt,
}
/// The actor who performed the audit logged action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogActor {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key: Option<AuditLogActorApiKey>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session: Option<AuditLogActorSession>,
	/// The type of actor. Is either `session` or `api_key`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<AuditLogActorType>,
}
/// The API Key used to perform the audit logged action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogActorApiKey {
	/// The tracking id of the API key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account: Option<AuditLogActorServiceAccount>,
	/// The type of API key. Can be either `user` or `service_account`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<AuditLogActorApiKeyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<AuditLogActorUser>,
}
/// The type of API key. Can be either `user` or `service_account`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AuditLogActorApiKeyType {
	User,
	#[serde(rename = "service_account")]
	ServiceAccount,
}
/// The service account that performed the audit logged action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogActorServiceAccount {
	/// The service account id.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}
/// The session in which the audit logged action was performed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogActorSession {
	/// The IP address from which the action was performed.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ip_address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<AuditLogActorUser>,
}
/// The type of actor. Is either `session` or `api_key`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AuditLogActorType {
	Session,
	#[serde(rename = "api_key")]
	ApiKey,
}
/// The user who performed the audit logged action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogActorUser {
	/// The user email.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	/// The user id.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}
/// The event type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AuditLogEventType {
	#[serde(rename = "api_key.created")]
	ApiKeyCreated,
	#[serde(rename = "api_key.updated")]
	ApiKeyUpdated,
	#[serde(rename = "api_key.deleted")]
	ApiKeyDeleted,
	#[serde(rename = "invite.sent")]
	InviteSent,
	#[serde(rename = "invite.accepted")]
	InviteAccepted,
	#[serde(rename = "invite.deleted")]
	InviteDeleted,
	#[serde(rename = "login.succeeded")]
	LoginSucceeded,
	#[serde(rename = "login.failed")]
	LoginFailed,
	#[serde(rename = "logout.succeeded")]
	LogoutSucceeded,
	#[serde(rename = "logout.failed")]
	LogoutFailed,
	#[serde(rename = "organization.updated")]
	OrganizationUpdated,
	#[serde(rename = "project.created")]
	ProjectCreated,
	#[serde(rename = "project.updated")]
	ProjectUpdated,
	#[serde(rename = "project.archived")]
	ProjectArchived,
	#[serde(rename = "service_account.created")]
	ServiceAccountCreated,
	#[serde(rename = "service_account.updated")]
	ServiceAccountUpdated,
	#[serde(rename = "service_account.deleted")]
	ServiceAccountDeleted,
	#[serde(rename = "rate_limit.updated")]
	RateLimitUpdated,
	#[serde(rename = "rate_limit.deleted")]
	RateLimitDeleted,
	#[serde(rename = "user.added")]
	UserAdded,
	#[serde(rename = "user.updated")]
	UserUpdated,
	#[serde(rename = "user.deleted")]
	UserDeleted,
}
/// The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AutoChunkingStrategyRequestParam {
	/// Always `auto`.
	pub r#type: AutoChunkingStrategyRequestParamType,
}
/// Always `auto`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AutoChunkingStrategyRequestParamType {
	Auto,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CancelUploadRequest {
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionDeleted {
	/// Whether the chat completion was deleted.
	pub deleted: bool,
	/// The ID of the chat completion that was deleted.
	pub id: String,
	/// The type of object being deleted.
	pub object: ChatCompletionDeletedObject,
}
/// The type of object being deleted.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionDeletedObject {
	#[serde(rename = "chat.completion.deleted")]
	ChatCompletionDeleted,
}
/// Specifying a particular function via `{"name": "my_function"}` forces the model to call that function.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionFunctionCallOption {
	/// The name of the function to call.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionFunctions {
	/// A description of what the function does, used by the model to choose when and how to call the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<FunctionParameters>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCall {
	/// The function that the model called.
	pub function: ChatCompletionMessageToolCallFunction,
	/// The ID of the tool call.
	pub id: String,
	/// The type of the tool. Currently, only `function` is supported.
	pub r#type: ChatCompletionMessageToolCallType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCallChunk {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<ChatCompletionMessageToolCallChunkFunction>,
	/// The ID of the tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	pub index: i64,
	/// The type of the tool. Currently, only `function` is supported.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ChatCompletionMessageToolCallChunkType>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCallChunkFunction {
	/// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/// The name of the function to call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
/// The type of the tool. Currently, only `function` is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionMessageToolCallChunkType {
	Function,
}
/// The function that the model called.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCallFunction {
	/// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
	pub arguments: String,
	/// The name of the function to call.
	pub name: String,
}
/// The type of the tool. Currently, only `function` is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionMessageToolCallType {
	Function,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionModalitiesChatCompletionModalities {
	Text,
	Audio,
}
/// Specifies a tool the model should use. Use to force the model to call a specific function.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionNamedToolChoice {
	pub function: ChatCompletionNamedToolChoiceFunction,
	/// The type of the tool. Currently, only `function` is supported.
	pub r#type: ChatCompletionNamedToolChoiceType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionNamedToolChoiceFunction {
	/// The name of the function to call.
	pub name: String,
}
/// The type of the tool. Currently, only `function` is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionNamedToolChoiceType {
	Function,
}
/// Messages sent by the model in response to user messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestAssistantMessage {
	/// Data about a previous audio response from the model. 
	/// [Learn more](https://platform.openai.com/docs/guides/audio).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<ChatCompletionRequestAssistantMessageAudio>,
	/// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<ChatCompletionRequestAssistantMessageContent>,
	/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionRequestAssistantMessageFunctionCall>,
	/// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The refusal message by the assistant.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	/// The role of the messages author, in this case `assistant`.
	pub role: ChatCompletionRequestAssistantMessageRole,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<ChatCompletionMessageToolCalls>,
}
/// Data about a previous audio response from the model. 
/// [Learn more](https://platform.openai.com/docs/guides/audio).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageAudio {
	/// Unique identifier for a previous audio response from the model.
	pub id: String,
}
/// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
	String(String),
	Array(ChatCompletionRequestAssistantMessageContentArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
	ChatCompletionRequestMessageContentPartRefusal(ChatCompletionRequestMessageContentPartRefusal),
}
/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageFunctionCall {
	/// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
	pub arguments: String,
	/// The name of the function to call.
	pub name: String,
}
/// The role of the messages author, in this case `assistant`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageRole {
	Assistant,
}
/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user. With o1 models and newer, `developer` messages
/// replace the previous `system` messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestDeveloperMessage {
	/// The contents of the developer message.
	pub content: ChatCompletionRequestDeveloperMessageContent,
	/// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The role of the messages author, in this case `developer`.
	pub role: ChatCompletionRequestDeveloperMessageRole,
}
/// The contents of the developer message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
	String(String),
	Array(ChatCompletionRequestDeveloperMessageContentArray),
}
/// The role of the messages author, in this case `developer`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageRole {
	Developer,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestFunctionMessage {
	/// The contents of the function message.
	pub content: String,
	/// The name of the function to call.
	pub name: String,
	/// The role of the messages author, in this case `function`.
	pub role: ChatCompletionRequestFunctionMessageRole,
}
/// The role of the messages author, in this case `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestFunctionMessageRole {
	Function,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessage {
	ChatCompletionRequestDeveloperMessage(ChatCompletionRequestDeveloperMessage),
	ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),
	ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),
	ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
	ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),
	ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}
/// Learn about [audio inputs](https://platform.openai.com/docs/guides/audio).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudio {
	pub input_audio: ChatCompletionRequestMessageContentPartAudioInputAudio,
	/// The type of the content part. Always `input_audio`.
	pub r#type: ChatCompletionRequestMessageContentPartAudioType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
	/// Base64 encoded audio data.
	pub data: String,
	/// The format of the encoded audio data. Currently supports "wav" and "mp3".
	pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}
/// The format of the encoded audio data. Currently supports "wav" and "mp3".
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
	Wav,
	Mp3,
}
/// The type of the content part. Always `input_audio`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartAudioType {
	#[serde(rename = "input_audio")]
	InputAudio,
}
/// Learn about [file inputs](https://platform.openai.com/docs/guides/text) for text generation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFile {
	pub file: ChatCompletionRequestMessageContentPartFileFile,
	/// The type of the content part. Always `file`.
	pub r#type: ChatCompletionRequestMessageContentPartFileType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFileFile {
	/// The base64 encoded file data, used when passing the file to the model 
	/// as a string.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_data: Option<String>,
	/// The ID of an uploaded file to use as input.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The name of the file, used when passing the file to the model as a 
	/// string.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
}
/// The type of the content part. Always `file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartFileType {
	File,
}
/// Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartImage {
	pub image_url: ChatCompletionRequestMessageContentPartImageImageUrl,
	/// The type of the content part.
	pub r#type: ChatCompletionRequestMessageContentPartImageType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
	/// Specifies the detail level of the image. Learn more in the [Vision guide](https://platform.openai.com/docs/guides/vision#low-or-high-fidelity-image-understanding).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<ChatCompletionRequestMessageContentPartImageImageUrlDetail>,
	/// Either a URL of the image or the base64 encoded image data.
	pub url: String,
}
/// Specifies the detail level of the image. Learn more in the [Vision guide](https://platform.openai.com/docs/guides/vision#low-or-high-fidelity-image-understanding).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartImageImageUrlDetail {
	Auto,
	Low,
	High,
}
/// The type of the content part.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartImageType {
	#[serde(rename = "image_url")]
	ImageUrl,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
	/// The refusal message generated by the model.
	pub refusal: String,
	/// The type of the content part.
	pub r#type: ChatCompletionRequestMessageContentPartRefusalType,
}
/// The type of the content part.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartRefusalType {
	Refusal,
}
/// Learn about [text inputs](https://platform.openai.com/docs/guides/text-generation).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartText {
	/// The text content.
	pub text: String,
	/// The type of the content part.
	pub r#type: ChatCompletionRequestMessageContentPartTextType,
}
/// The type of the content part.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPartTextType {
	Text,
}
/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user. With o1 models and newer, use `developer` messages
/// for this purpose instead.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestSystemMessage {
	/// The contents of the system message.
	pub content: ChatCompletionRequestSystemMessageContent,
	/// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The role of the messages author, in this case `system`.
	pub role: ChatCompletionRequestSystemMessageRole,
}
/// The contents of the system message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
	String(String),
	Array(ChatCompletionRequestSystemMessageContentArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}
/// The role of the messages author, in this case `system`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageRole {
	System,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestToolMessage {
	/// The contents of the tool message.
	pub content: ChatCompletionRequestToolMessageContent,
	/// The role of the messages author, in this case `tool`.
	pub role: ChatCompletionRequestToolMessageRole,
	/// Tool call that this message is responding to.
	pub tool_call_id: String,
}
/// The contents of the tool message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
	String(String),
	Array(ChatCompletionRequestToolMessageContentArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}
/// The role of the messages author, in this case `tool`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageRole {
	Tool,
}
/// Messages sent by an end user, containing prompts or additional context
/// information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionRequestUserMessage {
	/// The contents of the user message.
	pub content: ChatCompletionRequestUserMessageContent,
	/// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The role of the messages author, in this case `user`.
	pub role: ChatCompletionRequestUserMessageRole,
}
/// The contents of the user message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
	String(String),
	Array(ChatCompletionRequestUserMessageContentArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
	ChatCompletionRequestMessageContentPartImage(ChatCompletionRequestMessageContentPartImage),
	ChatCompletionRequestMessageContentPartAudio(ChatCompletionRequestMessageContentPartAudio),
	ChatCompletionRequestMessageContentPartFile(ChatCompletionRequestMessageContentPartFile),
}
/// The role of the messages author, in this case `user`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageRole {
	User,
}
/// A chat completion message generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessage {
	/// Annotations for the message, when applicable, as when using the
	/// [web search tool](https://platform.openai.com/docs/guides/tools-web-search?api-mode=chat).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<ChatCompletionResponseMessageAnnotations>>,
	/// If the audio output modality is requested, this object contains data
	/// about the audio response from the model. [Learn more](https://platform.openai.com/docs/guides/audio).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<ChatCompletionResponseMessageAudio>,
	/// The contents of the message.
	pub content: String,
	/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionResponseMessageFunctionCall>,
	/// The refusal message generated by the model.
	pub refusal: String,
	/// The role of the author of this message.
	pub role: ChatCompletionResponseMessageRole,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<ChatCompletionMessageToolCalls>,
}
/// A URL citation when using web search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessageAnnotations {
	/// The type of the URL citation. Always `url_citation`.
	pub r#type: ChatCompletionResponseMessageAnnotationsType,
	/// A URL citation when using web search.
	pub url_citation: ChatCompletionResponseMessageAnnotationsUrlCitation,
}
/// The type of the URL citation. Always `url_citation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionResponseMessageAnnotationsType {
	#[serde(rename = "url_citation")]
	UrlCitation,
}
/// A URL citation when using web search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessageAnnotationsUrlCitation {
	/// The index of the last character of the URL citation in the message.
	pub end_index: i64,
	/// The index of the first character of the URL citation in the message.
	pub start_index: i64,
	/// The title of the web resource.
	pub title: String,
	/// The URL of the web resource.
	pub url: String,
}
/// If the audio output modality is requested, this object contains data
/// about the audio response from the model. [Learn more](https://platform.openai.com/docs/guides/audio).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessageAudio {
	/// Base64 encoded audio bytes generated by the model, in the format
	/// specified in the request.
	pub data: String,
	/// The Unix timestamp (in seconds) for when this audio response will
	/// no longer be accessible on the server for use in multi-turn
	/// conversations.
	pub expires_at: i64,
	/// Unique identifier for this audio response.
	pub id: String,
	/// Transcript of the audio generated by the model.
	pub transcript: String,
}
/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessageFunctionCall {
	/// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
	pub arguments: String,
	/// The name of the function to call.
	pub name: String,
}
/// The role of the author of this message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionResponseMessageRole {
	Assistant,
}
/// The role of the author of a message
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionRole {
	Developer,
	System,
	User,
	Assistant,
	Tool,
	Function,
}
/// Options for streaming response. Only set this when you set `stream: true`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionStreamOptions {
	/// If set, an additional chunk will be streamed before the `data: [DONE]`
	/// message. The `usage` field on this chunk shows the token usage statistics
	/// for the entire request, and the `choices` field will always be an empty
	/// array. 
	/// 
	/// All other chunks will also include a `usage` field, but with a null
	/// value. **NOTE:** If the stream is interrupted, you may not receive the
	/// final usage chunk which contains the total token usage for the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_usage: Option<bool>,
}
/// A chat completion delta generated by streamed model responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionStreamResponseDelta {
	/// The contents of the chunk message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionStreamResponseDeltaFunctionCall>,
	/// The refusal message generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	/// The role of the author of this message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<ChatCompletionStreamResponseDeltaRole>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
}
/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionStreamResponseDeltaFunctionCall {
	/// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/// The name of the function to call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
/// The role of the author of this message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionStreamResponseDeltaRole {
	Developer,
	System,
	User,
	Assistant,
	Tool,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionTokenLogprob {
	/// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
	pub bytes: Vec<i64>,
	/// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
	pub logprob: f64,
	/// The token.
	pub token: String,
	/// List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.
	pub top_logprobs: Vec<ChatCompletionTokenLogprobTopLogprobs>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionTokenLogprobTopLogprobs {
	/// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
	pub bytes: Vec<i64>,
	/// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
	pub logprob: f64,
	/// The token.
	pub token: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionTool {
	pub function: FunctionObject,
	/// The type of the tool. Currently, only `function` is supported.
	pub r#type: ChatCompletionToolType,
}
/// Controls which (if any) tool is called by the model.
/// `none` means the model will not call any tool and instead generates a message.
/// `auto` means the model can pick between generating a message or calling one or more tools.
/// `required` means the model must call one or more tools.
/// Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
/// 
/// `none` is the default when no tools are present. `auto` is the default if tools are present.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
	None(String),
	Auto(String),
	Required(String),
	ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}
/// The type of the tool. Currently, only `function` is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChatCompletionToolType {
	Function,
}
/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChunkingStrategyRequestParam {
	AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
	StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum ClickType {
	Click,
}
/// The output of a code interpreter tool call that is a file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterFileOutput {
	pub files: Vec<CodeInterpreterFileOutputFiles>,
	/// The type of the code interpreter file output. Always `files`.
	pub r#type: CodeInterpreterFileOutputType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterFileOutputFiles {
	/// The ID of the file.
	pub file_id: String,
	/// The MIME type of the file.
	pub mime_type: String,
}
/// The type of the code interpreter file output. Always `files`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterFileOutputType {
	Files,
}
/// The output of a code interpreter tool call that is text.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterTextOutput {
	/// The logs of the code interpreter tool call.
	pub logs: String,
	/// The type of the code interpreter text output. Always `logs`.
	pub r#type: CodeInterpreterTextOutputType,
}
/// The type of the code interpreter text output. Always `logs`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterTextOutputType {
	Logs,
}
/// A tool that runs code.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterTool {
	/// The IDs of the files to run the code on.
	pub file_ids: Vec<String>,
	/// The type of the code interpreter tool. Always `code_interpreter`.
	pub r#type: CodeInterpreterToolType,
}
/// A tool call to run code.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterToolCall {
	/// The code to run.
	pub code: String,
	/// The unique ID of the code interpreter tool call.
	pub id: String,
	/// The results of the code interpreter tool call.
	pub results: Vec<CodeInterpreterToolOutput>,
	/// The status of the code interpreter tool call.
	pub status: CodeInterpreterToolCallStatus,
	/// The type of the code interpreter tool call. Always `code_interpreter_call`.
	pub r#type: CodeInterpreterToolCallType,
}
/// The status of the code interpreter tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Interpreting,
	Completed,
}
/// The type of the code interpreter tool call. Always `code_interpreter_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterToolCallType {
	#[serde(rename = "code_interpreter_call")]
	CodeInterpreterCall,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterToolOutput {
	CodeInterpreterTextOutput(CodeInterpreterTextOutput),
	CodeInterpreterFileOutput(CodeInterpreterFileOutput),
}
/// The type of the code interpreter tool. Always `code_interpreter`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CodeInterpreterToolType {
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
}
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompleteUploadRequest {
	/// The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub md5: Option<String>,
	/// The ordered list of Part IDs.
	pub part_ids: Vec<String>,
}
/// Usage statistics for the completion request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompletionUsage {
	/// Number of tokens in the generated completion.
	pub completion_tokens: i64,
	/// Breakdown of tokens used in a completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completion_tokens_details: Option<CompletionUsageCompletionTokensDetails>,
	/// Number of tokens in the prompt.
	pub prompt_tokens: i64,
	/// Breakdown of tokens used in the prompt.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt_tokens_details: Option<CompletionUsagePromptTokensDetails>,
	/// Total number of tokens used in the request (prompt + completion).
	pub total_tokens: i64,
}
/// Breakdown of tokens used in a completion.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompletionUsageCompletionTokensDetails {
	/// When using Predicted Outputs, the number of tokens in the
	/// prediction that appeared in the completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accepted_prediction_tokens: Option<i64>,
	/// Audio input tokens generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<i64>,
	/// Tokens generated by the model for reasoning.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_tokens: Option<i64>,
	/// When using Predicted Outputs, the number of tokens in the
	/// prediction that did not appear in the completion. However, like
	/// reasoning tokens, these tokens are still counted in the total
	/// completion tokens for purposes of billing, output, and context window
	/// limits.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rejected_prediction_tokens: Option<i64>,
}
/// Breakdown of tokens used in the prompt.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompletionUsagePromptTokensDetails {
	/// Audio input tokens present in the prompt.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<i64>,
	/// Cached tokens present in the prompt.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cached_tokens: Option<i64>,
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
#[serde(untagged)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerToolCallOutputResource {
	/// The safety checks reported by the API that have been acknowledged by the 
	/// developer.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,
	/// The ID of the computer tool call that produced the output.
	pub call_id: String,
	/// The unique ID of the computer call tool output.
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
#[serde(untagged)]
pub enum ComputerToolCallOutputStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the computer tool call output. Always `computer_call_output`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum ComputerToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the computer call. Always `computer_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ComputerToolCallType {
	#[serde(rename = "computer_call")]
	ComputerCall,
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
/// Multi-modal input and output contents.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Content {
	InputContent(InputContent),
	OutputContent(OutputContent),
}
/// An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Coordinate {
	/// The x-coordinate.
	pub x: i64,
	/// The y-coordinate.
	pub y: i64,
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
	pub tools: Option<Vec<CreateAssistantRequestTools>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
}
/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateAssistantRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
	/// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateAssistantRequestToolResourcesFileSearch {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CreateAssistantRequestToolResourcesFileSearch: Option<CreateAssistantRequestToolResourcesFileSearchCreateAssistantRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateAssistantRequestToolResourcesFileSearchCreateAssistantRequestToolResourcesFileSearch {
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateAssistantRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateChatCompletionRequest {
	/// Parameters for audio output. Required when audio output is requested with
	/// `modalities: ["audio"]`. [Learn more](https://platform.openai.com/docs/guides/audio).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<CreateChatCompletionRequestAudio>,
	/// Number between -2.0 and 2.0. Positive values penalize new tokens based on
	/// their existing frequency in the text so far, decreasing the model's
	/// likelihood to repeat the same line verbatim.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub frequency_penalty: Option<f64>,
	/// Deprecated in favor of `tool_choice`.
	/// 
	/// Controls which (if any) function is called by the model.
	/// 
	/// `none` means the model will not call a function and instead generates a
	/// message.
	/// 
	/// `auto` means the model can pick between generating a message or calling a
	/// function.
	/// 
	/// Specifying a particular function via `{"name": "my_function"}` forces the
	/// model to call that function.
	/// 
	/// `none` is the default when no functions are present. `auto` is the default
	/// if functions are present.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<CreateChatCompletionRequestFunctionCall>,
	/// Deprecated in favor of `tools`.
	/// 
	/// A list of functions the model may generate JSON inputs for.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub functions: Option<Vec<ChatCompletionFunctions>>,
	/// Modify the likelihood of specified tokens appearing in the completion.
	/// 
	/// Accepts a JSON object that maps tokens (specified by their token ID in the
	/// tokenizer) to an associated bias value from -100 to 100. Mathematically,
	/// the bias is added to the logits generated by the model prior to sampling.
	/// The exact effect will vary per model, but values between -1 and 1 should
	/// decrease or increase likelihood of selection; values like -100 or 100
	/// should result in a ban or exclusive selection of the relevant token.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logit_bias: Option<serde_json::Value>,
	/// Whether to return log probabilities of the output tokens or not. If true,
	/// returns the log probabilities of each output token returned in the
	/// `content` of `message`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<bool>,
	/// An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<i64>,
	/// The maximum number of [tokens](/tokenizer) that can be generated in the
	/// chat completion. This value can be used to control
	/// [costs](https://openai.com/api/pricing/) for text generated via API.
	/// 
	/// This value is now deprecated in favor of `max_completion_tokens`, and is
	/// not compatible with [o1 series models](https://platform.openai.com/docs/guides/reasoning).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens: Option<i64>,
	/// A list of messages comprising the conversation so far. Depending on the
	/// [model](https://platform.openai.com/docs/models) you use, different message types (modalities) are
	/// supported, like [text](https://platform.openai.com/docs/guides/text-generation),
	/// [images](https://platform.openai.com/docs/guides/vision), and [audio](https://platform.openai.com/docs/guides/audio).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub messages: Option<Vec<ChatCompletionRequestMessage>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<ResponseModalities>,
	/// Model ID used to generate the response, like `gpt-4o` or `o1`. OpenAI
	/// offers a wide range of models with different capabilities, performance
	/// characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models)
	/// to browse and compare available models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModelIdsShared>,
	/// How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	/// Configuration for a [Predicted Output](https://platform.openai.com/docs/guides/predicted-outputs),
	/// which can greatly improve response times when large parts of the model
	/// response are known ahead of time. This is most common when you are
	/// regenerating a file with only minor changes to most of the content.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prediction: Option<CreateChatCompletionRequestPrediction>,
	/// Number between -2.0 and 2.0. Positive values penalize new tokens based on
	/// whether they appear in the text so far, increasing the model's likelihood
	/// to talk about new topics.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub presence_penalty: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/// An object specifying the format that the model must output.
	/// 
	/// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
	/// Structured Outputs which ensures the model will match your supplied JSON
	/// schema. Learn more in the [Structured Outputs
	/// guide](https://platform.openai.com/docs/guides/structured-outputs).
	/// 
	/// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
	/// ensures the message the model generates is valid JSON. Using `json_schema`
	/// is preferred for models that support it.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateChatCompletionRequestResponseFormat>,
	/// This feature is in Beta.
	/// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.
	/// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<i64>,
	/// Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:
	///   - If set to 'auto', and the Project is Scale tier enabled, the system
	///     will utilize scale tier credits until they are exhausted.
	///   - If set to 'auto', and the Project is not Scale tier enabled, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
	///   - If set to 'default', the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
	///   - When not set, the default behavior is 'auto'.
	/// 
	///   When this parameter is set, the response body will include the `service_tier` utilized.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<CreateChatCompletionRequestServiceTier>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop: Option<StopConfiguration>,
	/// Whether or not to store the output of this chat completion request for 
	/// use in our [model distillation](https://platform.openai.com/docs/guides/distillation) or
	/// [evals](https://platform.openai.com/docs/guides/evals) products.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store: Option<bool>,
	/// If set to true, the model response data will be streamed to the client
	/// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
	/// See the [Streaming section below](https://platform.openai.com/docs/api-reference/chat/streaming)
	/// for more information, along with the [streaming responses](https://platform.openai.com/docs/guides/streaming-responses)
	/// guide for more information on how to handle the streaming events.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_options: Option<ChatCompletionStreamOptions>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	/// We generally recommend altering this or `top_p` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<ChatCompletionToolChoiceOption>,
	/// A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
	/// An integer between 0 and 20 specifying the number of most likely tokens to
	/// return at each token position, each with an associated log probability.
	/// `logprobs` must be set to `true` if this parameter is used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<i64>,
	/// An alternative to sampling with temperature, called nucleus sampling,
	/// where the model considers the results of the tokens with top_p probability
	/// mass. So 0.1 means only the tokens comprising the top 10% probability mass
	/// are considered.
	/// 
	/// We generally recommend altering this or `temperature` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
	/// This tool searches the web for relevant results to use in a response.
	/// Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search?api-mode=chat).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web_search_options: Option<CreateChatCompletionRequestWebSearchOptions>,
}
/// Parameters for audio output. Required when audio output is requested with
/// `modalities: ["audio"]`. [Learn more](https://platform.openai.com/docs/guides/audio).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateChatCompletionRequestAudio {
	/// Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,
	/// `opus`, or `pcm16`.
	pub format: CreateChatCompletionRequestAudioFormat,
	/// The voice the model uses to respond. Supported voices are 
	/// `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, and `shimmer`.
	pub voice: VoiceIdsShared,
}
/// Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,
/// `opus`, or `pcm16`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestAudioFormat {
	Wav,
	Mp3,
	Flac,
	Opus,
	Pcm16,
}
/// Deprecated in favor of `tool_choice`.
/// 
/// Controls which (if any) function is called by the model.
/// 
/// `none` means the model will not call a function and instead generates a
/// message.
/// 
/// `auto` means the model can pick between generating a message or calling a
/// function.
/// 
/// Specifying a particular function via `{"name": "my_function"}` forces the
/// model to call that function.
/// 
/// `none` is the default when no functions are present. `auto` is the default
/// if functions are present.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestFunctionCall {
	None(String),
	Auto(String),
	ChatCompletionFunctionCallOption(ChatCompletionFunctionCallOption),
}
/// Configuration for a [Predicted Output](https://platform.openai.com/docs/guides/predicted-outputs),
/// which can greatly improve response times when large parts of the model
/// response are known ahead of time. This is most common when you are
/// regenerating a file with only minor changes to most of the content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestPrediction {
	PredictionContent(PredictionContent),
}
/// An object specifying the format that the model must output.
/// 
/// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
/// Structured Outputs which ensures the model will match your supplied JSON
/// schema. Learn more in the [Structured Outputs
/// guide](https://platform.openai.com/docs/guides/structured-outputs).
/// 
/// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
/// ensures the message the model generates is valid JSON. Using `json_schema`
/// is preferred for models that support it.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestResponseFormat {
	ResponseFormatText(ResponseFormatText),
	ResponseFormatJsonSchema(ResponseFormatJsonSchema),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
}
/// Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:
///   - If set to 'auto', and the Project is Scale tier enabled, the system
///     will utilize scale tier credits until they are exhausted.
///   - If set to 'auto', and the Project is not Scale tier enabled, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
///   - If set to 'default', the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
///   - When not set, the default behavior is 'auto'.
/// 
///   When this parameter is set, the response body will include the `service_tier` utilized.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestServiceTier {
	Auto,
	Default,
}
/// This tool searches the web for relevant results to use in a response.
/// Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search?api-mode=chat).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateChatCompletionRequestWebSearchOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_context_size: Option<WebSearchContextSize>,
	/// Approximate location parameters for the search.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_location: Option<CreateChatCompletionRequestWebSearchOptionsUserLocation>,
}
/// Approximate location parameters for the search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateChatCompletionRequestWebSearchOptionsUserLocation {
	pub approximate: WebSearchLocation,
	/// The type of location approximation. Always `approximate`.
	pub r#type: CreateChatCompletionRequestWebSearchOptionsUserLocationType,
}
/// The type of location approximation. Always `approximate`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateChatCompletionRequestWebSearchOptionsUserLocationType {
	Approximate,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateCompletionRequest {
	/// Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.
	/// 
	/// When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.
	/// 
	/// **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub best_of: Option<i64>,
	/// Echo back the prompt in addition to the completion
	#[serde(skip_serializing_if = "Option::is_none")]
	pub echo: Option<bool>,
	/// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
	/// 
	/// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/guides/text-generation)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub frequency_penalty: Option<f64>,
	/// Modify the likelihood of specified tokens appearing in the completion.
	/// 
	/// Accepts a JSON object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
	/// 
	/// As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logit_bias: Option<serde_json::Value>,
	/// Include the log probabilities on the `logprobs` most likely output tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.
	/// 
	/// The maximum value for `logprobs` is 5.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<i64>,
	/// The maximum number of [tokens](/tokenizer) that can be generated in the completion.
	/// 
	/// The token count of your prompt plus `max_tokens` cannot exceed the model's context length. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens: Option<i64>,
	/// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models) for descriptions of them.
	pub model: CreateCompletionRequestModel,
	/// How many completions to generate for each prompt.
	/// 
	/// **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i64>,
	/// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
	/// 
	/// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/guides/text-generation)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub presence_penalty: Option<f64>,
	/// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.
	/// 
	/// Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.
	pub prompt: CreateCompletionRequestPrompt,
	/// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.
	/// 
	/// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop: Option<StopConfiguration>,
	/// Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_options: Option<ChatCompletionStreamOptions>,
	/// The suffix that comes after a completion of inserted text.
	/// 
	/// This parameter is only supported for `gpt-3.5-turbo-instruct`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	/// 
	/// We generally recommend altering this or `top_p` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or `temperature` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.
/// 
/// Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateCompletionRequestPrompt {
	String(String),
	Array(CreateCompletionRequestPromptArray),
	Array(CreateCompletionRequestPromptArray),
	Array(CreateCompletionRequestPromptArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateEmbeddingRequest {
	/// The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<i64>,
	/// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoding_format: Option<CreateEmbeddingRequestEncodingFormat>,
	/// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.
	pub input: CreateEmbeddingRequestInput,
	/// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models) for descriptions of them.
	pub model: CreateEmbeddingRequestModel,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateEmbeddingRequestEncodingFormat {
	Float,
	Base64,
}
/// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
	String(String),
	Array(CreateEmbeddingRequestInputArray),
	Array(CreateEmbeddingRequestInputArray),
	Array(CreateEmbeddingRequestInputArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateEmbeddingResponse {
	/// The list of embeddings generated by the model.
	pub data: Vec<Embedding>,
	/// The name of the model used to generate the embedding.
	pub model: String,
	/// The object type, which is always "list".
	pub object: CreateEmbeddingResponseObject,
	/// The usage information for the request.
	pub usage: CreateEmbeddingResponseUsage,
}
/// The object type, which is always "list".
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateEmbeddingResponseObject {
	List,
}
/// The usage information for the request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateEmbeddingResponseUsage {
	/// The number of tokens used by the prompt.
	pub prompt_tokens: i64,
	/// The total number of tokens used by the request.
	pub total_tokens: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFileRequest {
	/// The File object (not file name) to be uploaded.
	pub file: String,
	/// The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets
	pub purpose: CreateFileRequestPurpose,
}
/// The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFileRequestPurpose {
	Assistants,
	Batch,
	#[serde(rename = "fine-tune")]
	FineTune,
	Vision,
	#[serde(rename = "user_data")]
	UserData,
	Evals,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningCheckpointPermissionRequest {
	/// The project identifiers to grant access to.
	pub project_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequest {
	/// The hyperparameters used for the fine-tuning job.
	/// This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<CreateFineTuningJobRequestHyperparameters>,
	/// A list of integrations to enable for your fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integrations: Option<Vec<CreateFineTuningJobRequestIntegrations>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub method: Option<FineTuneMethod>,
	/// The name of the model to fine-tune. You can select one of the
	/// [supported models](https://platform.openai.com/docs/guides/fine-tuning#which-models-can-be-fine-tuned).
	pub model: CreateFineTuningJobRequestModel,
	/// The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.
	/// If a seed is not specified, one will be generated for you.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<i64>,
	/// A string of up to 64 characters that will be added to your fine-tuned model name.
	/// 
	/// For example, a `suffix` of "custom-model-name" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,
	/// The ID of an uploaded file that contains training data.
	/// 
	/// See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.
	/// 
	/// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
	/// 
	/// The contents of the file should differ depending on if the model uses the [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input), [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](https://platform.openai.com/docs/api-reference/fine-tuning/preference-input) format.
	/// 
	/// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
	pub training_file: String,
	/// The ID of an uploaded file that contains validation data.
	/// 
	/// If you provide this file, the data is used to generate validation
	/// metrics periodically during fine-tuning. These metrics can be viewed in
	/// the fine-tuning results file.
	/// The same data should not be present in both train and validation files.
	/// 
	/// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
	/// 
	/// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_file: Option<String>,
}
/// The hyperparameters used for the fine-tuning job.
/// This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequestHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters
	/// are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<CreateFineTuningJobRequestHyperparametersBatchSize>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
	/// overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<CreateFineTuningJobRequestHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle
	/// through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<CreateFineTuningJobRequestHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters
/// are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
/// overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle
/// through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrations {
	/// The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported.
	pub r#type: CreateFineTuningJobRequestIntegrationsType,
	/// The settings for your integration with Weights and Biases. This payload specifies the project that
	/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
	/// to your run, and set a default entity (team, username, etc) to be associated with your run.
	pub wandb: CreateFineTuningJobRequestIntegrationsWandb,
}
/// The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestIntegrationsType {
	Wandb(String),
}
/// The settings for your integration with Weights and Biases. This payload specifies the project that
/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
/// to your run, and set a default entity (team, username, etc) to be associated with your run.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrationsWandb {
	/// The entity to use for the run. This allows you to set the team or username of the WandB user that you would
	/// like associated with the run. If not set, the default entity for the registered WandB API key is used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/// A display name to set for the run. If not set, we will use the Job ID as the name.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The name of the project that the new run will be created under.
	pub project: String,
	/// A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
	/// default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateImageEditRequest {
	/// The image to edit. Must be a valid PNG file, less than 4MB, and square. If mask is not provided, image must have transparency, which will be used as the mask.
	pub image: String,
	/// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mask: Option<String>,
	/// The model to use for image generation. Only `dall-e-2` is supported at this time.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageEditRequestModel>,
	/// The number of images to generate. Must be between 1 and 10.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i64>,
	/// A text description of the desired image(s). The maximum length is 1000 characters.
	pub prompt: String,
	/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageEditRequestResponseFormat>,
	/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageEditRequestSize>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateImageEditRequestResponseFormat {
	Url,
	#[serde(rename = "b64_json")]
	B64Json,
}
/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateImageEditRequestSize {
	#[serde(rename = "256x256")]
	Type256x256,
	#[serde(rename = "512x512")]
	Type512x512,
	#[serde(rename = "1024x1024")]
	Type1024x1024,
}
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
#[serde(untagged)]
pub enum CreateImageRequestQuality {
	Standard,
	Hd,
}
/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateImageRequestResponseFormat {
	Url,
	#[serde(rename = "b64_json")]
	B64Json,
}
/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`. Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum CreateImageRequestStyle {
	Vivid,
	Natural,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateImageVariationRequest {
	/// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
	pub image: String,
	/// The model to use for image generation. Only `dall-e-2` is supported at this time.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageVariationRequestModel>,
	/// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i64>,
	/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageVariationRequestResponseFormat>,
	/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageVariationRequestSize>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateImageVariationRequestResponseFormat {
	Url,
	#[serde(rename = "b64_json")]
	B64Json,
}
/// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateImageVariationRequestSize {
	#[serde(rename = "256x256")]
	Type256x256,
	#[serde(rename = "512x512")]
	Type512x512,
	#[serde(rename = "1024x1024")]
	Type1024x1024,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMessageRequest {
	/// A list of files attached to the message, and the tools they should be added to.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attachments: Option<Vec<CreateMessageRequestAttachments>>,
	pub content: CreateMessageRequestContent,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The role of the entity that is creating the message. Allowed values include:
	/// - `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.
	/// - `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.
	pub role: CreateMessageRequestRole,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMessageRequestAttachments {
	/// The ID of the file to attach to the message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The tools to add this file to.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateMessageRequestAttachmentsTools>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateMessageRequestAttachmentsTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateMessageRequestContent {
	String(String),
	Array(CreateMessageRequestContentArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateMessageRequestContentCreateMessageRequestContent {
	MessageContentImageFileObject(MessageContentImageFileObject),
	MessageContentImageUrlObject(MessageContentImageUrlObject),
	MessageRequestContentTextObject(MessageRequestContentTextObject),
}
/// The role of the entity that is creating the message. Allowed values include:
/// - `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.
/// - `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateMessageRequestRole {
	User,
	Assistant,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateModelResponseProperties {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	/// We generally recommend altering this or `top_p` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// An alternative to sampling with temperature, called nucleus sampling,
	/// where the model considers the results of the tokens with top_p probability
	/// mass. So 0.1 means only the tokens comprising the top 10% probability mass
	/// are considered.
	/// 
	/// We generally recommend altering this or `temperature` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
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
	Array(CreateModerationRequestInputArray),
	Array(CreateModerationRequestInputArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateModerationRequestInputCreateModerationRequestInput {
	Object(serde_json::Value),
	Object(serde_json::Value),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
	Array(CreateResponseInputArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateRunRequest {
	/// Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_instructions: Option<String>,
	/// Adds additional messages to the thread before creating the run.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_messages: Option<Vec<CreateMessageRequest>>,
	/// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) to use to execute this run.
	pub assistant_id: String,
	/// Overrides the [instructions](https://platform.openai.com/docs/api-reference/assistants/createAssistant) of the assistant. This is useful for modifying the behavior on a per-run basis.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<i64>,
	/// The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_prompt_tokens: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The ID of the [Model](https://platform.openai.com/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateRunRequestModel>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
	/// If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<CreateRunRequestToolChoice>,
	/// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateRunRequestTools>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation_strategy: Option<CreateRunRequestTruncationStrategy>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateRunRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateRunRequestTruncationStrategy {
	/// The number of most recent messages from the thread when constructing the context for the run.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_messages: Option<i64>,
	/// The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`.
	pub r#type: TruncationObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateSpeechRequest {
	/// The text to generate audio for. The maximum length is 4096 characters.
	pub input: String,
	/// Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// One of the available [TTS models](https://platform.openai.com/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.
	pub model: CreateSpeechRequestModel,
	/// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateSpeechRequestResponseFormat>,
	/// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f64>,
	/// The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech#voice-options).
	pub voice: VoiceIdsShared,
}
/// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateSpeechRequestResponseFormat {
	Mp3,
	Opus,
	Aac,
	Flac,
	Wav,
	Pcm,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadAndRunRequest {
	/// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) to use to execute this run.
	pub assistant_id: String,
	/// Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<i64>,
	/// The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_prompt_tokens: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The ID of the [Model](https://platform.openai.com/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateThreadAndRunRequestModel>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
	/// If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thread: Option<CreateThreadRequest>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<CreateThreadAndRunRequestToolChoice>,
	/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateThreadAndRunRequestToolResources>,
	/// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateThreadAndRunRequestTools>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation_strategy: Option<CreateThreadAndRunRequestTruncationStrategy>,
}
/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadAndRunRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateThreadAndRunRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
	/// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
	/// The ID of the [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateThreadAndRunRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadAndRunRequestTruncationStrategy {
	/// The number of most recent messages from the thread when constructing the context for the run.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_messages: Option<i64>,
	/// The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`.
	pub r#type: TruncationObjectType,
}
/// Options to create a new thread. If no thread is provided when running a 
/// request, an empty thread will be created.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadRequest {
	/// A list of [messages](https://platform.openai.com/docs/api-reference/messages) to start the thread with.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub messages: Option<Vec<CreateMessageRequest>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateThreadRequestToolResources>,
}
/// A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateThreadRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateThreadRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadRequestToolResourcesCodeInterpreter {
	/// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateThreadRequestToolResourcesFileSearch {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CreateThreadRequestToolResourcesFileSearch: Option<CreateThreadRequestToolResourcesFileSearchCreateThreadRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateThreadRequestToolResourcesFileSearchCreateThreadRequestToolResourcesFileSearch {
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTranscriptionRequest {
	/// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
	pub file: String,
	/// Additional information to include in the transcription response. 
	/// `logprobs` will return the log probabilities of the tokens in the 
	/// response to understand the model's confidence in the transcription. 
	/// `logprobs` only works with response_format set to `json` and only with 
	/// the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<TranscriptionInclude>>,
	/// The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model).
	pub model: CreateTranscriptionRequestModel,
	/// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should match the audio language.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AudioResponseFormat>,
	/// If set to true, the model response data will be streamed to the client
	/// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). 
	/// See the [Streaming section of the Speech-to-Text guide](https://platform.openai.com/docs/guides/speech-to-text?lang=curl#streaming-transcriptions)
	/// for more information.
	/// 
	/// Note: Streaming is not supported for the `whisper-1` model and will be ignored.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp_granularities: Option<Vec<CreateTranscriptionRequestTimestampGranularities>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateTranscriptionRequestTimestampGranularities {
	Word,
	Segment,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTranslationRequest {
	/// The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
	pub file: String,
	/// ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.
	pub model: CreateTranslationRequestModel,
	/// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should be in English.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
	/// The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateTranslationRequestResponseFormat>,
	/// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
}
/// The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateTranslationRequestResponseFormat {
	Json,
	Text,
	Srt,
	#[serde(rename = "verbose_json")]
	VerboseJson,
	Vtt,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTranslationResponseJson {
	pub text: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTranslationResponseVerboseJson {
	/// The duration of the input audio.
	pub duration: f64,
	/// The language of the output translation (always `english`).
	pub language: String,
	/// Segments of the translated text and their corresponding details.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub segments: Option<Vec<TranscriptionSegment>>,
	/// The translated text.
	pub text: String,
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
#[serde(untagged)]
pub enum CreateUploadRequestPurpose {
	Assistants,
	Batch,
	#[serde(rename = "fine-tune")]
	FineTune,
	Vision,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateVectorStoreFileBatchRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
	/// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
	pub file_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateVectorStoreFileRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
	/// A [File](https://platform.openai.com/docs/api-reference/files) ID that the vector store should use. Useful for tools like `file_search` that can access files.
	pub file_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateVectorStoreRequest {
	/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<CreateVectorStoreRequestChunkingStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<VectorStoreExpirationAfter>,
	/// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The name of the vector store.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateVectorStoreRequestChunkingStrategy {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CreateVectorStoreRequestChunkingStrategy: Option<CreateVectorStoreRequestChunkingStrategyCreateVectorStoreRequestChunkingStrategy>,
}
/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateVectorStoreRequestChunkingStrategyCreateVectorStoreRequestChunkingStrategy {
	AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
	StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DefaultProjectErrorResponse {
	pub code: i64,
	pub message: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteAssistantResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteAssistantResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteAssistantResponseObject {
	#[serde(rename = "assistant.deleted")]
	AssistantDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteFileResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteFileResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteFileResponseObject {
	File,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteFineTuningCheckpointPermissionResponse {
	/// Whether the fine-tuned model checkpoint permission was successfully deleted.
	pub deleted: bool,
	/// The ID of the fine-tuned model checkpoint permission that was deleted.
	pub id: String,
	/// The object type, which is always "checkpoint.permission".
	pub object: DeleteFineTuningCheckpointPermissionResponseObject,
}
/// The object type, which is always "checkpoint.permission".
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteFineTuningCheckpointPermissionResponseObject {
	#[serde(rename = "checkpoint.permission")]
	CheckpointPermission,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteMessageResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteMessageResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteMessageResponseObject {
	#[serde(rename = "thread.message.deleted")]
	ThreadMessageDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteModelResponse {
	pub deleted: bool,
	pub id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteThreadResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteThreadResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteThreadResponseObject {
	#[serde(rename = "thread.deleted")]
	ThreadDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteVectorStoreFileResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteVectorStoreFileResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteVectorStoreFileResponseObject {
	#[serde(rename = "vector_store.file.deleted")]
	VectorStoreFileDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteVectorStoreResponse {
	pub deleted: bool,
	pub id: String,
	pub object: DeleteVectorStoreResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum DeleteVectorStoreResponseObject {
	#[serde(rename = "vector_store.deleted")]
	VectorStoreDeleted,
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
#[serde(untagged)]
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum EasyInputMessageRole {
	User,
	Assistant,
	System,
	Developer,
}
/// The type of the message input. Always `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum EasyInputMessageType {
	Message,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Error {
	pub code: String,
	pub message: String,
	pub param: String,
	pub r#type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorResponse {
	pub error: Error,
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum FilePathType {
	#[serde(rename = "file_path")]
	FilePath,
}
/// The ranker to use for the file search. If not specified will use the `auto` ranker.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
	pub results: Option<Vec<FileSearchToolCallResults>>,
	/// The status of the file search tool call. One of `in_progress`, 
	/// `searching`, `incomplete` or `failed`,
	pub status: FileSearchToolCallStatus,
	/// The type of the file search tool call. Always `file_search_call`.
	pub r#type: FileSearchToolCallType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchToolCallResults {
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
#[serde(untagged)]
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum FileSearchToolType {
	#[serde(rename = "file_search")]
	FileSearch,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneChatCompletionRequestAssistantMessage {
	/// Data about a previous audio response from the model. 
	/// [Learn more](https://platform.openai.com/docs/guides/audio).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<ChatCompletionRequestAssistantMessageAudio>,
	/// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<ChatCompletionRequestAssistantMessageContent>,
	/// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionRequestAssistantMessageFunctionCall>,
	/// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The refusal message by the assistant.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	/// The role of the messages author, in this case `assistant`.
	pub role: ChatCompletionRequestAssistantMessageRole,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<ChatCompletionMessageToolCalls>,
	/// Controls whether the assistant message is trained against (0 or 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<i64>,
}
/// Configuration for the DPO fine-tuning method.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneDPOMethod {
	/// The hyperparameters used for the fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneDPOMethodHyperparameters>,
}
/// The hyperparameters used for the fine-tuning job.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneDPOMethodHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneDPOMethodHyperparametersBatchSize>,
	/// The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub beta: Option<FineTuneDPOMethodHyperparametersBeta>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneDPOMethodHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneDPOMethodHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersBeta {
	Auto(String),
	Number(f64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
/// The method used for fine-tuning.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneMethod {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dpo: Option<FineTuneDPOMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supervised: Option<FineTuneSupervisedMethod>,
	/// The type of method. Is either `supervised` or `dpo`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<FineTuneMethodType>,
}
/// The type of method. Is either `supervised` or `dpo`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneMethodType {
	Supervised,
	Dpo,
}
/// Configuration for the supervised fine-tuning method.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneSupervisedMethod {
	/// The hyperparameters used for the fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneSupervisedMethodHyperparameters>,
}
/// The hyperparameters used for the fine-tuning job.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuneSupervisedMethodHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneSupervisedMethodHyperparametersBatchSize>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneSupervisedMethodHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneSupervisedMethodHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningIntegration {
	/// The type of the integration being enabled for the fine-tuning job
	pub r#type: FineTuningIntegrationType,
	/// The settings for your integration with Weights and Biases. This payload specifies the project that
	/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
	/// to your run, and set a default entity (team, username, etc) to be associated with your run.
	pub wandb: FineTuningIntegrationWandb,
}
/// The type of the integration being enabled for the fine-tuning job
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuningIntegrationType {
	Wandb,
}
/// The settings for your integration with Weights and Biases. This payload specifies the project that
/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
/// to your run, and set a default entity (team, username, etc) to be associated with your run.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningIntegrationWandb {
	/// The entity to use for the run. This allows you to set the team or username of the WandB user that you would
	/// like associated with the run. If not set, the default entity for the registered WandB API key is used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/// A display name to set for the run. If not set, we will use the Job ID as the name.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The name of the project that the new run will be created under.
	pub project: String,
	/// A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
	/// default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCallOutputResource {
	/// The unique ID of the function tool call generated by the model.
	pub call_id: String,
	/// The unique ID of the function call tool output.
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
#[serde(untagged)]
pub enum FunctionToolCallOutputStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the function tool call output. Always `function_call_output`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionToolCallOutputType {
	#[serde(rename = "function_call_output")]
	FunctionCallOutput,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCallResource {
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
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionToolCallStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the function tool call. Always `function_call`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionToolCallType {
	#[serde(rename = "function_call")]
	FunctionCall,
}
/// The type of the function tool. Always `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FunctionToolType {
	Function,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImagesResponse {
	pub created: i64,
	pub data: Vec<Image>,
}
/// Specify additional output data to include in the model response. Currently
/// supported values are:
/// - `file_search_call.results`: Include the search results of
///   the file search tool call.
/// - `message.input_image.image_url`: Include image urls from the input message.
/// - `computer_call_output.output.image_url`: Include image urls from the computer call output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Includable {
	#[serde(rename = "file_search_call.results")]
	FileSearchCallResults,
	#[serde(rename = "message.input_image.image_url")]
	MessageInputImageImageUrl,
	#[serde(rename = "computer_call_output.output.image_url")]
	ComputerCallOutputOutputImageUrl,
}
/// An audio input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputAudio {
	/// Base64-encoded audio data.
	pub data: String,
	/// The format of the audio data. Currently supported formats are `mp3` and
	/// `wav`.
	pub format: InputAudioFormat,
	/// The type of the input item. Always `input_audio`.
	pub r#type: InputAudioType,
}
/// The format of the audio data. Currently supported formats are `mp3` and
/// `wav`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InputAudioFormat {
	Mp3,
	Wav,
}
/// The type of the input item. Always `input_audio`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InputAudioType {
	#[serde(rename = "input_audio")]
	InputAudio,
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum InputImageDetail {
	High,
	Low,
	Auto,
}
/// The type of the input item. Always `input_image`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputMessageResource {
	pub content: InputMessageContentList,
	/// The unique ID of the message input.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
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
#[serde(untagged)]
pub enum InputMessageRole {
	User,
	System,
	Developer,
}
/// The status of item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InputMessageStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the message input. Always set to `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum InputTextType {
	#[serde(rename = "input_text")]
	InputText,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteDeleteResponse {
	pub deleted: bool,
	pub id: String,
	/// The object type, which is always `organization.invite.deleted`
	pub object: InviteDeleteResponseObject,
}
/// The object type, which is always `organization.invite.deleted`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InviteDeleteResponseObject {
	#[serde(rename = "organization.invite.deleted")]
	OrganizationInviteDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteListResponse {
	pub data: Vec<Invite>,
	/// The first `invite_id` in the retrieved `list`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	/// The `has_more` property is used for pagination to indicate there are additional results.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_more: Option<bool>,
	/// The last `invite_id` in the retrieved `list`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	/// The object type, which is always `list`
	pub object: InviteListResponseObject,
}
/// The object type, which is always `list`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InviteListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteRequest {
	/// Send an email to this address
	pub email: String,
	/// An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub projects: Option<Vec<InviteRequestProjects>>,
	/// `owner` or `reader`
	pub role: InviteRequestRole,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteRequestProjects {
	/// Project's public ID
	pub id: String,
	/// Project membership role
	pub role: InviteRequestProjectsRole,
}
/// Project membership role
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InviteRequestProjectsRole {
	Member,
	Owner,
}
/// `owner` or `reader`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum InviteRequestRole {
	Reader,
	Owner,
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
#[serde(untagged)]
pub enum ItemReferenceType {
	#[serde(rename = "item_reference")]
	ItemReference,
}
/// Content item used to generate a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ItemResource {
	InputMessageResource(InputMessageResource),
	OutputMessage(OutputMessage),
	FileSearchToolCall(FileSearchToolCall),
	ComputerToolCall(ComputerToolCall),
	ComputerToolCallOutputResource(ComputerToolCallOutputResource),
	WebSearchToolCall(WebSearchToolCall),
	FunctionToolCallResource(FunctionToolCallResource),
	FunctionToolCallOutputResource(FunctionToolCallOutputResource),
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
#[serde(untagged)]
pub enum KeyPressType {
	Keypress,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListAuditLogsResponse {
	pub data: Vec<AuditLog>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: ListAuditLogsResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListAuditLogsResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListBatchesResponse {
	pub data: Vec<Batch>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	pub has_more: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub object: ListBatchesResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListBatchesResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFilesResponse {
	pub data: Vec<OpenAIFile>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningCheckpointPermissionResponse {
	pub data: Vec<FineTuningCheckpointPermission>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	pub has_more: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub object: ListFineTuningCheckpointPermissionResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListFineTuningCheckpointPermissionResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningJobCheckpointsResponse {
	pub data: Vec<FineTuningJobCheckpoint>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	pub has_more: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub object: ListFineTuningJobCheckpointsResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListFineTuningJobCheckpointsResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningJobEventsResponse {
	pub data: Vec<FineTuningJobEvent>,
	pub has_more: bool,
	pub object: ListFineTuningJobEventsResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListFineTuningJobEventsResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListMessagesResponse {
	pub data: Vec<MessageObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListModelsResponse {
	pub data: Vec<Model>,
	pub object: ListModelsResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListModelsResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListPaginatedFineTuningJobsResponse {
	pub data: Vec<FineTuningJob>,
	pub has_more: bool,
	pub object: ListPaginatedFineTuningJobsResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ListPaginatedFineTuningJobsResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListRunStepsResponse {
	pub data: Vec<RunStepObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListRunsResponse {
	pub data: Vec<RunObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListThreadsResponse {
	pub data: Vec<ThreadObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListVectorStoreFilesResponse {
	pub data: Vec<VectorStoreFileObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListVectorStoresResponse {
	pub data: Vec<VectorStoreObject>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogProb {
	/// The bytes that were used to generate the log probability.
	pub bytes: Vec<i64>,
	/// The log probability of the token.
	pub logprob: f64,
	/// The token that was used to generate the log probability.
	pub token: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<Vec<LogProbProperties>>,
}
/// A log probability object.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogProbProperties {
	/// The bytes that were used to generate the log probability.
	pub bytes: Vec<i64>,
	/// The log probability of the token.
	pub logprob: f64,
	/// The token that was used to generate the log probability.
	pub token: String,
}
/// References an image [File](https://platform.openai.com/docs/api-reference/files) in the content of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentImageFileObject {
	pub image_file: MessageContentImageFileObjectImageFile,
	/// Always `image_file`.
	pub r#type: MessageContentImageFileObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentImageFileObjectImageFile {
	/// Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageContentImageFileObjectImageFileDetail>,
	/// The [File](https://platform.openai.com/docs/api-reference/files) ID of the image in the message content. Set `purpose="vision"` when uploading the File if you need to later display the file content.
	pub file_id: String,
}
/// Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentImageFileObjectImageFileDetail {
	Auto,
	Low,
	High,
}
/// Always `image_file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentImageFileObjectType {
	#[serde(rename = "image_file")]
	ImageFile,
}
/// References an image URL in the content of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentImageUrlObject {
	pub image_url: MessageContentImageUrlObjectImageUrl,
	/// The type of the content part.
	pub r#type: MessageContentImageUrlObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentImageUrlObjectImageUrl {
	/// Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageContentImageUrlObjectImageUrlDetail>,
	/// The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp.
	pub url: String,
}
/// Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentImageUrlObjectImageUrlDetail {
	Auto,
	Low,
	High,
}
/// The type of the content part.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentImageUrlObjectType {
	#[serde(rename = "image_url")]
	ImageUrl,
}
/// The refusal content generated by the assistant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentRefusalObject {
	pub refusal: String,
	/// Always `refusal`.
	pub r#type: MessageContentRefusalObjectType,
}
/// Always `refusal`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentRefusalObjectType {
	Refusal,
}
/// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextAnnotationsFileCitationObject {
	pub end_index: i64,
	pub file_citation: MessageContentTextAnnotationsFileCitationObjectFileCitation,
	pub start_index: i64,
	/// The text in the message content that needs to be replaced.
	pub text: String,
	/// Always `file_citation`.
	pub r#type: MessageContentTextAnnotationsFileCitationObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
	/// The ID of the specific File the citation is from.
	pub file_id: String,
}
/// Always `file_citation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentTextAnnotationsFileCitationObjectType {
	#[serde(rename = "file_citation")]
	FileCitation,
}
/// A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextAnnotationsFilePathObject {
	pub end_index: i64,
	pub file_path: MessageContentTextAnnotationsFilePathObjectFilePath,
	pub start_index: i64,
	/// The text in the message content that needs to be replaced.
	pub text: String,
	/// Always `file_path`.
	pub r#type: MessageContentTextAnnotationsFilePathObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
	/// The ID of the file that was generated.
	pub file_id: String,
}
/// Always `file_path`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentTextAnnotationsFilePathObjectType {
	#[serde(rename = "file_path")]
	FilePath,
}
/// The text content that is part of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextObject {
	pub text: MessageContentTextObjectText,
	/// Always `text`.
	pub r#type: MessageContentTextObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageContentTextObjectText {
	pub annotations: Vec<MessageContentTextObjectTextAnnotations>,
	/// The data that makes up the text.
	pub value: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentTextObjectTextAnnotations {
	MessageContentTextAnnotationsFileCitationObject(MessageContentTextAnnotationsFileCitationObject),
	MessageContentTextAnnotationsFilePathObject(MessageContentTextAnnotationsFilePathObject),
}
/// Always `text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageContentTextObjectType {
	Text,
}
/// References an image [File](https://platform.openai.com/docs/api-reference/files) in the content of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentImageFileObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_file: Option<MessageDeltaContentImageFileObjectImageFile>,
	/// The index of the content part in the message.
	pub index: i64,
	/// Always `image_file`.
	pub r#type: MessageDeltaContentImageFileObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentImageFileObjectImageFile {
	/// Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageDeltaContentImageFileObjectImageFileDetail>,
	/// The [File](https://platform.openai.com/docs/api-reference/files) ID of the image in the message content. Set `purpose="vision"` when uploading the File if you need to later display the file content.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}
/// Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentImageFileObjectImageFileDetail {
	Auto,
	Low,
	High,
}
/// Always `image_file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentImageFileObjectType {
	#[serde(rename = "image_file")]
	ImageFile,
}
/// References an image URL in the content of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentImageUrlObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<MessageDeltaContentImageUrlObjectImageUrl>,
	/// The index of the content part in the message.
	pub index: i64,
	/// Always `image_url`.
	pub r#type: MessageDeltaContentImageUrlObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentImageUrlObjectImageUrl {
	/// Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageDeltaContentImageUrlObjectImageUrlDetail>,
	/// The URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}
/// Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentImageUrlObjectImageUrlDetail {
	Auto,
	Low,
	High,
}
/// Always `image_url`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentImageUrlObjectType {
	#[serde(rename = "image_url")]
	ImageUrl,
}
/// The refusal content that is part of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentRefusalObject {
	/// The index of the refusal part in the message.
	pub index: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	/// Always `refusal`.
	pub r#type: MessageDeltaContentRefusalObjectType,
}
/// Always `refusal`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentRefusalObjectType {
	Refusal,
}
/// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_index: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_citation: Option<MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>,
	/// The index of the annotation in the text content part.
	pub index: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_index: Option<i64>,
	/// The text in the message content that needs to be replaced.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// Always `file_citation`.
	pub r#type: MessageDeltaContentTextAnnotationsFileCitationObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
	/// The ID of the specific File the citation is from.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/// The specific quote in the file.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quote: Option<String>,
}
/// Always `file_citation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentTextAnnotationsFileCitationObjectType {
	#[serde(rename = "file_citation")]
	FileCitation,
}
/// A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_index: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_path: Option<MessageDeltaContentTextAnnotationsFilePathObjectFilePath>,
	/// The index of the annotation in the text content part.
	pub index: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_index: Option<i64>,
	/// The text in the message content that needs to be replaced.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// Always `file_path`.
	pub r#type: MessageDeltaContentTextAnnotationsFilePathObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
	/// The ID of the file that was generated.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}
/// Always `file_path`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentTextAnnotationsFilePathObjectType {
	#[serde(rename = "file_path")]
	FilePath,
}
/// The text content that is part of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextObject {
	/// The index of the content part in the message.
	pub index: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<MessageDeltaContentTextObjectText>,
	/// Always `text`.
	pub r#type: MessageDeltaContentTextObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaContentTextObjectText {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<MessageDeltaContentTextObjectTextAnnotations>>,
	/// The data that makes up the text.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentTextObjectTextAnnotations {
	MessageDeltaContentTextAnnotationsFileCitationObject(MessageDeltaContentTextAnnotationsFileCitationObject),
	MessageDeltaContentTextAnnotationsFilePathObject(MessageDeltaContentTextAnnotationsFilePathObject),
}
/// Always `text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageDeltaContentTextObjectType {
	Text,
}
/// The text content that is part of a message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageRequestContentTextObject {
	/// Text content to be sent to the model
	pub text: String,
	/// Always `text`.
	pub r#type: MessageRequestContentTextObjectType,
}
/// Always `text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageRequestContentTextObjectType {
	Text,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum MessageStreamEvent {
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
}
/// Describes an OpenAI model offering that can be used with the API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Model {
	/// The Unix timestamp (in seconds) when the model was created.
	pub created: i64,
	/// The model identifier, which can be referenced in the API endpoints.
	pub id: String,
	/// The object type, which is always "model".
	pub object: ModelObject,
	/// The organization that owns the model.
	pub owned_by: String,
}
/// The object type, which is always "model".
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ModelObject {
	Model,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModelResponseProperties {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
	/// We generally recommend altering this or `top_p` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// An alternative to sampling with temperature, called nucleus sampling,
	/// where the model considers the results of the tokens with top_p probability
	/// mass. So 0.1 means only the tokens comprising the top 10% probability mass
	/// are considered.
	/// 
	/// We generally recommend altering this or `temperature` but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
	pub tools: Option<Vec<ModifyAssistantRequestTools>>,
	/// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
	/// 
	/// We generally recommend altering this or temperature but not both.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f64>,
}
/// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ModifyAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ModifyAssistantRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
	/// Overrides the list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyAssistantRequestToolResourcesFileSearch {
	/// Overrides the [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ModifyAssistantRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyMessageRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyRunRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyThreadRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<ModifyThreadRequestToolResources>,
}
/// A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyThreadRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ModifyThreadRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ModifyThreadRequestToolResourcesFileSearch>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
	/// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyThreadRequestToolResourcesFileSearch {
	/// The [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
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
#[serde(untagged)]
pub enum MoveType {
	Move,
}
/// The `File` object represents a document that has been uploaded to OpenAI.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OpenAIFile {
	/// The size of the file, in bytes.
	pub bytes: i64,
	/// The Unix timestamp (in seconds) for when the file was created.
	pub created_at: i64,
	/// The Unix timestamp (in seconds) for when the file will expire.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<i64>,
	/// The name of the file.
	pub filename: String,
	/// The file identifier, which can be referenced in the API endpoints.
	pub id: String,
	/// The object type, which is always `file`.
	pub object: OpenAIFileObject,
	/// The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`.
	pub purpose: OpenAIFilePurpose,
	/// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
	pub status: OpenAIFileStatus,
	/// Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_details: Option<String>,
}
/// The object type, which is always `file`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OpenAIFileObject {
	File,
}
/// The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OpenAIFilePurpose {
	Assistants,
	#[serde(rename = "assistants_output")]
	AssistantsOutput,
	Batch,
	#[serde(rename = "batch_output")]
	BatchOutput,
	#[serde(rename = "fine-tune")]
	FineTune,
	#[serde(rename = "fine-tune-results")]
	FineTuneResults,
	Vision,
}
/// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OpenAIFileStatus {
	Uploaded,
	Processed,
	Error,
}
/// This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OtherChunkingStrategyResponseParam {
	/// Always `other`.
	pub r#type: OtherChunkingStrategyResponseParamType,
}
/// Always `other`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OtherChunkingStrategyResponseParamType {
	Other,
}
/// An audio output from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputAudio {
	/// Base64-encoded audio data from the model.
	pub data: String,
	/// The transcript of the audio data from the model.
	pub transcript: String,
	/// The type of the output audio. Always `output_audio`.
	pub r#type: OutputAudioType,
}
/// The type of the output audio. Always `output_audio`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OutputAudioType {
	#[serde(rename = "output_audio")]
	OutputAudio,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OutputContent {
	OutputText(OutputText),
	Refusal(Refusal),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OutputItem {
	#[serde(rename = "OutputMessage(OutputMessage)")]
	OutputMessage(outputMessage),
	#[serde(rename = "FileSearchToolCall(FileSearchToolCall)")]
	FileSearchToolCall(fileSearchToolCall),
	#[serde(rename = "FunctionToolCall(FunctionToolCall)")]
	FunctionToolCall(functionToolCall),
	#[serde(rename = "WebSearchToolCall(WebSearchToolCall)")]
	WebSearchToolCall(webSearchToolCall),
	#[serde(rename = "ComputerToolCall(ComputerToolCall)")]
	ComputerToolCall(computerToolCall),
	#[serde(rename = "ReasoningItem(ReasoningItem)")]
	ReasoningItem(reasoningItem),
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
#[serde(untagged)]
pub enum OutputMessageRole {
	Assistant,
}
/// The status of the message input. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when input items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum OutputMessageStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
/// The type of the output message. Always `message`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum OutputTextType {
	#[serde(rename = "output_text")]
	OutputText,
}
/// Static predicted output content, such as the content of a text file that is
/// being regenerated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PredictionContent {
	/// The content that should be matched when generating a model response.
	/// If generated tokens would match this content, the entire model response
	/// can be returned much more quickly.
	pub content: PredictionContentContent,
	/// The type of the predicted content you want to provide. This type is
	/// currently always `content`.
	pub r#type: PredictionContentType,
}
/// The content that should be matched when generating a model response.
/// If generated tokens would match this content, the entire model response
/// can be returned much more quickly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum PredictionContentContent {
	String(String),
	Array(PredictionContentContentArray),
}
/// The type of the predicted content you want to provide. This type is
/// currently always `content`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum PredictionContentType {
	Content,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectApiKeyDeleteResponse {
	pub deleted: bool,
	pub id: String,
	pub object: ProjectApiKeyDeleteResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectApiKeyDeleteResponseObject {
	#[serde(rename = "organization.project.api_key.deleted")]
	OrganizationProjectApiKeyDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectApiKeyListResponse {
	pub data: Vec<ProjectApiKey>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: ProjectApiKeyListResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectApiKeyListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectCreateRequest {
	/// The friendly name of the project, this name appears in reports.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectListResponse {
	pub data: Vec<Project>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: ProjectListResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectRateLimitListResponse {
	pub data: Vec<ProjectRateLimit>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: ProjectRateLimitListResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectRateLimitListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectRateLimitUpdateRequest {
	/// The maximum batch input tokens per day. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_1_day_max_input_tokens: Option<i64>,
	/// The maximum audio megabytes per minute. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_audio_megabytes_per_1_minute: Option<i64>,
	/// The maximum images per minute. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_images_per_1_minute: Option<i64>,
	/// The maximum requests per day. Only relevant for certain models.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_day: Option<i64>,
	/// The maximum requests per minute.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_minute: Option<i64>,
	/// The maximum tokens per minute.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens_per_1_minute: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountApiKey {
	pub created_at: i64,
	pub id: String,
	pub name: String,
	/// The object type, which is always `organization.project.service_account.api_key`
	pub object: ProjectServiceAccountApiKeyObject,
	pub value: String,
}
/// The object type, which is always `organization.project.service_account.api_key`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectServiceAccountApiKeyObject {
	#[serde(rename = "organization.project.service_account.api_key")]
	OrganizationProjectServiceAccountApiKey,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountCreateRequest {
	/// The name of the service account being created.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountCreateResponse {
	pub api_key: ProjectServiceAccountApiKey,
	pub created_at: i64,
	pub id: String,
	pub name: String,
	pub object: ProjectServiceAccountCreateResponseObject,
	/// Service accounts can only have one role of type `member`
	pub role: ProjectServiceAccountCreateResponseRole,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectServiceAccountCreateResponseObject {
	#[serde(rename = "organization.project.service_account")]
	OrganizationProjectServiceAccount,
}
/// Service accounts can only have one role of type `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectServiceAccountCreateResponseRole {
	Member,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountDeleteResponse {
	pub deleted: bool,
	pub id: String,
	pub object: ProjectServiceAccountDeleteResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectServiceAccountDeleteResponseObject {
	#[serde(rename = "organization.project.service_account.deleted")]
	OrganizationProjectServiceAccountDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectServiceAccountListResponse {
	pub data: Vec<ProjectServiceAccount>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: ProjectServiceAccountListResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectServiceAccountListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUpdateRequest {
	/// The updated name of the project, this name appears in reports.
	pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserCreateRequest {
	/// `owner` or `member`
	pub role: ProjectUserCreateRequestRole,
	/// The ID of the user.
	pub user_id: String,
}
/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectUserCreateRequestRole {
	Owner,
	Member,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserDeleteResponse {
	pub deleted: bool,
	pub id: String,
	pub object: ProjectUserDeleteResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectUserDeleteResponseObject {
	#[serde(rename = "organization.project.user.deleted")]
	OrganizationProjectUserDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserListResponse {
	pub data: Vec<ProjectUser>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserUpdateRequest {
	/// `owner` or `member`
	pub role: ProjectUserUpdateRequestRole,
}
/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ProjectUserUpdateRequestRole {
	Owner,
	Member,
}
/// The item to add to the conversation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeConversationItem {
	/// The arguments of the function call (for `function_call` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/// The ID of the function call (for `function_call` and 
	/// `function_call_output` items). If passed on a `function_call_output` 
	/// item, the server will check that a `function_call` item with the same 
	/// ID exists in the conversation history.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub call_id: Option<String>,
	/// The content of the message, applicable for `message` items. 
	/// - Message items of role `system` support only `input_text` content
	/// - Message items of role `user` support `input_text` and `input_audio` 
	///   content
	/// - Message items of role `assistant` support `text` content.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<RealtimeConversationItemContent>>,
	/// The unique ID of the item, this can be generated by the client to help 
	/// manage server-side context, but is not required because the server will 
	/// generate one if not provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The name of the function being called (for `function_call` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Identifier for the API object being returned - always `realtime.item`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeConversationItemObject>,
	/// The output of the function call (for `function_call_output` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
	/// The role of the message sender (`user`, `assistant`, `system`), only 
	/// applicable for `message` items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<RealtimeConversationItemRole>,
	/// The status of the item (`completed`, `incomplete`). These have no effect 
	/// on the conversation, but are accepted for consistency with the 
	/// `conversation.item.created` event.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeConversationItemStatus>,
	/// The type of the item (`message`, `function_call`, `function_call_output`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemType>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeConversationItemContent {
	/// Base64-encoded audio bytes, used for `input_audio` content type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/// ID of a previous conversation item to reference (for `item_reference`
	/// content types in `response.create` events). These can reference both
	/// client and server created items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The text content, used for `input_text` and `text` content types.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// The transcript of the audio, used for `input_audio` content type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
	/// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemContentType>,
}
/// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemContentType {
	#[serde(rename = "input_audio")]
	InputAudio,
	#[serde(rename = "input_text")]
	InputText,
	#[serde(rename = "item_reference")]
	ItemReference,
	Text,
}
/// Identifier for the API object being returned - always `realtime.item`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemObject {
	#[serde(rename = "realtime.item")]
	RealtimeItem,
}
/// The role of the message sender (`user`, `assistant`, `system`), only 
/// applicable for `message` items.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemRole {
	User,
	Assistant,
	System,
}
/// The status of the item (`completed`, `incomplete`). These have no effect 
/// on the conversation, but are accepted for consistency with the 
/// `conversation.item.created` event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemStatus {
	Completed,
	Incomplete,
}
/// The type of the item (`message`, `function_call`, `function_call_output`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemType {
	Message,
	#[serde(rename = "function_call")]
	FunctionCall,
	#[serde(rename = "function_call_output")]
	FunctionCallOutput,
}
/// The item to add to the conversation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeConversationItemWithReference {
	/// The arguments of the function call (for `function_call` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/// The ID of the function call (for `function_call` and 
	/// `function_call_output` items). If passed on a `function_call_output` 
	/// item, the server will check that a `function_call` item with the same 
	/// ID exists in the conversation history.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub call_id: Option<String>,
	/// The content of the message, applicable for `message` items. 
	/// - Message items of role `system` support only `input_text` content
	/// - Message items of role `user` support `input_text` and `input_audio` 
	///   content
	/// - Message items of role `assistant` support `text` content.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<RealtimeConversationItemWithReferenceContent>>,
	/// For an item of type (`message` | `function_call` | `function_call_output`)
	/// this field allows the client to assign the unique ID of the item. It is
	/// not required because the server will generate one if not provided.
	/// 
	/// For an item of type `item_reference`, this field is required and is a
	/// reference to any item that has previously existed in the conversation.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The name of the function being called (for `function_call` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Identifier for the API object being returned - always `realtime.item`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeConversationItemWithReferenceObject>,
	/// The output of the function call (for `function_call_output` items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
	/// The role of the message sender (`user`, `assistant`, `system`), only 
	/// applicable for `message` items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<RealtimeConversationItemWithReferenceRole>,
	/// The status of the item (`completed`, `incomplete`). These have no effect 
	/// on the conversation, but are accepted for consistency with the 
	/// `conversation.item.created` event.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeConversationItemWithReferenceStatus>,
	/// The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemWithReferenceType>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeConversationItemWithReferenceContent {
	/// Base64-encoded audio bytes, used for `input_audio` content type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/// ID of a previous conversation item to reference (for `item_reference`
	/// content types in `response.create` events). These can reference both
	/// client and server created items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The text content, used for `input_text` and `text` content types.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// The transcript of the audio, used for `input_audio` content type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
	/// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemWithReferenceContentType>,
}
/// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemWithReferenceContentType {
	#[serde(rename = "input_audio")]
	InputAudio,
	#[serde(rename = "input_text")]
	InputText,
	#[serde(rename = "item_reference")]
	ItemReference,
	Text,
}
/// Identifier for the API object being returned - always `realtime.item`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemWithReferenceObject {
	#[serde(rename = "realtime.item")]
	RealtimeItem,
}
/// The role of the message sender (`user`, `assistant`, `system`), only 
/// applicable for `message` items.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemWithReferenceRole {
	User,
	Assistant,
	System,
}
/// The status of the item (`completed`, `incomplete`). These have no effect 
/// on the conversation, but are accepted for consistency with the 
/// `conversation.item.created` event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemWithReferenceStatus {
	Completed,
	Incomplete,
}
/// The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeConversationItemWithReferenceType {
	Message,
	#[serde(rename = "function_call")]
	FunctionCall,
	#[serde(rename = "function_call_output")]
	FunctionCallOutput,
}
/// The response resource.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponse {
	/// Which conversation the response is added to, determined by the `conversation`
	/// field in the `response.create` event. If `auto`, the response will be added to
	/// the default conversation and the value of `conversation_id` will be an id like
	/// `conv_1234`. If `none`, the response will not be added to any conversation and
	/// the value of `conversation_id` will be `null`. If responses are being triggered
	/// by server VAD, the response will be added to the default conversation, thus
	/// the `conversation_id` will be an id like `conv_1234`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub conversation_id: Option<String>,
	/// The unique ID of the response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// Maximum number of output tokens for a single assistant response,
	/// inclusive of tool calls, that was used in this response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_output_tokens: Option<RealtimeResponseMaxOutputTokens>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The set of modalities the model used to respond. If there are multiple modalities,
	/// the model will pick one, for example if `modalities` is `["text", "audio"]`, the model
	/// could be responding in either text or audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeResponseModalities>>,
	/// The object type, must be `realtime.response`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeResponseObject>,
	/// The list of output items generated by the response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<Vec<RealtimeConversationItem>>,
	/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeResponseOutputAudioFormat>,
	/// The final status of the response (`completed`, `cancelled`, `failed`, or 
	/// `incomplete`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeResponseStatus>,
	/// Additional details about the status.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_details: Option<RealtimeResponseStatusDetails>,
	/// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// Usage statistics for the Response, this will correspond to billing. A 
	/// Realtime API session will maintain a conversation context and append new 
	/// Items to the Conversation, thus output from previous turns (text and 
	/// audio tokens) will become the input for later turns.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<RealtimeResponseUsage>,
	/// The voice the model used to respond.
	/// Current voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
	/// `onyx`, `nova`, `sage`, `shimmer`, and `verse`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
}
/// Create a new Realtime response with these parameters
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseCreateParams {
	/// Controls which conversation the response is added to. Currently supports
	/// `auto` and `none`, with `auto` as the default value. The `auto` value
	/// means that the contents of the response will be added to the default
	/// conversation. Set this to `none` to create an out-of-band response which 
	/// will not add items to default conversation.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub conversation: Option<RealtimeResponseCreateParamsConversation>,
	/// Input items to include in the prompt for the model. Using this field
	/// creates a new context for this Response instead of using the default
	/// conversation. An empty array `[]` will clear the context for this Response.
	/// Note that this can include references to items from the default conversation.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<Vec<RealtimeConversationItemWithReference>>,
	/// The default system instructions (i.e. system message) prepended to model 
	/// calls. This field allows the client to guide the model on desired 
	/// responses. The model can be instructed on response content and format, 
	/// (e.g. "be extremely succinct", "act friendly", "here are examples of good 
	/// responses") and on audio behavior (e.g. "talk quickly", "inject emotion 
	/// into your voice", "laugh frequently"). The instructions are not guaranteed 
	/// to be followed by the model, but they provide guidance to the model on the 
	/// desired behavior.
	/// 
	/// Note that the server sets default instructions which will be used if this 
	/// field is not set and are visible in the `session.created` event at the 
	/// start of the session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// Maximum number of output tokens for a single assistant response,
	/// inclusive of tool calls. Provide an integer between 1 and 4096 to
	/// limit output tokens, or `inf` for the maximum available tokens for a
	/// given model. Defaults to `inf`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeResponseCreateParamsMaxResponseOutputTokens>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeResponseCreateParamsModalities>>,
	/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeResponseCreateParamsOutputAudioFormat>,
	/// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// How the model chooses tools. Options are `auto`, `none`, `required`, or 
	/// specify a function, like `{"type": "function", "function": {"name": "my_function"}}`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/// Tools (functions) available to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeResponseCreateParamsTools>>,
	/// The voice the model uses to respond. Voice cannot be changed during the 
	/// session once the model has responded with audio at least once. Current 
	/// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
	/// `onyx`, `nova`, `sage`, `shimmer`, and `verse`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
}
/// Controls which conversation the response is added to. Currently supports
/// `auto` and `none`, with `auto` as the default value. The `auto` value
/// means that the contents of the response will be added to the default
/// conversation. Set this to `none` to create an out-of-band response which 
/// will not add items to default conversation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsConversation {
	String(String),
	Auto(String),
	None(String),
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens {
	Integer(i64),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsModalities {
	Text,
	Audio,
}
/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsOutputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseCreateParamsTools {
	/// The description of the function, including guidance on when and how 
	/// to call it, and guidance about what to tell the user when calling 
	/// (if anything).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Parameters of the function in JSON Schema.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeResponseCreateParamsToolsParameters>,
	/// The type of the tool, i.e. `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeResponseCreateParamsToolsType>,
}
/// Parameters of the function in JSON Schema.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseCreateParamsToolsParameters {
}
/// The type of the tool, i.e. `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsToolsType {
	Function,
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls, that was used in this response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseMaxOutputTokens {
	Integer(i64),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseModalities {
	Text,
	Audio,
}
/// The object type, must be `realtime.response`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseObject {
	#[serde(rename = "realtime.response")]
	RealtimeResponse,
}
/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseOutputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// The final status of the response (`completed`, `cancelled`, `failed`, or 
/// `incomplete`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseStatus {
	Completed,
	Cancelled,
	Failed,
	Incomplete,
}
/// Additional details about the status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseStatusDetails {
	/// A description of the error that caused the response to fail, 
	/// populated when the `status` is `failed`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<RealtimeResponseStatusDetailsError>,
	/// The reason the Response did not complete. For a `cancelled` Response, 
	/// one of `turn_detected` (the server VAD detected a new start of speech) 
	/// or `client_cancelled` (the client sent a cancel event). For an 
	/// `incomplete` Response, one of `max_output_tokens` or `content_filter` 
	/// (the server-side safety filter activated and cut off the response).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<RealtimeResponseStatusDetailsReason>,
	/// The type of error that caused the response to fail, corresponding 
	/// with the `status` field (`completed`, `cancelled`, `incomplete`, 
	/// `failed`).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeResponseStatusDetailsType>,
}
/// A description of the error that caused the response to fail, 
/// populated when the `status` is `failed`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseStatusDetailsError {
	/// Error code, if any.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/// The type of error.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
/// The reason the Response did not complete. For a `cancelled` Response, 
/// one of `turn_detected` (the server VAD detected a new start of speech) 
/// or `client_cancelled` (the client sent a cancel event). For an 
/// `incomplete` Response, one of `max_output_tokens` or `content_filter` 
/// (the server-side safety filter activated and cut off the response).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseStatusDetailsReason {
	#[serde(rename = "turn_detected")]
	TurnDetected,
	#[serde(rename = "client_cancelled")]
	ClientCancelled,
	#[serde(rename = "max_output_tokens")]
	MaxOutputTokens,
	#[serde(rename = "content_filter")]
	ContentFilter,
}
/// The type of error that caused the response to fail, corresponding 
/// with the `status` field (`completed`, `cancelled`, `incomplete`, 
/// `failed`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeResponseStatusDetailsType {
	Completed,
	Cancelled,
	Failed,
	Incomplete,
}
/// Usage statistics for the Response, this will correspond to billing. A 
/// Realtime API session will maintain a conversation context and append new 
/// Items to the Conversation, thus output from previous turns (text and 
/// audio tokens) will become the input for later turns.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseUsage {
	/// Details about the input tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_token_details: Option<RealtimeResponseUsageInputTokenDetails>,
	/// The number of input tokens used in the Response, including text and 
	/// audio tokens.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_tokens: Option<i64>,
	/// Details about the output tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_token_details: Option<RealtimeResponseUsageOutputTokenDetails>,
	/// The number of output tokens sent in the Response, including text and 
	/// audio tokens.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_tokens: Option<i64>,
	/// The total number of tokens in the Response including input and output 
	/// text and audio tokens.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_tokens: Option<i64>,
}
/// Details about the input tokens used in the Response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseUsageInputTokenDetails {
	/// The number of audio tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<i64>,
	/// The number of cached tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cached_tokens: Option<i64>,
	/// The number of text tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_tokens: Option<i64>,
}
/// Details about the output tokens used in the Response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeResponseUsageOutputTokenDetails {
	/// The number of audio tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<i64>,
	/// The number of text tokens used in the Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_tokens: Option<i64>,
}
/// Realtime session object configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSession {
	/// Unique identifier for the session that looks like `sess_1234567890abcdef`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
	/// single channel (mono), and little-endian byte order.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeSessionInputAudioFormat>,
	/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
	/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
	/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeSessionInputAudioNoiseReduction>,
	/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
	/// The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. "be extremely succinct", "act friendly", "here are examples of good  responses") and on audio behavior (e.g. "talk quickly", "inject emotion  into your voice", "laugh frequently"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.
	/// 
	/// Note that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// Maximum number of output tokens for a single assistant response,
	/// inclusive of tool calls. Provide an integer between 1 and 4096 to
	/// limit output tokens, or `inf` for the maximum available tokens for a
	/// given model. Defaults to `inf`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionMaxResponseOutputTokens>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeSessionModalities>>,
	/// The Realtime model used for this session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeSessionModel>,
	/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, output audio is sampled at a rate of 24kHz.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeSessionOutputAudioFormat>,
	/// Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// How the model chooses tools. Options are `auto`, `none`, `required`, or 
	/// specify a function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/// Tools (functions) available to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionTools>>,
	/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
	/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
	/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionTurnDetection>,
	/// The voice the model uses to respond. Voice cannot be changed during the 
	/// session once the model has responded with audio at least once. Current 
	/// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, 
	/// `shimmer` and `verse`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
}
/// Realtime session object configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequest {
	/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
	/// single channel (mono), and little-endian byte order.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeSessionCreateRequestInputAudioFormat>,
	/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
	/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
	/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeSessionCreateRequestInputAudioNoiseReduction>,
	/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionCreateRequestInputAudioTranscription>,
	/// The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. "be extremely succinct", "act friendly", "here are examples of good  responses") and on audio behavior (e.g. "talk quickly", "inject emotion  into your voice", "laugh frequently"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.
	/// 
	/// Note that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// Maximum number of output tokens for a single assistant response,
	/// inclusive of tool calls. Provide an integer between 1 and 4096 to
	/// limit output tokens, or `inf` for the maximum available tokens for a
	/// given model. Defaults to `inf`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeSessionCreateRequestModalities>>,
	/// The Realtime model used for this session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeSessionCreateRequestModel>,
	/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, output audio is sampled at a rate of 24kHz.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeSessionCreateRequestOutputAudioFormat>,
	/// Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// How the model chooses tools. Options are `auto`, `none`, `required`, or 
	/// specify a function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/// Tools (functions) available to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionCreateRequestTools>>,
	/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
	/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
	/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionCreateRequestTurnDetection>,
	/// The voice the model uses to respond. Voice cannot be changed during the 
	/// session once the model has responded with audio at least once. Current 
	/// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
	/// `onyx`, `nova`, `sage`, `shimmer`, and `verse`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
}
/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
/// single channel (mono), and little-endian byte order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestInputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
	/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestInputAudioNoiseReductionType>,
}
/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename = "near_field")]
	NearField,
	#[serde(rename = "far_field")]
	FarField,
}
/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
	/// The language of the input audio. Supplying the input language in
	/// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
	/// will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// An optional text to guide the model's style or continue a previous audio
	/// segment.
	/// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting).
	/// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
	Integer(i64),
}
/// The set of modalities the model can respond with. To disable audio,
/// set this to ["text"].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestModalities {
}
/// The Realtime model used for this session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestModel {
	#[serde(rename = "gpt-4o-realtime-preview")]
	Gpt4ORealtimePreview,
	#[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
	Gpt4ORealtimePreview20241001,
	#[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
	Gpt4ORealtimePreview20241217,
	#[serde(rename = "gpt-4o-mini-realtime-preview")]
	Gpt4OMiniRealtimePreview,
	#[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
	Gpt4OMiniRealtimePreview20241217,
}
/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, output audio is sampled at a rate of 24kHz.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestOutputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestTools {
	/// The description of the function, including guidance on when and how 
	/// to call it, and guidance about what to tell the user when calling 
	/// (if anything).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Parameters of the function in JSON Schema.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeSessionCreateRequestToolsParameters>,
	/// The type of the tool, i.e. `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestToolsType>,
}
/// Parameters of the function in JSON Schema.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestToolsParameters {
}
/// The type of the tool, i.e. `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestToolsType {
	Function,
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionCreateRequestTurnDetection {
	/// Whether or not to automatically generate a response when a VAD stop event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeSessionCreateRequestTurnDetectionEagerness>,
	/// Whether or not to automatically interrupt any ongoing response with output to the default
	/// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
	/// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
	/// milliseconds). Defaults to 300ms.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<i64>,
	/// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
	/// to 500ms. With shorter values the model will respond more quickly, 
	/// but may jump in on short pauses from the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<i64>,
	/// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
	/// higher threshold will require louder audio to activate the model, and 
	/// thus might perform better in noisy environments.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f64>,
	/// Type of turn detection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestTurnDetectionType>,
}
/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestTurnDetectionEagerness {
	Low,
	Medium,
	High,
	Auto,
}
/// Type of turn detection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestTurnDetectionType {
	#[serde(rename = "server_vad")]
	ServerVad,
	#[serde(rename = "semantic_vad")]
	SemanticVad,
}
/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
/// single channel (mono), and little-endian byte order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionInputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionInputAudioNoiseReduction {
	/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionInputAudioNoiseReductionType>,
}
/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionInputAudioNoiseReductionType {
	#[serde(rename = "near_field")]
	NearField,
	#[serde(rename = "far_field")]
	FarField,
}
/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionInputAudioTranscription {
	/// The language of the input audio. Supplying the input language in
	/// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
	/// will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// An optional text to guide the model's style or continue a previous audio
	/// segment.
	/// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting).
	/// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionMaxResponseOutputTokens {
	Integer(i64),
}
/// The set of modalities the model can respond with. To disable audio,
/// set this to ["text"].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionModalities {
}
/// The Realtime model used for this session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionModel {
	#[serde(rename = "gpt-4o-realtime-preview")]
	Gpt4ORealtimePreview,
	#[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
	Gpt4ORealtimePreview20241001,
	#[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
	Gpt4ORealtimePreview20241217,
	#[serde(rename = "gpt-4o-mini-realtime-preview")]
	Gpt4OMiniRealtimePreview,
	#[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
	Gpt4OMiniRealtimePreview20241217,
}
/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, output audio is sampled at a rate of 24kHz.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionOutputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionTools {
	/// The description of the function, including guidance on when and how 
	/// to call it, and guidance about what to tell the user when calling 
	/// (if anything).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Parameters of the function in JSON Schema.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeSessionToolsParameters>,
	/// The type of the tool, i.e. `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionToolsType>,
}
/// Parameters of the function in JSON Schema.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionToolsParameters {
}
/// The type of the tool, i.e. `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionToolsType {
	Function,
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeSessionTurnDetection {
	/// Whether or not to automatically generate a response when a VAD stop event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeSessionTurnDetectionEagerness>,
	/// Whether or not to automatically interrupt any ongoing response with output to the default
	/// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
	/// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
	/// milliseconds). Defaults to 300ms.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<i64>,
	/// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
	/// to 500ms. With shorter values the model will respond more quickly, 
	/// but may jump in on short pauses from the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<i64>,
	/// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
	/// higher threshold will require louder audio to activate the model, and 
	/// thus might perform better in noisy environments.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f64>,
	/// Type of turn detection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionTurnDetectionType>,
}
/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionTurnDetectionEagerness {
	Low,
	Medium,
	High,
	Auto,
}
/// Type of turn detection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionTurnDetectionType {
	#[serde(rename = "server_vad")]
	ServerVad,
	#[serde(rename = "semantic_vad")]
	SemanticVad,
}
/// Realtime transcription session object configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequest {
	/// The set of items to include in the transcription. Current available items are:
	/// - `item.input_audio_transcription.logprobs`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<String>>,
	/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
	/// single channel (mono), and little-endian byte order.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeTranscriptionSessionCreateRequestInputAudioFormat>,
	/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
	/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
	/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
	/// Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscription>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeTranscriptionSessionCreateRequestModalities>>,
	/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
	/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
	/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeTranscriptionSessionCreateRequestTurnDetection>,
}
/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
/// single channel (mono), and little-endian byte order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
	/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType>,
}
/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename = "near_field")]
	NearField,
	#[serde(rename = "far_field")]
	FarField,
}
/// Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioTranscription {
	/// The language of the input audio. Supplying the input language in
	/// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
	/// will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel>,
	/// An optional text to guide the model's style or continue a previous audio
	/// segment.
	/// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting).
	/// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}
/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel {
	#[serde(rename = "gpt-4o-transcribe")]
	Gpt4OTranscribe,
	#[serde(rename = "gpt-4o-mini-transcribe")]
	Gpt4OMiniTranscribe,
	#[serde(rename = "whisper-1")]
	Whisper1,
}
/// The set of modalities the model can respond with. To disable audio,
/// set this to ["text"].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestModalities {
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
	/// Whether or not to automatically generate a response when a VAD stop event occurs. Not available for transcription sessions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness>,
	/// Whether or not to automatically interrupt any ongoing response with output to the default
	/// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs. Not available for transcription sessions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
	/// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
	/// milliseconds). Defaults to 300ms.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<i64>,
	/// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
	/// to 500ms. With shorter values the model will respond more quickly, 
	/// but may jump in on short pauses from the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<i64>,
	/// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
	/// higher threshold will require louder audio to activate the model, and 
	/// thus might perform better in noisy environments.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f64>,
	/// Type of turn detection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionType>,
}
/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness {
	Low,
	Medium,
	High,
	Auto,
}
/// Type of turn detection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
	#[serde(rename = "server_vad")]
	ServerVad,
	#[serde(rename = "semantic_vad")]
	SemanticVad,
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
	pub summary: Vec<ReasoningItemSummary>,
	/// The type of the object. Always `reasoning`.
	pub r#type: ReasoningItemType,
}
/// The status of the item. One of `in_progress`, `completed`, or
/// `incomplete`. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ReasoningItemStatus {
	#[serde(rename = "in_progress")]
	InProgress,
	Completed,
	Incomplete,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItemSummary {
	/// A short summary of the reasoning used by the model when generating
	/// the response.
	pub text: String,
	/// The type of the object. Always `summary_text`.
	pub r#type: ReasoningItemSummaryType,
}
/// The type of the object. Always `summary_text`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ReasoningItemSummaryType {
	#[serde(rename = "summary_text")]
	SummaryText,
}
/// The type of the object. Always `reasoning`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum RefusalType {
	Refusal,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Response {
	/// Unix timestamp (in seconds) of when this Response was created.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_at: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<ResponseError>,
	/// Unique identifier for this Response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// Details about why the response is incomplete.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incomplete_details: Option<ResponseIncompleteDetails>,
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
	/// The object type of this resource - always set to `response`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<ResponseObject>,
	/// An array of content items generated by the model.
	/// 
	/// - The length and order of items in the `output` array is dependent
	///   on the model's response.
	/// - Rather than accessing the first item in the `output` array and 
	///   assuming it's an `assistant` message with the content generated by
	///   the model, you might consider using the `output_text` property where
	///   supported in SDKs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<Vec<OutputItem>>,
	/// SDK-only convenience property that contains the aggregated text output 
	/// from all `output_text` items in the `output` array, if any are present. 
	/// Supported in the Python and JavaScript SDKs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_text: Option<String>,
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
	/// The status of the response generation. One of `completed`, `failed`, 
	/// `in_progress`, or `incomplete`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ResponseStatus>,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<ResponseUsage>,
	/// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}
/// An error object returned when the model fails to generate a Response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseError {
	pub code: ResponseErrorCode,
	/// A human-readable description of the error.
	pub message: String,
}
/// The error code for the response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseErrorCode {
	#[serde(rename = "server_error")]
	ServerError,
	#[serde(rename = "rate_limit_exceeded")]
	RateLimitExceeded,
	#[serde(rename = "invalid_prompt")]
	InvalidPrompt,
	#[serde(rename = "vector_store_timeout")]
	VectorStoreTimeout,
	#[serde(rename = "invalid_image")]
	InvalidImage,
	#[serde(rename = "invalid_image_format")]
	InvalidImageFormat,
	#[serde(rename = "invalid_base64_image")]
	InvalidBase64Image,
	#[serde(rename = "invalid_image_url")]
	InvalidImageUrl,
	#[serde(rename = "image_too_large")]
	ImageTooLarge,
	#[serde(rename = "image_too_small")]
	ImageTooSmall,
	#[serde(rename = "image_parse_error")]
	ImageParseError,
	#[serde(rename = "image_content_policy_violation")]
	ImageContentPolicyViolation,
	#[serde(rename = "invalid_image_mode")]
	InvalidImageMode,
	#[serde(rename = "image_file_too_large")]
	ImageFileTooLarge,
	#[serde(rename = "unsupported_image_media_type")]
	UnsupportedImageMediaType,
	#[serde(rename = "empty_image_file")]
	EmptyImageFile,
	#[serde(rename = "failed_to_download_image")]
	FailedToDownloadImage,
	#[serde(rename = "image_file_not_found")]
	ImageFileNotFound,
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
/// JSON Schema response format. Used to generate structured JSON responses.
/// Learn more about [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFormatJsonSchema {
	/// Structured Outputs configuration options, including a JSON Schema.
	pub json_schema: ResponseFormatJsonSchemaJsonSchema,
	/// The type of response format being defined. Always `json_schema`.
	pub r#type: ResponseFormatJsonSchemaType,
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
#[serde(untagged)]
pub enum ResponseFormatJsonSchemaType {
	#[serde(rename = "json_schema")]
	JsonSchema,
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
/// Details about why the response is incomplete.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseIncompleteDetails {
	/// The reason why the response is incomplete.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<ResponseIncompleteDetailsReason>,
}
/// The reason why the response is incomplete.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseIncompleteDetailsReason {
	#[serde(rename = "max_output_tokens")]
	MaxOutputTokens,
	#[serde(rename = "content_filter")]
	ContentFilter,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseModalitiesResponseModalities {
	Text,
	Audio,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseModalitiesTextOnlyResponseModalitiesTextOnly {
	Text,
}
/// The object type of this resource - always set to `response`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseObject {
	Response,
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
/// The status of the response generation. One of `completed`, `failed`, 
/// `in_progress`, or `incomplete`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ResponseStatus {
	Completed,
	Failed,
	#[serde(rename = "in_progress")]
	InProgress,
	Incomplete,
}
/// Represents token usage details including input tokens, output tokens,
/// a breakdown of output tokens, and the total tokens used.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUsage {
	/// The number of input tokens.
	pub input_tokens: i64,
	/// A detailed breakdown of the input tokens.
	pub input_tokens_details: ResponseUsageInputTokensDetails,
	/// The number of output tokens.
	pub output_tokens: i64,
	/// A detailed breakdown of the output tokens.
	pub output_tokens_details: ResponseUsageOutputTokensDetails,
	/// The total number of tokens used.
	pub total_tokens: i64,
}
/// A detailed breakdown of the input tokens.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUsageInputTokensDetails {
	/// The number of tokens that were retrieved from the cache. 
	/// [More on prompt caching](https://platform.openai.com/docs/guides/prompt-caching).
	pub cached_tokens: i64,
}
/// A detailed breakdown of the output tokens.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUsageOutputTokensDetails {
	/// The number of reasoning tokens.
	pub reasoning_tokens: i64,
}
/// Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunCompletionUsage {
	/// Number of completion tokens used over the course of the run.
	pub completion_tokens: i64,
	/// Number of prompt tokens used over the course of the run.
	pub prompt_tokens: i64,
	/// Total number of tokens used (prompt + completion).
	pub total_tokens: i64,
}
/// Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepCompletionUsage {
	/// Number of completion tokens used over the course of the run step.
	pub completion_tokens: i64,
	/// Number of prompt tokens used over the course of the run step.
	pub prompt_tokens: i64,
	/// Total number of tokens used (prompt + completion).
	pub total_tokens: i64,
}
/// Details of the message creation by the run step.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_creation: Option<RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>,
	/// Always `message_creation`.
	pub r#type: RunStepDeltaStepDetailsMessageCreationObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
	/// The ID of the message that was created by this run step.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_id: Option<String>,
}
/// Always `message_creation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsMessageCreationObjectType {
	#[serde(rename = "message_creation")]
	MessageCreation,
}
/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
	/// The Code Interpreter tool call definition.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>,
	/// The ID of the tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The index of the tool call in the tool calls array.
	pub index: i64,
	/// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeObjectType,
}
/// The Code Interpreter tool call definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
	/// The input to the Code Interpreter tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<String>,
	/// The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outputs: Option<Vec<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs {
	RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject(RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject),
	RunStepDeltaStepDetailsToolCallsCodeOutputImageObject(RunStepDeltaStepDetailsToolCallsCodeOutputImageObject),
}
/// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectType {
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>,
	/// The index of the output in the outputs array.
	pub index: i64,
	/// Always `image`.
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
	/// The [file](https://platform.openai.com/docs/api-reference/files) ID of the image.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}
/// Always `image`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
	Image,
}
/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
	/// The index of the output in the outputs array.
	pub index: i64,
	/// The text output from the Code Interpreter tool call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logs: Option<String>,
	/// Always `logs`.
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
}
/// Always `logs`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
	Logs,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
	/// For now, this is always going to be an empty object.
	pub file_search: serde_json::Value,
	/// The ID of the tool call object.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The index of the tool call in the tool calls array.
	pub index: i64,
	/// The type of tool call. This is always going to be `file_search` for this type of tool call.
	pub r#type: RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
}
/// The type of tool call. This is always going to be `file_search` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
	#[serde(rename = "file_search")]
	FileSearch,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
	/// The definition of the function that was called.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>,
	/// The ID of the tool call object.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The index of the tool call in the tool calls array.
	pub index: i64,
	/// The type of tool call. This is always going to be `function` for this type of tool call.
	pub r#type: RunStepDeltaStepDetailsToolCallsFunctionObjectType,
}
/// The definition of the function that was called.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
	/// The arguments passed to the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/// The name of the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The output of the function. This will be `null` if the outputs have not been [submitted](https://platform.openai.com/docs/api-reference/runs/submitToolOutputs) yet.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}
/// The type of tool call. This is always going to be `function` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsFunctionObjectType {
	Function,
}
/// Details of the tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsObject {
	/// An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<Vec<RunStepDeltaStepDetailsToolCallsObjectToolCalls>>,
	/// Always `tool_calls`.
	pub r#type: RunStepDeltaStepDetailsToolCallsObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsObjectToolCalls {
	RunStepDeltaStepDetailsToolCallsCodeObject(RunStepDeltaStepDetailsToolCallsCodeObject),
	RunStepDeltaStepDetailsToolCallsFileSearchObject(RunStepDeltaStepDetailsToolCallsFileSearchObject),
	RunStepDeltaStepDetailsToolCallsFunctionObject(RunStepDeltaStepDetailsToolCallsFunctionObject),
}
/// Always `tool_calls`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsObjectType {
	#[serde(rename = "tool_calls")]
	ToolCalls,
}
/// Details of the message creation by the run step.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsMessageCreationObject {
	pub message_creation: RunStepDetailsMessageCreationObjectMessageCreation,
	/// Always `message_creation`.
	pub r#type: RunStepDetailsMessageCreationObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
	/// The ID of the message that was created by this run step.
	pub message_id: String,
}
/// Always `message_creation`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsMessageCreationObjectType {
	#[serde(rename = "message_creation")]
	MessageCreation,
}
/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsCodeObject {
	/// The Code Interpreter tool call definition.
	pub code_interpreter: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
	/// The ID of the tool call.
	pub id: String,
	/// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
	pub r#type: RunStepDetailsToolCallsCodeObjectType,
}
/// The Code Interpreter tool call definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
	/// The input to the Code Interpreter tool call.
	pub input: String,
	/// The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type.
	pub outputs: Vec<RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs {
	RunStepDetailsToolCallsCodeOutputLogsObject(RunStepDetailsToolCallsCodeOutputLogsObject),
	RunStepDetailsToolCallsCodeOutputImageObject(RunStepDetailsToolCallsCodeOutputImageObject),
}
/// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsCodeObjectType {
	#[serde(rename = "code_interpreter")]
	CodeInterpreter,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
	pub image: RunStepDetailsToolCallsCodeOutputImageObjectImage,
	/// Always `image`.
	pub r#type: RunStepDetailsToolCallsCodeOutputImageObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
	/// The [file](https://platform.openai.com/docs/api-reference/files) ID of the image.
	pub file_id: String,
}
/// Always `image`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsCodeOutputImageObjectType {
	Image,
}
/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
	/// The text output from the Code Interpreter tool call.
	pub logs: String,
	/// Always `logs`.
	pub r#type: RunStepDetailsToolCallsCodeOutputLogsObjectType,
}
/// Always `logs`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsCodeOutputLogsObjectType {
	Logs,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchObject {
	/// For now, this is always going to be an empty object.
	pub file_search: serde_json::Value,
	/// The ID of the tool call object.
	pub id: String,
	/// The type of tool call. This is always going to be `file_search` for this type of tool call.
	pub r#type: RunStepDetailsToolCallsFileSearchObjectType,
}
/// The type of tool call. This is always going to be `file_search` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsFileSearchObjectType {
	#[serde(rename = "file_search")]
	FileSearch,
}
/// The ranking options for the file search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
	pub ranker: FileSearchRanker,
	/// The score threshold for the file search. All values must be a floating point number between 0 and 1.
	pub score_threshold: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsFunctionObject {
	/// The definition of the function that was called.
	pub function: RunStepDetailsToolCallsFunctionObjectFunction,
	/// The ID of the tool call object.
	pub id: String,
	/// The type of tool call. This is always going to be `function` for this type of tool call.
	pub r#type: RunStepDetailsToolCallsFunctionObjectType,
}
/// The definition of the function that was called.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
	/// The arguments passed to the function.
	pub arguments: String,
	/// The name of the function.
	pub name: String,
	/// The output of the function. This will be `null` if the outputs have not been [submitted](https://platform.openai.com/docs/api-reference/runs/submitToolOutputs) yet.
	pub output: String,
}
/// The type of tool call. This is always going to be `function` for this type of tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsFunctionObjectType {
	Function,
}
/// Details of the tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunStepDetailsToolCallsObject {
	/// An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.
	pub tool_calls: Vec<RunStepDetailsToolCallsObjectToolCalls>,
	/// Always `tool_calls`.
	pub r#type: RunStepDetailsToolCallsObjectType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsObjectToolCalls {
	RunStepDetailsToolCallsCodeObject(RunStepDetailsToolCallsCodeObject),
	RunStepDetailsToolCallsFileSearchObject(RunStepDetailsToolCallsFileSearchObject),
	RunStepDetailsToolCallsFunctionObject(RunStepDetailsToolCallsFunctionObject),
}
/// Always `tool_calls`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsObjectType {
	#[serde(rename = "tool_calls")]
	ToolCalls,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStepStreamEvent {
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunStreamEvent {
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
	Object(serde_json::Value),
}
/// Tool call objects
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunToolCallObject {
	/// The function definition.
	pub function: RunToolCallObjectFunction,
	/// The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](https://platform.openai.com/docs/api-reference/runs/submitToolOutputs) endpoint.
	pub id: String,
	/// The type of tool call the output is required for. For now, this is always `function`.
	pub r#type: RunToolCallObjectType,
}
/// The function definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunToolCallObjectFunction {
	/// The arguments that the model expects you to pass to the function.
	pub arguments: String,
	/// The name of the function.
	pub name: String,
}
/// The type of tool call the output is required for. For now, this is always `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RunToolCallObjectType {
	Function,
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum ScrollType {
	Scroll,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StaticChunkingStrategy {
	/// The number of tokens that overlap between chunks. The default value is `400`.
	/// 
	/// Note that the overlap must not exceed half of `max_chunk_size_tokens`.
	pub chunk_overlap_tokens: i64,
	/// The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`.
	pub max_chunk_size_tokens: i64,
}
/// Customize your own chunking strategy by setting chunk size and chunk overlap.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StaticChunkingStrategyRequestParam {
	pub r#static: StaticChunkingStrategy,
	/// Always `static`.
	pub r#type: StaticChunkingStrategyRequestParamType,
}
/// Always `static`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum StaticChunkingStrategyRequestParamType {
	Static,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StaticChunkingStrategyResponseParam {
	pub r#static: StaticChunkingStrategy,
	/// Always `static`.
	pub r#type: StaticChunkingStrategyResponseParamType,
}
/// Always `static`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum StaticChunkingStrategyResponseParamType {
	Static,
}
/// Up to 4 sequences where the API will stop generating further tokens. The
/// returned text will not contain the stop sequence.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum StopConfiguration {
	String(String),
	Array(StopConfigurationArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SubmitToolOutputsRunRequest {
	/// If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/// A list of tools for which the outputs are being submitted.
	pub tool_outputs: Vec<SubmitToolOutputsRunRequestToolOutputs>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SubmitToolOutputsRunRequestToolOutputs {
	/// The output of the tool call to be submitted to continue the run.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
	/// The ID of the tool call in the `required_action` object within the run object the output is being submitted for.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_call_id: Option<String>,
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
pub enum ThreadStreamEvent {
	Object(serde_json::Value),
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum TranscriptionInclude {
	Logprobs,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TranscriptionSegment {
	/// Average logprob of the segment. If the value is lower than -1, consider the logprobs failed.
	pub avg_logprob: f64,
	/// Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed.
	pub compression_ratio: f64,
	/// End time of the segment in seconds.
	pub end: f64,
	/// Unique identifier of the segment.
	pub id: i64,
	/// Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent.
	pub no_speech_prob: f64,
	/// Seek offset of the segment.
	pub seek: i64,
	/// Start time of the segment in seconds.
	pub start: f64,
	/// Temperature parameter used for generating the segment.
	pub temperature: f64,
	/// Text content of the segment.
	pub text: String,
	/// Array of token IDs for the text content.
	pub tokens: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TranscriptionWord {
	/// End time of the word in seconds.
	pub end: f64,
	/// Start time of the word in seconds.
	pub start: f64,
	/// The text content of the word.
	pub word: String,
}
/// Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TruncationObject {
	/// The number of most recent messages from the thread when constructing the context for the run.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_messages: Option<i64>,
	/// The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`.
	pub r#type: TruncationObjectType,
}
/// The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum TruncationObjectType {
	Auto,
	#[serde(rename = "last_messages")]
	LastMessages,
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
#[serde(untagged)]
pub enum TypeType {
	Type,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateVectorStoreFileAttributesRequest {
	pub attributes: VectorStoreFileAttributes,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateVectorStoreRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<UpdateVectorStoreRequestExpiresAfter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/// The name of the vector store.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateVectorStoreRequestExpiresAfter {
	/// Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`.
	pub anchor: VectorStoreExpirationAfterAnchor,
	/// The number of days after the anchor time that the vector store will expire.
	pub days: i64,
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
#[serde(untagged)]
pub enum UrlCitationType {
	#[serde(rename = "url_citation")]
	UrlCitation,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UsageResponse {
	pub data: Vec<UsageTimeBucket>,
	pub has_more: bool,
	pub next_page: String,
	pub object: UsageResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UsageResponseObject {
	Page,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UsageTimeBucket {
	pub end_time: i64,
	pub object: UsageTimeBucketObject,
	pub result: Vec<UsageTimeBucketResult>,
	pub start_time: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UsageTimeBucketObject {
	Bucket,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UsageTimeBucketResult {
	UsageCompletionsResult(UsageCompletionsResult),
	UsageEmbeddingsResult(UsageEmbeddingsResult),
	UsageModerationsResult(UsageModerationsResult),
	UsageImagesResult(UsageImagesResult),
	UsageAudioSpeechesResult(UsageAudioSpeechesResult),
	UsageAudioTranscriptionsResult(UsageAudioTranscriptionsResult),
	UsageVectorStoresResult(UsageVectorStoresResult),
	UsageCodeInterpreterSessionsResult(UsageCodeInterpreterSessionsResult),
	CostsResult(CostsResult),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDeleteResponse {
	pub deleted: bool,
	pub id: String,
	pub object: UserDeleteResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UserDeleteResponseObject {
	#[serde(rename = "organization.user.deleted")]
	OrganizationUserDeleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserListResponse {
	pub data: Vec<User>,
	pub first_id: String,
	pub has_more: bool,
	pub last_id: String,
	pub object: UserListResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UserListResponseObject {
	List,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRoleUpdateRequest {
	/// `owner` or `reader`
	pub role: UserRoleUpdateRequestRole,
}
/// `owner` or `reader`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum UserRoleUpdateRequestRole {
	Owner,
	Reader,
}
/// The expiration policy for a vector store.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreExpirationAfter {
	/// Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`.
	pub anchor: VectorStoreExpirationAfterAnchor,
	/// The number of days after the anchor time that the vector store will expire.
	pub days: i64,
}
/// Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreExpirationAfterAnchor {
	#[serde(rename = "last_active_at")]
	LastActiveAt,
}
/// Represents the parsed content of a vector store file.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreFileContentResponse {
	/// Parsed content of the file.
	pub data: Vec<VectorStoreFileContentResponseData>,
	/// Indicates if there are more content pages to fetch.
	pub has_more: bool,
	/// The token for the next page, if any.
	pub next_page: String,
	/// The object type, which is always `vector_store.file_content.page`
	pub object: VectorStoreFileContentResponseObject,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreFileContentResponseData {
	/// The text content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// The content type (currently only `"text"`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}
/// The object type, which is always `vector_store.file_content.page`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreFileContentResponseObject {
	#[serde(rename = "vector_store.file_content.page")]
	VectorStoreFileContentPage,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreSearchRequest {
	/// A filter to apply based on file attributes.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filters: Option<VectorStoreSearchRequestFilters>,
	/// The maximum number of results to return. This number should be between 1 and 50 inclusive.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<i64>,
	/// A query string for a search
	pub query: VectorStoreSearchRequestQuery,
	/// Ranking options for search.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<HashMap<String, String>>,
	/// Whether to rewrite the natural language query for vector search.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rewrite_query: Option<bool>,
}
/// A filter to apply based on file attributes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
	ComparisonFilter(ComparisonFilter),
	CompoundFilter(CompoundFilter),
}
/// A query string for a search
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreSearchRequestQuery {
	String(String),
	Array(VectorStoreSearchRequestQueryArray),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreSearchResultContentObject {
	/// The text content returned from search.
	pub text: String,
	/// The type of content.
	pub r#type: VectorStoreSearchResultContentObjectType,
}
/// The type of content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreSearchResultContentObjectType {
	Text,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreSearchResultItem {
	pub attributes: VectorStoreFileAttributes,
	/// Content chunks from the file.
	pub content: Vec<VectorStoreSearchResultContentObject>,
	/// The ID of the vector store file.
	pub file_id: String,
	/// The name of the vector store file.
	pub filename: String,
	/// The similarity score for the result.
	pub score: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VectorStoreSearchResultsPage {
	/// The list of search result items.
	pub data: Vec<VectorStoreSearchResultItem>,
	/// Indicates if there are more results to fetch.
	pub has_more: bool,
	/// The token for the next page, if any.
	pub next_page: String,
	/// The object type, which is always `vector_store.search_results.page`
	pub object: VectorStoreSearchResultsPageObject,
	pub search_query: Vec<String>,
}
/// The object type, which is always `vector_store.search_results.page`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum VectorStoreSearchResultsPageObject {
	#[serde(rename = "vector_store.search_results.page")]
	VectorStoreSearchResultsPage,
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
#[serde(untagged)]
pub enum WaitType {
	Wait,
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
#[serde(untagged)]
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
#[serde(untagged)]
pub enum WebSearchToolCallType {
	#[serde(rename = "web_search_call")]
	WebSearchCall,
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
pub type AssistantObject = serde_json::Value;
pub type AuditLog = serde_json::Value;
pub type Batch = serde_json::Value;
pub type BatchRequestInput = serde_json::Value;
pub type BatchRequestOutput = serde_json::Value;
pub type ChatCompletionList = serde_json::Value;
pub type ChatCompletionMessageList = serde_json::Value;
pub type ChatCompletionMessageToolCalls = Vec<ChatCompletionMessageToolCall>;
pub type ChatCompletionModalities = Vec<ChatCompletionModalitiesChatCompletionModalities>;
pub type ChatCompletionRequestAssistantMessageContentArray = Vec<ChatCompletionRequestAssistantMessageContentPart>;
pub type ChatCompletionRequestDeveloperMessageContentArray = Vec<ChatCompletionRequestMessageContentPartText>;
pub type ChatCompletionRequestSystemMessageContentArray = Vec<ChatCompletionRequestSystemMessageContentPart>;
pub type ChatCompletionRequestToolMessageContentArray = Vec<ChatCompletionRequestToolMessageContentPart>;
pub type ChatCompletionRequestUserMessageContentArray = Vec<ChatCompletionRequestUserMessageContentPart>;
pub type CostsResult = serde_json::Value;
pub type CreateAssistantRequestModel = serde_json::Value;
pub type CreateChatCompletionFunctionResponse = serde_json::Value;
pub type CreateChatCompletionImageResponse = serde_json::Value;
pub type CreateChatCompletionResponse = serde_json::Value;
pub type CreateChatCompletionStreamResponse = serde_json::Value;
pub type CreateCompletionRequestModel = serde_json::Value;
pub type CreateCompletionRequestPromptArray = Vec<Vec<Vec<i64>>>;
pub type CreateCompletionResponse = serde_json::Value;
pub type CreateEmbeddingRequestInputArray = Vec<Vec<Vec<i64>>>;
pub type CreateEmbeddingRequestModel = serde_json::Value;
pub type CreateFineTuningJobRequestModel = serde_json::Value;
pub type CreateImageEditRequestModel = serde_json::Value;
pub type CreateImageRequestModel = serde_json::Value;
pub type CreateImageVariationRequestModel = serde_json::Value;
pub type CreateMessageRequestContentArray = Vec<CreateMessageRequestContentCreateMessageRequestContent>;
pub type CreateModerationRequestInputArray = Vec<CreateModerationRequestInputCreateModerationRequestInput>;
pub type CreateModerationRequestModel = serde_json::Value;
pub type CreateModerationResponse = serde_json::Value;
pub type CreateResponseInputArray = Vec<InputItem>;
pub type CreateRunRequestModel = serde_json::Value;
pub type CreateRunRequestToolChoice = AssistantsApiToolChoiceOption;
pub type CreateSpeechRequestModel = serde_json::Value;
pub type CreateThreadAndRunRequestModel = serde_json::Value;
pub type CreateThreadAndRunRequestToolChoice = AssistantsApiToolChoiceOption;
pub type CreateTranscriptionRequestModel = serde_json::Value;
pub type CreateTranscriptionResponseJson = serde_json::Value;
pub type CreateTranscriptionResponseStreamEvent = serde_json::Value;
pub type CreateTranscriptionResponseVerboseJson = serde_json::Value;
pub type CreateTranslationRequestModel = serde_json::Value;
pub type DoneEvent = serde_json::Value;
pub type Embedding = serde_json::Value;
pub type ErrorEvent = serde_json::Value;
pub type FineTuneChatRequestInput = serde_json::Value;
pub type FineTuneCompletionRequestInput = serde_json::Value;
pub type FineTunePreferenceRequestInput = serde_json::Value;
pub type FineTuningCheckpointPermission = serde_json::Value;
pub type FineTuningJob = serde_json::Value;
pub type FineTuningJobCheckpoint = serde_json::Value;
pub type FineTuningJobEvent = serde_json::Value;
pub type FunctionParameters = serde_json::Value;
pub type Image = serde_json::Value;
pub type InputMessageContentList = Vec<InputContent>;
pub type Invite = serde_json::Value;
pub type ListAssistantsResponse = serde_json::Value;
pub type MessageDeltaObject = serde_json::Value;
pub type MessageObject = serde_json::Value;
pub type Metadata = HashMap<String, String>;
pub type ModelIds = serde_json::Value;
pub type ModelIdsResponses = serde_json::Value;
pub type ModelIdsShared = serde_json::Value;
pub type ModifyAssistantRequestModel = serde_json::Value;
pub type ParallelToolCalls = bool;
pub type PredictionContentContentArray = Vec<ChatCompletionRequestMessageContentPartText>;
pub type Project = serde_json::Value;
pub type ProjectApiKey = serde_json::Value;
pub type ProjectRateLimit = serde_json::Value;
pub type ProjectServiceAccount = serde_json::Value;
pub type ProjectUser = serde_json::Value;
pub type RealtimeClientEvent = serde_json::Value;
pub type RealtimeClientEventConversationItemCreate = serde_json::Value;
pub type RealtimeClientEventConversationItemDelete = serde_json::Value;
pub type RealtimeClientEventConversationItemRetrieve = serde_json::Value;
pub type RealtimeClientEventConversationItemTruncate = serde_json::Value;
pub type RealtimeClientEventInputAudioBufferAppend = serde_json::Value;
pub type RealtimeClientEventInputAudioBufferClear = serde_json::Value;
pub type RealtimeClientEventInputAudioBufferCommit = serde_json::Value;
pub type RealtimeClientEventResponseCancel = serde_json::Value;
pub type RealtimeClientEventResponseCreate = serde_json::Value;
pub type RealtimeClientEventSessionUpdate = serde_json::Value;
pub type RealtimeClientEventTranscriptionSessionUpdate = serde_json::Value;
pub type RealtimeResponseCreateParamsToolsParameters = serde_json::Value;
pub type RealtimeServerEvent = serde_json::Value;
pub type RealtimeServerEventConversationCreated = serde_json::Value;
pub type RealtimeServerEventConversationItemCreated = serde_json::Value;
pub type RealtimeServerEventConversationItemDeleted = serde_json::Value;
pub type RealtimeServerEventConversationItemInputAudioTranscriptionCompleted = serde_json::Value;
pub type RealtimeServerEventConversationItemInputAudioTranscriptionDelta = serde_json::Value;
pub type RealtimeServerEventConversationItemInputAudioTranscriptionFailed = serde_json::Value;
pub type RealtimeServerEventConversationItemRetrieved = serde_json::Value;
pub type RealtimeServerEventConversationItemTruncated = serde_json::Value;
pub type RealtimeServerEventError = serde_json::Value;
pub type RealtimeServerEventInputAudioBufferCleared = serde_json::Value;
pub type RealtimeServerEventInputAudioBufferCommitted = serde_json::Value;
pub type RealtimeServerEventInputAudioBufferSpeechStarted = serde_json::Value;
pub type RealtimeServerEventInputAudioBufferSpeechStopped = serde_json::Value;
pub type RealtimeServerEventRateLimitsUpdated = serde_json::Value;
pub type RealtimeServerEventResponseAudioDelta = serde_json::Value;
pub type RealtimeServerEventResponseAudioDone = serde_json::Value;
pub type RealtimeServerEventResponseAudioTranscriptDelta = serde_json::Value;
pub type RealtimeServerEventResponseAudioTranscriptDone = serde_json::Value;
pub type RealtimeServerEventResponseContentPartAdded = serde_json::Value;
pub type RealtimeServerEventResponseContentPartDone = serde_json::Value;
pub type RealtimeServerEventResponseCreated = serde_json::Value;
pub type RealtimeServerEventResponseDone = serde_json::Value;
pub type RealtimeServerEventResponseFunctionCallArgumentsDelta = serde_json::Value;
pub type RealtimeServerEventResponseFunctionCallArgumentsDone = serde_json::Value;
pub type RealtimeServerEventResponseOutputItemAdded = serde_json::Value;
pub type RealtimeServerEventResponseOutputItemDone = serde_json::Value;
pub type RealtimeServerEventResponseTextDelta = serde_json::Value;
pub type RealtimeServerEventResponseTextDone = serde_json::Value;
pub type RealtimeServerEventSessionCreated = serde_json::Value;
pub type RealtimeServerEventSessionUpdated = serde_json::Value;
pub type RealtimeServerEventTranscriptionSessionUpdated = serde_json::Value;
pub type RealtimeSessionCreateRequestModalities = serde_json::Value;
pub type RealtimeSessionCreateRequestToolsParameters = serde_json::Value;
pub type RealtimeSessionCreateResponse = serde_json::Value;
pub type RealtimeSessionModalities = serde_json::Value;
pub type RealtimeSessionToolsParameters = serde_json::Value;
pub type RealtimeTranscriptionSessionCreateRequestModalities = serde_json::Value;
pub type RealtimeTranscriptionSessionCreateResponse = serde_json::Value;
pub type ResponseAudioDeltaEvent = serde_json::Value;
pub type ResponseAudioDoneEvent = serde_json::Value;
pub type ResponseAudioTranscriptDeltaEvent = serde_json::Value;
pub type ResponseAudioTranscriptDoneEvent = serde_json::Value;
pub type ResponseCodeInterpreterCallCodeDeltaEvent = serde_json::Value;
pub type ResponseCodeInterpreterCallCodeDoneEvent = serde_json::Value;
pub type ResponseCodeInterpreterCallCompletedEvent = serde_json::Value;
pub type ResponseCodeInterpreterCallInProgressEvent = serde_json::Value;
pub type ResponseCodeInterpreterCallInterpretingEvent = serde_json::Value;
pub type ResponseCompletedEvent = serde_json::Value;
pub type ResponseContentPartAddedEvent = serde_json::Value;
pub type ResponseContentPartDoneEvent = serde_json::Value;
pub type ResponseCreatedEvent = serde_json::Value;
pub type ResponseErrorEvent = serde_json::Value;
pub type ResponseFailedEvent = serde_json::Value;
pub type ResponseFileSearchCallCompletedEvent = serde_json::Value;
pub type ResponseFileSearchCallInProgressEvent = serde_json::Value;
pub type ResponseFileSearchCallSearchingEvent = serde_json::Value;
pub type ResponseFormatJsonSchemaSchema = serde_json::Value;
pub type ResponseFunctionCallArgumentsDeltaEvent = serde_json::Value;
pub type ResponseFunctionCallArgumentsDoneEvent = serde_json::Value;
pub type ResponseInProgressEvent = serde_json::Value;
pub type ResponseIncompleteEvent = serde_json::Value;
pub type ResponseItemList = serde_json::Value;
pub type ResponseModalities = Vec<ResponseModalitiesResponseModalities>;
pub type ResponseModalitiesTextOnly = Vec<ResponseModalitiesTextOnlyResponseModalitiesTextOnly>;
pub type ResponseOutputItemAddedEvent = serde_json::Value;
pub type ResponseOutputItemDoneEvent = serde_json::Value;
pub type ResponseRefusalDeltaEvent = serde_json::Value;
pub type ResponseRefusalDoneEvent = serde_json::Value;
pub type ResponseStreamEvent = serde_json::Value;
pub type ResponseTextAnnotationDeltaEvent = serde_json::Value;
pub type ResponseTextDeltaEvent = serde_json::Value;
pub type ResponseTextDoneEvent = serde_json::Value;
pub type ResponseWebSearchCallCompletedEvent = serde_json::Value;
pub type ResponseWebSearchCallInProgressEvent = serde_json::Value;
pub type ResponseWebSearchCallSearchingEvent = serde_json::Value;
pub type RunObject = serde_json::Value;
pub type RunStepDeltaObject = serde_json::Value;
pub type RunStepDetailsToolCallsFileSearchResultObject = serde_json::Value;
pub type RunStepObject = serde_json::Value;
pub type StopConfigurationArray = Vec<String>;
pub type ThreadObject = serde_json::Value;
pub type TranscriptTextDeltaEvent = serde_json::Value;
pub type TranscriptTextDoneEvent = serde_json::Value;
pub type Upload = serde_json::Value;
pub type UploadPart = serde_json::Value;
pub type UsageAudioSpeechesResult = serde_json::Value;
pub type UsageAudioTranscriptionsResult = serde_json::Value;
pub type UsageCodeInterpreterSessionsResult = serde_json::Value;
pub type UsageCompletionsResult = serde_json::Value;
pub type UsageEmbeddingsResult = serde_json::Value;
pub type UsageImagesResult = serde_json::Value;
pub type UsageModerationsResult = serde_json::Value;
pub type UsageVectorStoresResult = serde_json::Value;
pub type User = serde_json::Value;
pub type VectorStoreFileAttributes = serde_json::Value;
pub type VectorStoreFileBatchObject = serde_json::Value;
pub type VectorStoreFileObject = serde_json::Value;
pub type VectorStoreObject = serde_json::Value;
pub type VectorStoreSearchRequestQueryArray = Vec<String>;
pub type VoiceIdsShared = serde_json::Value;
