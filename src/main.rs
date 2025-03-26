mod schemas;

mod r#enum;
mod object;

use std::collections::{BTreeSet, BTreeMap};

use convert_case::{Case, Casing};
use saphyr::Yaml;
use anyhow::{Context, Result, bail};

use r#enum::{Enum, EnumType};
use object::{Object, FieldValue, Field};

#[derive(Debug)]
enum Data {
    Object(Object),
    Enum(Enum)
}

fn parse_enum (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, String>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Parsing as enum: {value:#?}");

    let description = value["description"].as_str();
    
    // First, check that this is/isn't an `anyOf`-type enum
    if let Some(enum_options) = value["anyOf"].as_vec() {
        let mut enum_values = Vec::new();

        for enum_option in enum_options {
            // Check that it's not a foreign struct
            if let Some(referred_struct_raw) = enum_option["$ref"].as_str() {
                let parsed_referred_struct = referred_struct_raw.split("/")
                    .skip(3)
                    .next()
                    .context("Failed to parse the referred struct")?;

                // Add the requested struct recursively
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    parsed_referred_struct,
                    &global_yaml["components"]["schemas"][parsed_referred_struct]
                )
                    .with_context(|| format!("Couldn't parse the object {parsed_referred_struct}"))?;
                println!("Finished parsing {parsed_referred_struct}, continuing with enum {key}");

                // Copy all of the enum variants into this one
                if let Some(Data::Enum(referred_enum)) = schemas.get(parsed_referred_struct) {
                    for value in &referred_enum.values {
                        enum_values.push(value.to_string());
                    }
                } else {
                    panic!("Referred struct is not an enum in an enum type");
                }

                continue;
            }

            // Otherwise, recusively parse the enum's options down
            if let Some(values) = enum_option["enum"].as_vec() {
                for value in values {
                    if let Some(value) = value.as_str() {
                        enum_values.push(value.to_string());
                    }
                }

                continue;
            }

            // Lastly, just convert `type` to `UpperCamel` case
            if let Some(enum_type) = enum_option["type"].as_str() {
                let cased_enum_type = enum_type.to_case(Case::UpperCamel);
                enum_values.push(format!("{}({})",
                    cased_enum_type,
                    match enum_type {
                        "string" => {
                            "String"
                        },
                        "integer" => {
                            "i64"
                        },
                        _ => {
                            panic!("Unsupported type found")
                        }
                    }
                ));
            }
        } 
        

        // Add the enum to the schema
        schemas.insert(
            key.to_string(),
            Data::Enum(Enum {
                name: key.to_string(),
                description: description.map(|s| s.to_string()),
                values: enum_values,
                enum_type: EnumType::Standard,
            })
        );

        return Ok(())
    }
    // Second, check that this is/isn't an `oneOf`-type enum
    if let Some(enum_options) = value["oneOf"].as_vec() {
        let mut enum_values = Vec::new();

        for enum_option in enum_options {
            // Check that it's not a foreign struct
            if let Some(referred_struct_raw) = enum_option["$ref"].as_str() {
                let parsed_referred_struct = referred_struct_raw.split("/")
                    .skip(3)
                    .next()
                    .context("Failed to parse the referred struct")?;

                // Add the requested struct recursively
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    parsed_referred_struct,
                    &global_yaml["components"]["schemas"][parsed_referred_struct]
                )
                    .with_context(|| format!("Couldn't parse the object {parsed_referred_struct}"))?;
                println!("Finished parsing {parsed_referred_struct}, continuing with enum {key}");

                // Add the referred struct to the enum
                match schemas.get(parsed_referred_struct)
                    .with_context(|| format!("Couldn't get the referred struct: {}", parsed_referred_struct))?
                {
                    Data::Object(_) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct));
                    },
                    Data::Enum(_) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct));
                    }
                }

                continue;
            }

            // Otherwise, recusively parse the enum's options down
            if let Some(values) = enum_option["enum"].as_vec() {
                for value in values {
                    if let Some(value) = value.as_str() {
                        enum_values.push(value.to_string());
                    }
                }

                continue;
            }

            // Lastly, just convert `type` to `UpperCamel` case
            if let Some(enum_type) = enum_option["type"].as_str() {
                let cased_enum_type = enum_type.to_case(Case::UpperCamel);
                enum_values.push(format!("{}({})",
                    cased_enum_type,
                    match enum_type {
                        "string" => {
                            "String"
                        },
                        "integer" => {
                            "i64"
                        },
                        _ => {
                            panic!("Unsupported type found")
                        }
                    }
                ));
            }
        }

        // Add the enum to the schema
        schemas.insert(
            key.to_string(),
            Data::Enum(Enum {
                name: key.to_string(),
                description: description.map(|s| s.to_string()),
                values: enum_values,
                enum_type: EnumType::OneOf,
            })
        );
    
        return Ok(())
    }

    let values = value["enum"].as_vec()
        .context("Failed to get enum values")?;
    let mut enum_values = Vec::new();
    for value in values {
        if let Some(value) = value.as_str() {
            enum_values.push(value.to_string());
        }
    }

    // Add the enum to the schema
    schemas.insert(
        key.to_string(),
        Data::Enum(Enum {
            name: key.to_string(),
            description: description.map(|s| s.to_string()),
            values: enum_values,
            enum_type: EnumType::Standard,
        })
    );
    println!("Added enum: {}", key);

    Ok(())
}
fn parse_object (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, String>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Parsing as object: {value:#?}");

    let description = value["description"].as_str();
    let properties = value["properties"].as_hash()
        .context("Failed to get properties")?;
    let mut required = BTreeSet::new();
    if let Some(required_yaml_vec) = value["required"].as_vec() {
        for required_field_yaml in required_yaml_vec {
            if let Some(required_field) = required_field_yaml.as_str() {
                required.insert(required_field);
            }
        }
    }

    let mut object = Object {
        name: key.to_string(),
        description: description.map(|s| s.to_string()),
        properties: BTreeMap::new(),
    };

    for (property_key, property_value) in properties.iter() {
        let property_key = property_key.as_str().context("Failed to get key")?;

        let description = property_value["description"].as_str();
        let field_value = match property_value["type"].as_str() {
            Some("object") => {
                // Recursively add the object as `keyPropertyKey`
                let field_type_key = format!(
                    "{}{}", 
                    key,
                    property_key.to_case(Case::UpperCamel)
                );

                parse_object(
                    global_yaml,
                    schemas,
                    aliases,
                    field_type_key.as_str(),
                    property_value
                )
                    .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
                println!("Finished recursively adding object field {field_type_key}, continuing object {key}");

                FieldValue::ExternalType(field_type_key)
            },
            Some("enum") => {
                FieldValue::ExternalType(property_value["type"].as_str().unwrap().to_string())
            },
            Some("array") => {
                FieldValue::Array(property_value["type"].as_str().unwrap().to_string())
            },
            Some("string") => {
                if property_value["enum"].as_vec().is_some() {
                    let field_type_key = format!(
                        "{}{}", 
                        key,
                        property_key.to_case(Case::UpperCamel)
                    );

                    // Parse the enum
                    parse_enum(
                        global_yaml,
                        schemas,
                        aliases,
                        field_type_key.as_str(),
                        property_value
                    )
                        .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
                    println!("Finished recursively adding enum {field_type_key}, continuing object {key}");
                    
                    FieldValue::ExternalType(field_type_key)
                } else {
                    FieldValue::String
                }
            },
            Some("integer") => {
                FieldValue::Integer
            },
            Some("boolean") => {
                FieldValue::Boolean
            },
            _ => {
                // Check if it's an `anyOf` or `oneOf` enum
                if property_value["anyOf"].as_vec().is_some() ||
                    property_value["oneOf"].as_vec().is_some()
                {
                    let field_type_key = format!(
                        "{}{}", 
                        key,
                        property_key.to_case(Case::UpperCamel)
                    );

                    // Parse the enum
                    parse_enum(
                        global_yaml,
                        schemas,
                        aliases,
                        field_type_key.as_str(),
                        property_value
                    )
                        .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
                    println!("Finished recursively adding `anyOf`/`oneOf` enum {field_type_key}, continuing object {key}");

                    FieldValue::ExternalType(field_type_key)
                } else if let Some(referred_type) = property_value["$ref"].as_str() {
                    let parsed_referred_type = referred_type.split("/")
                        .skip(3)
                        .next()
                        .context("Failed to parse the referred type")?;
                    println!("Referred type: {}", parsed_referred_type);

                    let referred_type_yaml = &global_yaml["components"]["schemas"][parsed_referred_type];
                    println!("Referred type: {:#?}", referred_type_yaml);
                    
                    // Add the requested type recursively
                    parse(
                        global_yaml,
                        schemas,
                        aliases,
                        parsed_referred_type,
                        &referred_type_yaml,
                    )
                        .with_context(|| format!("Couldn't parse the object {parsed_referred_type}"))?;
                    println!("Finished recusively adding external type {parsed_referred_type}, continuing object {key}");

                    //println!("Schemas: {:#?}", schemas);
                    //println!("Aliases: {:#?}", aliases);

                    match schemas.get(parsed_referred_type) {
                        Some(Data::Object(_)) | Some(Data::Enum(_)) => {
                            FieldValue::ExternalType(parsed_referred_type.to_string())
                        },
                        None => {
                            match aliases.get(parsed_referred_type) {
                                Some(alias) => {
                                    FieldValue::ExternalType(alias.to_string())
                                },
                                None => {
                                    bail!("Couldn't get the referred type: {}", parsed_referred_type);
                                }
                            }
                        }
                    }
                } else {
                    println!("Unknown type `{:#?}`!\nErroneous Value: {:#?}", property_value["type"], property_value);
                    panic!("No type found");
                }
            }
        };

        object.properties.insert(property_key.to_string(), Field {
            description: description.map(|s| s.to_string()),
            value: field_value,
            required: required.contains(&property_key)
        });
    }

    // Add the object to the schema
    schemas.insert(
        key.to_string(),
        Data::Object(object)
    );
    println!("Added object: {}", key);

    Ok(())
} 
fn parse (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, String>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    let allowed = vec!(
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
    );
    if !allowed.contains(&key) {
        println!("Skipping {key}");
        return Ok(());
    }

    println!("Key: {}", key);
    println!("Value: {:#?}", value);

    // Check if the schema is an enum with `anyOf` or `oneOf`
    if let Some(_) = value["anyOf"].as_vec() {
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
    if let Some(_) = value["oneOf"].as_vec() {
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

    // Check for standard structs or enums
    match value["type"].as_str() {
        Some("object") => {
            // Check that it's not secretly a JSON value
            if let Some(additional_properties) = value["additionalProperties"].as_bool() {
                if additional_properties {
                    aliases.insert(key.to_string(), "serde_json::Value".to_string());
                }

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
                panic!("Unsupported type found");
            }
        },
        Some(&_) => {
            panic!("Unsupported type found");
        },
        None => {
            panic!("No type found");
        }
    }

    Ok(())
}
fn main() -> Result<()>{
    let openapi_yaml_raw = include_str!("../assets/openapi.yaml");
    let docs = Yaml::load_from_str(&openapi_yaml_raw)
        .context("Failed to load OpenAPI YAML")?;

    let schemas_yaml = docs[0]["components"]["schemas"]
        .as_hash()
        .context("Failed to get schemas")?;

    let mut schemas: BTreeMap<String, Data> = BTreeMap::new();
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();

    for (key, value) in schemas_yaml.iter() {
        let key = key.as_str().context("Failed to get key")?;

        parse(
            &docs[0],

            &mut schemas,
            &mut aliases,

            key,
            value
        )
            .context("Failed to parse the schema")?;
    }

    // Print the schema and alias Rust types
    let mut rust_schema_body = String::new();
    for ( key, value ) in schemas.iter() {
        let stringified = match value {
            Data::Object(object) => {
                format!("{}", object)
            },
            Data::Enum(r#enum) => {
                format!("{}", r#enum)
            }
        };
        println!("{key}:\n{stringified}\n\n");
        rust_schema_body += &stringified
            .replace("(/docs", "(https://platform.openai.com/docs");
        rust_schema_body += "\n";
    }
    for ( key, value ) in aliases.iter() {
        let stringfied = format!("pub type {} = {};\n", key, value);
        println!("{stringfied}");
        rust_schema_body += &stringfied;
    }

    // Write the useful snippets to files
    std::fs::write("assets/openapi.txt", format!("{:#?}", docs))
        .context("Failed to write OpenAPI YAML to file")?;
    std::fs::write("assets/openapi-schemas.txt", format!("{:#?}", schemas))
        .context("Failed to write OpenAPI schema text to file")?;
    std::fs::write("src/schemas.rs", &format!(
        "use serde::{{Serialize, Deserialize}};\n\n{}",
        rust_schema_body
    ))
        .context("Failed to write OpenAPI Rust schemas to file")?;

    Ok(())
}
