mod enums;
mod objects;
mod arrays;

use super::data::enums::Enum;
use super::data::objects::Object;
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
    /// The description of the alias
    pub description: Option<String>
}
impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut body = String::new();

        // Write the alias description
        if let Some(ref description) = self.description {
            for line in description.lines() {
                body.push_str(&format!("/// {}\n", line));
            }
        }

        // Write the alias name
        body += &format!("pub type {} = {};\n", self.name, self.r#type);

        write!(f, "{}", &body)
    }
}

pub fn parse (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,
    wanted_by: String,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Key: {}", key);
    println!("Value: {:#?}", value);

    // Check if the key already exists in the schemas or aliases
    if let Some(data) = schemas.get_mut(key) {
        println!("Data already exists for key `{key}`: {data:#?}");

        // Add the wanted_by field to the schema or enum
        match data {
            Data::Object(object) => {
                object.wanted_by.insert(wanted_by);
            },
            Data::Enum(r#enum) => {
                r#enum.wanted_by.insert(wanted_by);
            }
        }

        return Ok(())
    }

    // Check if the schema is an enum with `anyOf`, `oneOf`
    if value["anyOf"].as_vec().is_some() || value["oneOf"].as_vec().is_some() {
        parse_enum(
            global_yaml,
            schemas,
            aliases,
            wanted_by,
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
            wanted_by,
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
                    let description = value["description"].as_str()
                        .and_then(|st| Some(st.to_string()));

                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string(),
                        description,
                    });

                    return Ok(())
                }
            } else if let Some(r#type) = value["additionalProperties"]["type"].as_str() {
                // If it's a `string`, cast to a `HashMap<String, String>`
                if r#type == "string" {
                    let description = value["description"].as_str()
                        .and_then(|st| Some(st.to_string()));

                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "HashMap<String, String>".to_string(),
                        description,
                    });

                    return Ok(())
                }
            } else if let Some(x_oai_type_label) = value["x-oaiTypeLabel"].as_str() {
                // If it's a `map`, cast to a `serde_json::Value`
                if x_oai_type_label == "map" && value["properties"].as_hash().is_none() {
                    let description = value["description"].as_str()
                        .and_then(|st| Some(st.to_string()));

                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string(),
                        description,
                    });

                    return Ok(())
                }
            } else if value["x-oaiMeta"].as_hash().is_some() && value["properties"].as_hash().is_none() {
                let description = value["description"].as_str()
                    .and_then(|st| Some(st.to_string()));

                // If it's a `meta`, cast to a `serde_json::Value`
                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: "serde_json::Value".to_string(),
                    description,
                });

                return Ok(())
            } 

            parse_object(
                &global_yaml,
                schemas,
                aliases,
                wanted_by,
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
                    wanted_by,
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
                wanted_by,
                key,
                key,
                value
            )
                .with_context(|| format!("Couldn't parse the array {key}"))?;
            let description = value["description"].as_str()
                .and_then(|st| Some(st.to_string()));

            aliases.insert(key.to_string(), Alias {
                name: key.to_string(),
                r#type: format!("{}", array_field_value),
                description,
            });

            println!("Finished parsing {key} (array)");
        },
        Some("boolean") => {
            // Create an alias for the boolean
            let description = value["description"].as_str()
                .and_then(|st| Some(st.to_string()));

            aliases.insert(key.to_string(), Alias {
                name: key.to_string(),
                r#type: "bool".to_string(),
                description,
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
                    wanted_by,
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
                    wanted_by,
                    key,
                    key,
                    value
                )
                    .with_context(|| format!("Couldn't parse the array {key}"))?;
                let description = value["description"].as_str()
                    .and_then(|st| Some(st.to_string()));

                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: format!("{}", array_field_value),
                    description,
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