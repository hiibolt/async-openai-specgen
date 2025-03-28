use super::{
    Data, Alias, parse,

    super::data::enums::{Enum, EnumType},
    arrays::parse_array
};

use std::collections::{BTreeMap, BTreeSet};

use convert_case::{Case, Casing};
use saphyr::Yaml;
use anyhow::{Context, Result, bail};

fn reserialize_yaml (
    indentation_level: usize,
    yaml: &Yaml
) -> Result<String> {
    let mut yaml_string = String::new();
    let indentation = "\t".repeat(indentation_level);

    match yaml {
        Yaml::Hash(hash) => {
            // Recursively serialize the hash
            yaml_string.push_str(&format!("{}{{\n", indentation));
            for (key, value) in hash.iter() {
                let key = key.as_str().unwrap_or("unknown");

                if key == "$ref" {
                    if let Some(referred_type) = value.as_str() {
                        let referred_type = referred_type.split("/")
                            .skip(3)
                            .next()
                            .context("Failed to parse the referred type")?;
                        yaml_string.push_str(&format!("\t{},\n", referred_type));

                        continue;
                    }
                }

                yaml_string.push_str(&format!("\t{}{}:\n{},\n", indentation, key, reserialize_yaml(indentation_level + 1, value)?));
            }
            yaml_string.push_str(&format!("{}}}", indentation));
        },
        Yaml::Array(array) => {
            // Recursively serialize the array
            yaml_string.push_str(&format!("{}[\n", indentation));
            for value in array.iter() {
                yaml_string.push_str(&format!("\t{}{},\n", indentation, reserialize_yaml(indentation_level + 1, value)?));
            }
            yaml_string.push_str(&format!("{}]", indentation));
        },
        Yaml::Boolean(boolean) => {
            yaml_string.push_str(&format!("{}{}", indentation, boolean));
        },
        Yaml::Integer(integer) => {
            yaml_string.push_str(&format!("{}{}", indentation, integer));
        },
        Yaml::Real(real) => {
            yaml_string.push_str(&format!("{}{}", indentation, real));
        },
        Yaml::String(string) => {
            yaml_string.push_str(&format!("{}\"{}\"", indentation, string));
        },
        Yaml::Null => {
            yaml_string.push_str(&format!("{}null", indentation));
        },
        _ => {
            bail!("Unsupported YAML type: {yaml:#?}");
        }
    }

    Ok(yaml_string)
}
pub(super) fn parse_enum (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,
    wanted_by: String,

    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Parsing as enum: {value:#?}");

    let description = value["description"].as_str();
    let mut relies_on = BTreeSet::new();
    
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
                relies_on.insert(parsed_referred_struct.to_string());
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    wanted_by.clone(),
                    parsed_referred_struct,
                    &global_yaml["components"]["schemas"][parsed_referred_struct]
                )
                    .with_context(|| format!("Couldn't parse the object {parsed_referred_struct}"))?;
                println!("Finished parsing {parsed_referred_struct}, continuing with enum {key}");

                // Copy all of the enum variants into this one, but first check 
                //  that it wasn't turned into an alias.
                if let Some(alias) = aliases.get(parsed_referred_struct) {
                    println!("Casting enum `{}` to `serde_json::Value` because of child enum `{}`", key, alias);

                    let mut description = String::from("Any of:\n---------------\n");
                    for enum_option in enum_options {
                        description += reserialize_yaml(0, enum_option)
                            .context("Failed to reserialize the YAML")?
                            .as_str();
                        description += "\n---------------\n";
                    }

                    aliases.insert(key.to_string(), Alias {
                        name: key.to_string(),
                        r#type: "serde_json::Value".to_string(),
                        description: Some(description)
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
                    Some(Data::Object(_)) => {
                        // Add the referred struct as a variant
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct.to_case(Case::UpperCamel)));
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

                let mut description = String::from("Any of:\n---------------\n");
                for enum_option in enum_options {
                    description += reserialize_yaml(0, enum_option)
                        .context("Failed to reserialize the YAML")?
                        .as_str();
                    description += "\n---------------\n";
                }

                aliases.insert(key.to_string(), Alias {
                    name: key.to_string(),
                    r#type: "serde_json::Value".to_string(),
                    description: Some(description)
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
                enum_type: EnumType::AnyOf,
                wanted_by: BTreeSet::from([ wanted_by.clone() ]),
                relies_on,
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
                relies_on.insert(parsed_referred_struct.to_string());
                parse(
                    global_yaml,
                    schemas,
                    aliases,
                    wanted_by.clone(),
                    parsed_referred_struct,
                    &global_yaml["components"]["schemas"][parsed_referred_struct]
                )
                    .with_context(|| format!("Couldn't parse the object {parsed_referred_struct}"))?;
                println!("Finished parsing {parsed_referred_struct}, continuing with enum {key}");

                // Add the referred struct to the enum
                match schemas.get(parsed_referred_struct) {
                    Some(Data::Object(_)) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct.to_case(Case::UpperCamel)));
                    },
                    Some(Data::Enum(_)) => {
                        enum_values.push(format!("{}({})", parsed_referred_struct, parsed_referred_struct.to_case(Case::UpperCamel)));
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
                    let array_type = if let Some(referred_type) = enum_option["items"]["$ref"].as_str() {
                        let referred_type = referred_type.split("/")
                            .skip(3)
                            .next()
                            .context("Failed to parse the referred type")?;

                        // Add the requested struct recursively
                        println!("Need to recurse for `array` enum: {referred_type}");
                        relies_on.insert(referred_type.to_string());
                        parse(
                            global_yaml,
                            schemas,
                            aliases,
                            wanted_by.clone(),
                            referred_type,
                            &global_yaml["components"]["schemas"][referred_type]
                        )
                            .with_context(|| format!("Couldn't parse the object {referred_type}"))?;
                        println!("Finished parsing {referred_type}, continuing with enum {key}");

                        referred_type.to_case(Case::UpperCamel)
                    } else if enum_option["items"]["oneOf"].as_vec().is_some() {
                        String::from("Varied")
                    } else {
                        enum_option["items"]["type"].as_str()
                            .context("Failed to get the array type")?
                            .to_case(Case::UpperCamel)
                    };

                    let added_vector_alias_name = format!("{}{}Array", key, array_type);
                    
                    let array_field_value = parse_array(
                        global_yaml,
                        schemas,
                        aliases,
                        wanted_by.clone(),
                        key,
                        key,
                        enum_option
                    )
                        .with_context(|| format!("Couldn't parse the array {key}"))?;
                    let description = enum_option["description"].as_str()
                        .map(|s| s.to_string());

                    aliases.insert(added_vector_alias_name.clone(), Alias {
                        name: added_vector_alias_name.clone(),
                        r#type: format!("{}", array_field_value),
                        description,
                    });

                    enum_values.push(format!("{}({})",
                        added_vector_alias_name.clone(),
                        added_vector_alias_name
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
                wanted_by: BTreeSet::from([ wanted_by ]),
                relies_on,
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
            wanted_by: BTreeSet::from([ wanted_by.clone() ]),
            relies_on,
        })
    );
    println!("Added enum: {}", key);

    Ok(())
}
