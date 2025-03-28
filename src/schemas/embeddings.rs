
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
	CreateEmbeddingRequestInputStringArray(CreateEmbeddingRequestInputStringArray),
	CreateEmbeddingRequestInputIntegerArray(CreateEmbeddingRequestInputIntegerArray),
	CreateEmbeddingRequestInputArrayArray(CreateEmbeddingRequestInputArrayArray),
}
