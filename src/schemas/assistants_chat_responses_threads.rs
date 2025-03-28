
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// **o-series models only** 
/// 
/// Constrains effort on reasoning for 
/// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
/// Currently supported values are `low`, `medium`, and `high`. Reducing
/// reasoning effort can result in faster responses and fewer tokens used
/// on reasoning in a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
	Low,
	Medium,
	High,
}
