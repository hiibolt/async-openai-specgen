
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
/// Combine multiple filters using `and` or `or`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompoundFilter {
	/// Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`.
	pub filters: Vec<CompoundFilterItems>,
	/// Type of operation: `and` or `or`.
	pub r#type: CompoundFilterType,
}
