mod schemas;

mod r#enum;
mod object;

use std::collections::{HashSet, HashMap};

use convert_case::{Case, Casing};
use saphyr::Yaml;
use anyhow::{Context, Result};

use r#enum::Enum;
use object::{Object, FieldValue, Field};

#[derive(Debug)]
enum Data {
    Object(Object),
    Enum(Enum)
}

fn parse_enum (
    global_yaml: &Yaml,

    schemas: &mut HashMap<String, Data>,
    key: &str,
    value: &Yaml
) -> Result<()> {
    println!("Value: {value:#?}");

    let description = value["description"].as_str();
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
        })
    );


    Ok(())
}
fn parse_object (
    global_yaml: &Yaml,

    schemas: &mut HashMap<String, Data>,
    key: &str,
    value: &Yaml
) -> Result<()> {
    let description = value["description"].as_str();
    let properties = value["properties"].as_hash()
        .context("Failed to get properties")?;
    let mut required = HashSet::new();
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
        properties: HashMap::new(),
    };

    for (property_key, property_value) in properties.iter() {
        let property_key = property_key.as_str().context("Failed to get key")?;

        let description = property_value["description"].as_str();
        let field_value = match property_value["type"].as_str() {
            Some("object") => {
                FieldValue::Object(property_value["type"].as_str().unwrap().to_string())
            },
            Some("enum") => {
                FieldValue::Enum(property_value["type"].as_str().unwrap().to_string())
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
                        field_type_key.as_str(),
                        property_value
                    )
                        .context("Couldn't parse the enum!")?;
                    
                    FieldValue::Enum(field_type_key)
                } else {
                    FieldValue::String
                }
            },
            Some("integer") => {
                FieldValue::Integer
            },
            _ => {
                if let Some(referred_type) = property_value["$ref"].as_str() {
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
                        parsed_referred_type,
                        &referred_type_yaml,
                    )
                        .context("Couldn't parse the object!")?;

                    FieldValue::Object(parsed_referred_type.to_string())
                } else {
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

    Ok(())
} 
fn parse (
    global_yaml: &Yaml,

    schemas: &mut HashMap<String, Data>,
    key: &str,
    value: &Yaml
) -> Result<()> {
    let allowed = vec!(
        "InputText",
        "InputImage",
        "InputFile",
        "Reasoning"
    );
    if !allowed.contains(&key) {
        return Ok(());
    }

    println!("Key: {}", key);
    println!("Value: {:#?}", value);

    match value["type"].as_str() {
        Some("object") => {
            parse_object(
                &global_yaml,
                schemas,
                key,
                value
            )
                .context("Couldn't parse the object!")?;
        },
        Some("string") => {
            if value["enum"].as_vec().is_some() {
                parse_enum(
                    &global_yaml,
                    schemas,
                    key,
                    value
                )
                    .context("Couldn't parse the enum!")?;
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

    let mut schemas: HashMap<String, Data> = HashMap::new();

    for (key, value) in schemas_yaml.iter() {
        let key = key.as_str().context("Failed to get key")?;

        parse(
            &docs[0],
            &mut schemas,
            key,
            value
        )
            .context("Failed to parse the schema")?;
    }

    // Print the schema Rust types
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
