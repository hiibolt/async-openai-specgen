mod enums;
mod objects;
mod arrays;

use super::types::enums::Enum;
use super::types::objects::Object;
use enums::parse_enum;
use objects::parse_object;
use arrays::parse_array;

use std::collections::BTreeMap;

use saphyr::Yaml;
use anyhow::{Context, Result, bail};

#[derive(Debug)]
pub enum Data {
    Object(Object),
    Enum(Enum)
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alias {
    /// The name of the alias
    pub name: String,
    /// The type of the alias
    pub r#type: String,
}
impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &format!("pub type {} = {};\n", self.name, self.r#type))
    }
}

pub fn parse (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    /*
    let allowed = vec!(
        "Tool",
        "ResponseProperties",
        "ModelIdsResponses",
        "ModelIdsShared",
        "Reasoning",
        "ReasoningEffort",
        "ToolChoiceOptions",
        "ToolChoiceTypes",
        "ToolChoiceFunction",
        "TextResponseFormatConfiguration",
        "ResponseFormatText",
        "TextResponseFormatJsonSchema",
        "ResponseFormatJsonSchemaSchema",
        "ResponseFormatJsonObject",
        "FileSearchTool",
        "ComparisonFilter",
        "CompoundFilter",
        "FunctionTool",
        "ComputerTool",
        "WebSearchTool",
        "WebSearchLocation",
        "WebSearchContextSize",
        "Metadata",
        "ChatCompletionMessageToolCalls",
        "ChatCompletionMessageToolCall",
        "CreateRunRequest",
        "AssistantsApiToolChoiceOption",
        "AssistantsNamedToolChoice",
        "AssistantSupportedModels",
        "ReasoningEffort",
        "CreateMessageRequest",
        "MessageContentImageFileObject",
        "MessageContentImageUrlObject",
        "MessageRequestContentTextObject",
        "AssistantToolsCode",
        "AssistantToolsFileSearchTypeOnly",
        "Metadata",
        "AssistantToolsFileSearch",
        "FileSearchRankingOptions",
        "FileSearchRanker",
        "AssistantToolsFunction",
        "FunctionObject",
        "FunctionParameters",
        "TruncationObject",
        "ParallelToolCalls",
        "AssistantsApiResponseFormatOption",
        "ResponseFormatText",
        "ResponseFormatJsonObject",
        "ResponseFormatJsonSchema",
        "ResponseFormatJsonSchemaSchema"
    );
    if !allowed.contains(&key) {
        println!("Skipping {key}");
        return Ok(());
    } */

    println!("Key: {}", key);
    println!("Value: {:#?}", value);

    // Check if the schema is an enum with `anyOf`, `oneOf`
    if value["anyOf"].as_vec().is_some() || value["oneOf"].as_vec().is_some() {
        parse_enum(
            global_yaml,
            schemas,
            aliases,
            key,
            value
        )
            .with_context(|| format!("Couldn't parse the enum {key}"))?;

        return Ok(())
    }

    // Check if it's an `allOf` object
    if value["allOf"].as_vec().is_some() {
        parse_object(
            &global_yaml,
            schemas,
            aliases,
            key,
            value
        )
            .with_context(|| format!("Couldn't parse the object {key}"))?;
        println!("Finished parsing {key} (allOf)");

        return Ok(())
    }

    // Check for standard structs or enums
    match value["type"].as_str() {
        Some("object") => {
            // Check that it's not secretly a JSON value or a HashMap
            if let Some(additional_properties) = value["additionalProperties"].as_bool() {
                if additional_properties {
                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string()
                    });

                    return Ok(())
                }
            } else if let Some(r#type) = value["additionalProperties"]["type"].as_str() {
                // If it's a `string`, cast to a `HashMap<String, String>`
                if r#type == "string" {
                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "HashMap<String, String>".to_string()
                    });

                    return Ok(())
                }
            } else if let Some(x_oai_type_label) = value["x-oaiTypeLabel"].as_str() {
                // If it's a `map`, cast to a `serde_json::Value`
                if x_oai_type_label == "map" {
                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string()
                    });

                    return Ok(())
                }
            } else if value["x-oaiMeta"].as_hash().is_some() && value["properties"].as_hash().is_none() {
                // If it's a `meta`, cast to a `serde_json::Value`
                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: "serde_json::Value".to_string()
                });

                return Ok(())
            } 

            parse_object(
                &global_yaml,
                schemas,
                aliases,
                key,
                value
            )
                .with_context(|| format!("Couldn't parse the object {key}"))?;
            println!("Finished parsing {key} (object)");
        },
        Some("string") => {
            if value["enum"].as_vec().is_some() {
                parse_enum(
                    &global_yaml,
                    schemas,
                    aliases,
                    key,
                    value
                )
                    .with_context(|| format!("Couldn't parse the enum {key}"))?;
                println!("Finished parsing {key} (enum)");
            } else {
                bail!("Unsupported type found");
            }
        },
        Some("array") => {
            // Create an alias for the array
            let array_field_value = parse_array(
                &global_yaml,
                schemas,
                aliases,
                key,
                key,
                value
            )
                .with_context(|| format!("Couldn't parse the array {key}"))?;

            aliases.insert(key.to_string(), Alias {
                name: key.to_string(),
                r#type: format!("{}", array_field_value)
            });

            println!("Finished parsing {key} (array)");
        },
        Some("boolean") => {
            // Create an alias for the boolean
            aliases.insert(key.to_string(), Alias {
                name: key.to_string(),
                r#type: "bool".to_string()
            });

            println!("Finished parsing {key} (boolean)");
        }
        Some(other) => {
            println!("Unknown type `{:#?}`!\nErroneous Value: {:#?}", other, value);
            bail!("Unsupported type found");
        },
        None => {
            let assured_objects = vec!(
                "ImagesResponse",
                "OpenAIFile",
                "ListMessagesResponse",
                "Model",
                "ListRunStepsResponse",
                "ListThreadsResponse",
                "ListVectorStoreFilesResponse",
                "ListVectorStoresResponse"
            );

            if assured_objects.contains(&key) {
                println!("Bypassing struct `{key}`.");
                parse_object(
                    &global_yaml,
                    schemas,
                    aliases,
                    key,
                    value
                )
                    .with_context(|| format!("Couldn't parse the object {key}"))?;
                println!("Finished parsing {key} (object)");

                return Ok(())
            }

            // If we still aren't sure, but there's an `items` field, we 
            //  can assume it's an array
            if let Some(_) = value["items"].as_hash() {
                // Create an alias for the array
                let array_field_value = parse_array(
                    &global_yaml,
                    schemas,
                    aliases,
                    key,
                    key,
                    value
                )
                    .with_context(|| format!("Couldn't parse the array {key}"))?;

                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: format!("{}", array_field_value)
                });

                println!("Finished parsing {key} (array)");

                return Ok(())
            }

            println!("No type!\nErroneous Value: {:#?}", value);
            bail!("No type found");
        }
    }

    Ok(())
}