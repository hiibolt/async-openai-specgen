use super::{
    Data, Alias, parse,
    super::types::objects::{Object, FieldValue, Field},
    enums::parse_enum,
    arrays::parse_array
};

use std::collections::{BTreeSet, BTreeMap};

use convert_case::{Case, Casing};
use saphyr::Yaml;
use anyhow::{Context, Result, bail};

pub(super) fn parse_object (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Parsing as object: {value:#?}");

    let description = value["description"].as_str();

    // Before anything, check if it's secretly a JSON value
    if let Some(additional_properties) = value["additionalProperties"].as_bool() {
        if additional_properties {
            aliases.insert(key.to_string(), Alias {
                name: key.to_string(),
                r#type: "serde_json::Value".to_string()
            });

            return Ok(())
        }
    }

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

    if let Some(sub_objects) = value["allOf"].as_vec() {
        println!("Which is an `allOf` object");
        for sub_object in sub_objects {
            // Get the referred type, and steal its properties
            if let Some(referred_type_raw) = sub_object["$ref"].as_str() {
                let referred_type = referred_type_raw.split("/")
                    .skip(3)
                    .next()
                    .context("Failed to parse the referred type")?;
                let referred_type_yaml = &global_yaml["components"]["schemas"][referred_type];

                // Add the requested type recursively
                println!("Need to recurse for `allOf` object: {referred_type}");
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    referred_type,
                    &referred_type_yaml,
                )
                    .with_context(|| format!("Couldn't parse the object {referred_type}"))?;
                println!("Finished recusively adding object {referred_type}, continuing object {key}");

                match schemas.get(referred_type) {
                    Some(Data::Object(referred_object)) => {
                        for (property_key, field) in &referred_object.properties {
                            object.properties.insert(property_key.to_string(), field.clone());
                        }
                    },
                    Some(Data::Enum(_)) => {
                        // Add this struct as an alias instead
                        aliases.insert(key.to_string(), Alias {
                            name: key.to_string(),
                            r#type: referred_type.to_string()
                        });

                        return Ok(());
                    }
                    _ => {
                        println!("Referred type {} does not exist in an object type", referred_type);
                        bail!("Referred type does not exist in an object type");
                    }
                }
            } else if let Some(_) = sub_object["type"].as_str() {
                process_properties(
                    global_yaml,
                    schemas,
                    aliases,
                    &mut object,
                    key,
                    sub_object,
                    &mut required,
                ).with_context(|| format!("Couldn't process properties for {} on sub_object {:#?}", key, sub_object))?;
            }
        }
    } else {
        process_properties(
            global_yaml,
            schemas,
            aliases,
            &mut object,
            key,
            value,
            &mut required,
        ).with_context(|| format!("Couldn't process properties for {}", key))?;
    }

    
    // Add the object to the schema
    schemas.insert(
        key.to_string(),
        Data::Object(object)
    );
    println!("Added object: {}", key);

    Ok(())
} 
fn process_properties (
    global_yaml: &Yaml,
    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,

    object: &mut Object,
    key: &str,
    value: &Yaml,
    required: &mut BTreeSet<&str>,
) -> Result<()> {
    println!("About to process properties for {key}: {value:#?}");

    // Intentionally empty objects
    if let Some(additional_properties) = value["additionalProperties"].as_bool() {
        if !additional_properties && value["properties"].as_hash().is_none() {
            println!("Intentionally skipping object with no properties");

            return Ok(())
        }
    }

    // Check if it's a `oneOf` object
    if let Some(_) = value["oneOf"].as_vec() {
        let field_type_key = format!(
            "{}{}", 
            key,
            key.to_case(Case::UpperCamel)
        );

        // Parse the enum
        println!("Property is a `oneOf` object: {field_type_key}, parsing as enum");
        parse_enum(
            global_yaml,
            schemas,
            aliases,
            field_type_key.as_str(),
            value
        )
            .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
        println!("Finished recursively adding `oneOf` enum {field_type_key}, continuing object {key}");

        object.properties.insert(key.to_string(), Field {
            description: None,
            value: FieldValue::ExternalType(field_type_key),
            required: required.contains(&key)
        });

        return Ok(())
    }

    let properties = if let Some(properties) = value["properties"].as_hash() {
        properties
    } else {
        // Add it as a `serde_json::Value` object
        aliases.insert(key.to_string(), Alias {
            name: key.to_string(),
            r#type: "serde_json::Value".to_string()
        });

        return Ok(())
    };
    for (property_key, property_value) in properties.iter() {
        let property_key = property_key.as_str().context("Failed to get key")?;

        println!("Processing property on {key}: {property_key} - {property_value:#?}");

        let description = property_value["description"].as_str();
        let field_value = match property_value["type"].as_str() {
            Some("object") => {
                // Make sure it's not secretly a JSON value or a HashMap
                if let Some(additional_properties) = property_value["additionalProperties"].as_bool() {
                    if additional_properties {
                        FieldValue::ExternalType("serde_json::Value".to_string())
                    } else {
                        FieldValue::ExternalType("HashMap<String, String>".to_string())
                    }
                } else if let Some(r#type) = property_value["additionalProperties"]["type"].as_str() {
                    if r#type == "string" {
                        FieldValue::ExternalType("HashMap<String, String>".to_string())
                    } else {
                        FieldValue::ExternalType("serde_json::Value".to_string())
                    }
                } else if let Some(x_oai_type_label) = property_value["x-oaiTypeLabel"].as_str() {
                    if x_oai_type_label == "map" {
                        FieldValue::ExternalType("serde_json::Value".to_string())
                    } else {
                        FieldValue::ExternalType("HashMap<String, String>".to_string())
                    }
                } else if property_value["x-oaiMeta"].as_hash().is_some() && property_value["properties"].as_hash().is_none() {
                    FieldValue::ExternalType("serde_json::Value".to_string())
                } else {
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
                }
            },
            Some("enum") => {
                FieldValue::ExternalType(property_value["type"].as_str().unwrap().to_string())
            },
            Some("array") => {
                parse_array(
                    global_yaml,
                    schemas,
                    aliases,
                    key,
                    property_key,
                    property_value
                )
                    .with_context(|| format!("Couldn't parse the array {key}"))?
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
            Some("number") => {
                FieldValue::ExternalType("f64".to_string())
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
                } else if let Some(_) = property_value["allOf"].as_vec() {
                    let field_type_key = format!(
                        "{}{}", 
                        key,
                        property_key.to_case(Case::UpperCamel)
                    );

                    // Parse the object
                    parse_object(
                        global_yaml,
                        schemas,
                        aliases,
                        field_type_key.as_str(),
                        property_value
                    )
                        .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
                    println!("Finished recursively adding `allOf` object {field_type_key}, continuing object {key}");

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

                    match schemas.get(parsed_referred_type) {
                        Some(Data::Object(_)) | Some(Data::Enum(_)) => {
                            FieldValue::ExternalType(parsed_referred_type.to_string())
                        },
                        None => {
                            match aliases.get(parsed_referred_type) {
                                Some(alias) => {
                                    FieldValue::ExternalType(alias.name.clone())
                                },
                                None => {
                                    bail!("Couldn't get the referred type: {}", parsed_referred_type);
                                }
                            }
                        }
                    }
                } else {
                    // If it has an `items` key, it's an array
                    if let Some(_) = property_value["items"].as_hash() {
                        println!("Assuming property is an array of objects: {key}{property_key}");

                        let field_type_key = format!(
                            "{}{}", 
                            key,
                            property_key.to_case(Case::UpperCamel)
                        );

                        // Recursively add the object as `keyPropertyKey`
                        parse_object(
                            global_yaml,
                            schemas,
                            aliases,
                            field_type_key.as_str(),
                            property_value
                        )
                            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
                        println!("Finished recursively adding object field {field_type_key}, continuing object {key}");

                        FieldValue::Array(field_type_key)
                    } else {
                        println!("Unknown type `{:#?}`!\nErroneous Value: {:#?}", property_value["type"], property_value);
                        bail!("No type found");
                    }

                }
            }
        };

        object.properties.insert(property_key.to_string(), Field {
            description: description.map(|s| s.to_string()),
            value: field_value,
            required: required.contains(&property_key)
        });
    }

    Ok(())
}
