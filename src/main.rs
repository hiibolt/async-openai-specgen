use saphyr::Yaml;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
enum FieldValue {
    Object(String),
    Enum(String),
    Array(String),
    String,
    Integer,
}
impl std::fmt::Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FieldValue::Object(value) => write!(f, "{}", value),
            FieldValue::Enum(value) => write!(f, "{}", value),
            FieldValue::Array(value) => write!(f, "Vec<{}>", value),
            FieldValue::String => write!(f, "String"),
            FieldValue::Integer => write!(f, "i64"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Field {
    /// The description of the field
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// The value of the field
    value: FieldValue,
}
#[derive(Debug, Serialize, Deserialize)]
struct Object {
    /// The name of the object
    name: String,
    /// The description of the object
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// The properties of the object
    properties: HashMap<String, Field>,
}
impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut body = String::new();

        // Write the struct definition
        if let Some(ref description) = self.description {
            for line in description.lines() {
                body.push_str(&format!("/// {}\n", line));
            }
        }

        // Write the object name
        body.push_str(&format!("pub struct {} {{\n", self.name));

        // Write the object properties
        for (key, value) in self.properties.iter() {
            if let Some(ref description) = value.description {
                for line in description.lines() {
                    body.push_str(&format!("    /// {}\n", line));
                }
            }

            body.push_str(&format!("    pub {}: {},\n", key, value.value));
        }

        // Close the struct definition
        body.push_str("}");

        write!(f, "{}", body)
    }
}
struct Enum {}
enum Data {
    Object(Object),
    Enum(Enum)
}

fn main() -> Result<()>{
    let openapi_yaml_raw = include_str!("../assets/openapi.yaml");
    let docs = Yaml::load_from_str(&openapi_yaml_raw)
        .context("Failed to load OpenAPI YAML")?;

    let schemas = docs[0]["components"]["schemas"]
        .as_hash()
        .context("Failed to get schemas")?;

    for (key, value) in schemas.iter() {
        let key = key.as_str().context("Failed to get key")?;

        if key != "InputFile" {
            continue;
        }

        println!("Key: {}", key);
        println!("Value: {:#?}", value);

        match value["type"].as_str() {
            Some("object") => {
                let description = value["description"].as_str();
                let properties = value["properties"].as_hash()
                    .context("Failed to get properties")?;

                let mut object = Object {
                    name: key.to_string(),
                    description: description.map(|s| s.to_string()),
                    properties: HashMap::new(),
                };

                for (key, value) in properties.iter() {
                    let key = key.as_str().context("Failed to get key")?;

                    let description = value["description"].as_str();
                    let value = match value["type"].as_str() {
                        Some("object") => {
                            FieldValue::Object(value["type"].as_str().unwrap().to_string())
                        },
                        Some("enum") => {
                            FieldValue::Enum(value["type"].as_str().unwrap().to_string())
                        },
                        Some("array") => {
                            FieldValue::Array(value["type"].as_str().unwrap().to_string())
                        },
                        Some("string") => {
                            FieldValue::String
                        },
                        Some("integer") => {
                            FieldValue::Integer
                        },
                        _ => {
                            panic!("No type found");
                        }
                    };

                    object.properties.insert(key.to_string(), Field {
                        description: description.map(|s| s.to_string()),
                        value,
                    });

                    println!("Key: {}", key);
                    println!("Object: {:#?}", object);
                    println!("Code:\n{}", object);
                }
            },
            Some(&_) => {
                panic!("Unsupported type found");
            }
            None => {
                panic!("No type found");
            }
        }
    }

    // Write the useful snippets to files
    std::fs::write("assets/openapi.txt", format!("{:#?}", docs))
        .context("Failed to write OpenAPI YAML to file")?;
    std::fs::write("assets/openapi-schemas.txt", format!("{:#?}", schemas))
        .context("Failed to write OpenAPI YAML to file")?;

    Ok(())
}
