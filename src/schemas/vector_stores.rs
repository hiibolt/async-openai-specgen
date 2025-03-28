use super::responses_vector_stores::ComparisonFilter;
use super::responses_vector_stores::CompoundFilter;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AutoChunkingStrategyRequestParam {
	/// Always `auto`.
	pub r#type: AutoChunkingStrategyRequestParamType,
}
/// Always `auto`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AutoChunkingStrategyRequestParamType {
	Auto,
}
/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum ChunkingStrategyRequestParam {
	AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
	StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateVectorStoreRequestChunkingStrategy {
	AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
	StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
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
pub enum StaticChunkingStrategyRequestParamType {
	Static,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateVectorStoreFileAttributesRequest {
	pub attributes: VectorStoreFileAttributes,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
pub enum VectorStoreExpirationAfterAnchor {
	#[serde(rename = "last_active_at")]
	LastActiveAt,
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
	VectorStoreSearchRequestQueryStringArray(VectorStoreSearchRequestQueryStringArray),
}
