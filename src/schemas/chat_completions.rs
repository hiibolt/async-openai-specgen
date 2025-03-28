
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Options for streaming response. Only set this when you set `stream: true`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
/// Up to 4 sequences where the API will stop generating further tokens. The
/// returned text will not contain the stop sequence.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum StopConfiguration {
	String(String),
	StopConfigurationStringArray(StopConfigurationStringArray),
}
