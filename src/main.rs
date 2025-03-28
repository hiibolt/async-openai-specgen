mod _schemas;
//mod schemas;

mod parsing;
mod data;

use _schemas::{CreateResponse, CreateResponseInput, Response, OutputItem, OutputContent};

use std::collections::{BTreeMap, BTreeSet};

use saphyr::Yaml;
use anyhow::{bail, Context, Result};
use parsing::{Data, Alias, parse};
fn main() -> Result<()>{
    let openapi_yaml_raw = include_str!("../assets/openapi.yaml");
    let docs = Yaml::load_from_str(&openapi_yaml_raw)
        .context("Failed to load OpenAPI YAML")?;

    let schemas_yaml = docs[0]["components"]["schemas"]
        .as_hash()
        .context("Failed to get schemas")?;

    let mut schemas: BTreeMap<String, Data> = BTreeMap::new();
    let mut aliases: BTreeMap<String, Alias> = BTreeMap::new();

    let paths = docs[0]["paths"]
        .as_hash()
        .context("Failed to get paths")?;

    for (path, yaml) in paths {
        if let Some(path_string) = path.as_str() {
            let root_path = path_string.split('/')
                .skip(1)
                .next()
                .context("Failed to get root path")?;

            println!("Path: {}", root_path);

            for (_sub_field_name, sub_field_yaml) in yaml.as_hash()
                .context("Path YAML for route was not a hashmap!")?
            {
                if let Some(schema_ref) = sub_field_yaml["requestBody"]["content"]["application/json"]["schema"]["$ref"].as_str() {
                    let schema_ref = schema_ref.split("/")
                        .skip(3)
                        .next()
                        .context("Failed to get schema reference")?;

                    println!("Schema ref: {}", schema_ref);

                    // Find the schema reference in the schemas
                    if let Some(schema_yaml) = schemas_yaml.get(&Yaml::String(schema_ref.to_string())) {
                        // Parse the schema
                        parse(
                            &docs[0],
                            &mut schemas,
                            &mut aliases,
                            root_path.to_string(),
                            schema_ref,
                            schema_yaml
                        )
                        .context("Failed to parse the schema")?;
                    } else {
                        bail!("Schema reference {} not found in schemas", schema_ref);
                    }
                }
            }
        }
    }

    /*
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
    */

    // Print the schema and alias Rust types
    let mut rust_schema_bodies: BTreeMap<String, (String, BTreeSet<String>)> = BTreeMap::new();
    for ( key, value ) in schemas.iter() {
        let relies_on;
        let wanted_by;
        let stringified = match value {
            Data::Object(object) => {
                wanted_by = object.wanted_by.clone();
                relies_on = object.relies_on.clone();
                format!("{}", object)
            },
            Data::Enum(r#enum) => {
                wanted_by = r#enum.wanted_by.clone();
                relies_on = r#enum.relies_on.clone();
                format!("{}", r#enum)
            }
        };
        let file_title = if wanted_by.is_empty() {
            "uncategorized".to_string()
        } else if wanted_by.len() == 1 {
            format!("{}", wanted_by.iter().next().context("Unreachable wanted_by")?)
        } else {
            format!("{}", wanted_by.iter().map(|st| st.to_owned()).collect::<Vec<String>>().join("_"))
        };

        let rust_schema_body = rust_schema_bodies
            .entry(file_title.clone())
            .or_insert_with(|| (String::new(), BTreeSet::new()));

        // Copy the relies_on to the file as `use` statements
        for typename in relies_on.iter() {
            // Find the file title of the type schema
            let typename_schema = match schemas.get(typename) {
                Some(Data::Object(object)) => {
                    object.wanted_by.clone()
                },
                Some(Data::Enum(r#enum)) => {
                    r#enum.wanted_by.clone()
                },
                _ => {
                    // Check if it's an alias
                    if let Some(_) = aliases.get(typename) {
                        BTreeSet::from(["aliases".to_string()])
                    } else {
                        bail!("Type {} (needed for {key}) not found in schemas or aliases", typename);
                    }
                }
            };

            let inner_file_title = if typename_schema.is_empty() {
                "uncategorized".to_string()
            } else if typename_schema.len() == 1 {
                format!("{}", typename_schema.iter().next().context("Unreachable wanted_by")?)
            } else {
                format!("{}", typename_schema.iter().map(|st| st.to_owned()).collect::<Vec<String>>().join("_"))
            };

            // Add the `use` statement to the file if it's not in the same file
            if file_title.as_str() == inner_file_title.as_str() {
                continue;
            }
            (*rust_schema_body).1.insert(format!("use super::{}::{};", inner_file_title, typename));
        }
        (*rust_schema_body).0 += &stringified
            .replace("(/docs", "(https://platform.openai.com/docs");
        (*rust_schema_body).0 += "\n";
    }

    // First, delete and create the schemas directory
    std::fs::remove_dir_all("src/schemas")
        .context("Failed to remove schemas directory")?;
    std::fs::create_dir_all("src/schemas")
        .context("Failed to create schemas directory")?;

    // Create the `mod.rs` file
    let mod_content = rust_schema_bodies.keys()
        .map(|file_name| format!("pub mod {};\n", file_name.replace(".rs", "")))
        .collect::<String>()
        +
        "\n\n"
        +
        &rust_schema_bodies
            .keys()
            .map(|file_name| format!("pub use {}::*;\n", file_name.replace(".rs", "")))
            .collect::<String>();
    std::fs::write("src/schemas/mod.rs", &mod_content)
        .context("Failed to write schemas mod.rs file")?;

    // Write the useful snippets to files
    for (file_name, body) in rust_schema_bodies.iter() {
        let body = format!(
            "{}\nuse std::collections::HashMap;\nuse serde::{{Serialize, Deserialize}};\n\n{}",
            body.1.iter().map(|st| format!("{st}\n")).collect::<String>(),
            body.0
        );

        std::fs::write(format!("src/schemas/{}.rs", file_name), body)
            .context("Failed to write OpenAPI Rust schemas to file")?;
    }

    let mut aliases = aliases.into_iter().collect::<Vec<_>>();
    let mut alias_body = String::new();
    aliases.sort();
    aliases.dedup();
    for ( _key, value ) in aliases.iter() {
        let alias = format!("{}", value);
        alias_body += &alias;
        alias_body += "\n";
    }
    std::fs::write("src/schemas/aliases.rs", &alias_body)
        .context("Failed to write OpenAPI Rust aliases to file")?;

    // Write the useful snippets to files
    /*
    std::fs::write("assets/openapi.txt", format!("{:#?}", docs))
        .context("Failed to write OpenAPI YAML to file")?;
    std::fs::write("assets/openapi-schemas.txt", format!("{:#?}", schemas))
        .context("Failed to write OpenAPI schema text to file")?;
    std::fs::write("src/schemas.rs", &format!(
        "use std::collections::HashMap;\nuse serde::{{Serialize, Deserialize}};\n\n{}",
        rust_schema_body
    ))
        .context("Failed to write OpenAPI Rust schemas to file")?;
    println!("Successfully parsed OpenAI OpenAPI spec:\n - {} schemas\n - {} aliases\n\n", schemas.len(), aliases.len());
     */

    /*
    let create_response = CreateResponse {
        model: Some(serde_json::Value::String("gpt-3.5-turbo".to_string())),
        input: Some(CreateResponseInput::String("Hello world".to_string())),
        ..Default::default()
    };
    let create_response_json = serde_json::to_string(&create_response)
        .context("Failed to serialize CreateResponse")?;
    std::fs::write("assets/create_response.json", &create_response_json)
        .context("Failed to write CreateResponse JSON to file")?;

    let openai_key = std::env::var("OPENAI_API_KEY")
        .context("Failed to get OpenAI API key")?;
    let body = ureq::post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {openai_key}"))
        .send_json(&create_response)
        .context("Failed to send request to OpenAI API")?
        .body_mut()
        .read_to_string()?;

    std::fs::write("assets/openai_response.json", body.clone())
        .context("Failed to write OpenAI response JSON to file")?;

    let response = serde_json::from_str::<Response>(&body)
        .context("Failed to deserialize OpenAI response")?;
    let response_typing_string = format!("{response:#?}");

    std::fs::write("assets/openai_response.txt", &response_typing_string)
        .context("Failed to write OpenAI response text to file")?;

    let mut outputs_string = String::new();
    if let Some(outputs) = response.output {
        for output in outputs {
            match output {
                OutputItem::OutputMessage(output) => {
                    for output_content in output.content {
                        match output_content {
                            OutputContent::OutputText(output_text) => {
                                outputs_string += &format!("\"{}\"\n", output_text.text);
                            },
                            _ => {}
                        }
                    }
                }
                _ => { }
            }
        }
    }

    println!("Outputs:\n{outputs_string}");
    std::fs::write("assets/openai_response_outputs.txt", &outputs_string)
        .context("Failed to write OpenAI response outputs text to file")?;
    */

    Ok(())
}
