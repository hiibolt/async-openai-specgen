use super::chat::ModelResponseProperties;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
/// High level guidance for the amount of context window space to use for the 
/// search. One of `low`, `medium`, or `high`. `medium` is the default.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchContextSize {
	Low,
	Medium,
	High,
}
/// Approximate location parameters for the search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
