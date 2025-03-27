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
#[derive(Debug)]
struct Alias {
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

fn parse_enum (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Parsing as enum: {value:#?}");

    let description = value["description"].as_str();
    
    // First, check that this is/isn't an `anyOf`-type enum
    if let Some(enum_options) = value["anyOf"].as_vec() {
        let mut enum_values = Vec::new();

        for enum_option in enum_options.iter()
            .filter(|enum_option| enum_option["type"].as_str().is_none())
        {
            // Check that it's not a foreign struct
            if let Some(referred_struct_raw) = enum_option["$ref"].as_str() {
                let parsed_referred_struct = referred_struct_raw.split("/")
                    .skip(3)
                    .next()
                    .context("Failed to parse the referred struct")?;

                // Add the requested struct recursively
                println!("Need to recurse for `anyOf` enum: {parsed_referred_struct}");
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    parsed_referred_struct,
                    &global_yaml["components"]["schemas"][parsed_referred_struct]
                )
                    .with_context(|| format!("Couldn't parse the object {parsed_referred_struct}"))?;
                println!("Finished parsing {parsed_referred_struct}, continuing with enum {key}");

                // Copy all of the enum variants into this one, but first check 
                //  that it wasn't turned into an alias.
                if let Some(alias) = aliases.get(parsed_referred_struct) {
                    println!("Casting enum `{}` to `serde_json::Value` because of child enum `{}`", key, alias);

                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string()
                    });

                    return Ok(())
                }
                
                // If not, add the referred struct's enum values to this one
                match schemas.get(parsed_referred_struct) {
                    Some(Data::Enum(referred_enum)) => {
                        for value in &referred_enum.values {
                            enum_values.push(value.to_string());
                        }
                    },
                    Some(Data::Object(referred_obj)) => {
                        // Add the referred struct as a variant
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct));
                    }
                    _ => {
                        bail!("Referred struct is not an enum in an enum type");
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
        }

        // Lastly, ensure we have no custom types, otherwise cast to `serde_json::Value`
        for enum_option in enum_options.iter()
            .filter(|enum_option| enum_option["type"].as_str().is_some())
        {
            if let Some(enum_type) = enum_option["type"].as_str() {
                println!("Casting enum `{}` to `serde_json::Value` because of `{}`", key, enum_type);

                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: "serde_json::Value".to_string()
                });

                return Ok(())
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
                println!("Need to recurse for `oneOf` enum: {parsed_referred_struct}");
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
                match schemas.get(parsed_referred_struct) {
                    Some(Data::Object(_)) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct));
                    },
                    Some(Data::Enum(_)) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct));
                    },
                    None => {
                        // Check if it's an alias
                        match aliases.get(parsed_referred_struct) {
                            Some(alias) => {
                                enum_values.push(format!("{}({})", parsed_referred_struct, alias.name));
                            },
                            None => {
                                println!("Referred type doesn't exist: {parsed_referred_struct}");
                                bail!("Referred type is not an object in an enum type");
                            }
                        }
                    }
                }

                continue;
            }

            // Otherwise, recusively parse the enum's options down
            if let Some(values) = enum_option["enum"].as_vec() {
                for value in values {
                    if let Some(value) = value.as_str() {
                        // Fix the casing of `value`
                        let value = value.to_case(Case::UpperCamel);

                        enum_values.push(format!("{value}(String)"));
                    }
                }

                continue;
            }

            // Lastly, just convert `type` to `UpperCamel` case
            if let Some(enum_type) = enum_option["type"].as_str() {
                let cased_enum_type = enum_type.to_case(Case::UpperCamel);
                if enum_type == "array" {
                    // Add the array as an alias
                    let array_field_value = parse_array(
                        global_yaml,
                        schemas,
                        aliases,
                        key,
                        key,
                        &enum_option
                    )
                        .with_context(|| format!("Couldn't parse the array {key}"))?;

                    let vector_alias_name = format!("{}Array", key);

                    aliases.insert(vector_alias_name.clone(), Alias {
                        name: vector_alias_name.clone(),
                        r#type: format!("{}", array_field_value)
                    });

                    enum_values.push(format!("{}({})",
                        cased_enum_type,
                        vector_alias_name
                    ));

                    continue;
                }

                enum_values.push(format!("{}({})",
                    cased_enum_type,
                    match enum_type {
                        "string" => {
                            "String"
                        },
                        "integer" => {
                            "i64"
                        },
                        "boolean" => {
                            "bool"
                        },
                        "number" => {
                            "f64"
                        },
                        "object" => {
                            "serde_json::Value"
                        },
                        _ => {
                            println!("Unknown type `{:#?}`!\nErroneous Value: {:#?}", enum_type, enum_option);
                            bail!("Unsupported type found")
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
fn parse_array (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,

    key: &str,
    property_key: &str,
    property_value: &Yaml
) -> Result<FieldValue> {
    // First, check if the array type is external
    if let Some(referred_type) = property_value["items"]["$ref"].as_str() {
        let parsed_referred_type = referred_type.split("/")
            .skip(3)
            .next()
            .context("Failed to parse the referred type")?;
        let referred_type_yaml = &global_yaml["components"]["schemas"][parsed_referred_type];
        println!("Referred type: {} - {:#?}", parsed_referred_type, referred_type_yaml);
        
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
                return Ok(FieldValue::Array(parsed_referred_type.to_string()));
            },
            None => {
                match aliases.get(parsed_referred_type) {
                    Some(alias) => {
                        return Ok(FieldValue::Array(alias.name.clone()));
                    },
                    None => {
                        bail!("Couldn't get the referred type: {}", parsed_referred_type);
                    }
                }
            }
        }
    }

    // Otherwise, check if it's an array of objects
    if let Some(_) = property_value["items"]["properties"].as_hash() {
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
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
        println!("Finished recursively adding object field {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key))
    }

    // Otherwise, check if it's an array of enums
    if let Some(_) = property_value["items"]["enum"].as_vec() {
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
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
        println!("Finished recursively adding enum {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key));
    } else if let Some(_) = property_value["items"]["oneOf"].as_vec() {
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
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
        println!("Finished recursively adding `oneOf` enum {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key));
    } else if let Some(_) = property_value["items"]["allOf"].as_vec() {
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
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
        println!("Finished recursively adding `allOf` object {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key));
    }

    let result = match property_value["items"]["type"].as_str() {
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
                &property_value["items"]
            )
                .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
            println!("Finished recursively adding object field {field_type_key}, continuing object {key}");

            FieldValue::Array(field_type_key)
        },
        Some("string") => {
            FieldValue::Array("String".to_string())
        },
        Some("integer") => {
            FieldValue::Array("i64".to_string())
        },
        Some("boolean") => {
            FieldValue::Array("bool".to_string())
        },
        Some("number") => {
            FieldValue::Array("f64".to_string())
        },
        Some("array") => {
            // Nested arrays
            let array_field_value = parse_array(
                global_yaml,
                schemas,
                aliases,
                key,
                property_key,
                &property_value["items"]
            )
                .with_context(|| format!("Couldn't parse the array {key}"))?;

            FieldValue::Array(format!("Vec<{}>", array_field_value))
        },
        _ => {
            println!("Unknown array type `{:#?}`!\nErroneous Value: {:#?}", property_value["items"]["type"], property_value["items"]);
            bail!("Unsupported array type found");
        }
    };

    Ok(result)
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
fn parse (
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

    // Check if it's an `allOf` object
    if let Some(_) = value["allOf"].as_vec() {
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
fn main() -> Result<()>{
    let openapi_yaml_raw = include_str!("../assets/openapi.yaml");
    let docs = Yaml::load_from_str(&openapi_yaml_raw)
        .context("Failed to load OpenAPI YAML")?;

    let schemas_yaml = docs[0]["components"]["schemas"]
        .as_hash()
        .context("Failed to get schemas")?;

    let mut schemas: BTreeMap<String, Data> = BTreeMap::new();
    let mut aliases: BTreeMap<String, Alias> = BTreeMap::new();

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
    for ( _key, value ) in aliases.iter() {
        let stringfied = format!("{value}");
        println!("{stringfied}");
        rust_schema_body += &stringfied;
    }

    // Write the useful snippets to files
    std::fs::write("assets/openapi.txt", format!("{:#?}", docs))
        .context("Failed to write OpenAPI YAML to file")?;
    std::fs::write("assets/openapi-schemas.txt", format!("{:#?}", schemas))
        .context("Failed to write OpenAPI schema text to file")?;
    std::fs::write("src/schemas.rs", &format!(
        "use std::collections::HashMap;\nuse serde::{{Serialize, Deserialize}};\n\n{}",
        rust_schema_body
    ))
        .context("Failed to write OpenAPI Rust schemas to file")?;

    Ok(())
}
