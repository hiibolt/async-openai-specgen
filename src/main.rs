mod schemas;

mod parsing;
mod types;

use schemas::{CreateResponse, CreateResponseInput, Response, OutputItem, OutputContent};

use std::collections::BTreeMap;

use saphyr::Yaml;
use anyhow::{Context, Result};
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
    for ( _key, value ) in schemas.iter() {
        let stringified = match value {
            Data::Object(object) => {
                format!("{}", object)
            },
            Data::Enum(r#enum) => {
                format!("{}", r#enum)
            }
        };
        //println!("{key}:\n{stringified}\n\n");
        rust_schema_body += &stringified
            .replace("(/docs", "(https://platform.openai.com/docs");
        rust_schema_body += "\n";
    }

    rust_schema_body += "\n\n";

    let mut aliases = aliases.into_iter().collect::<Vec<_>>();
    aliases.sort();
    aliases.dedup();
    for ( _key, value ) in aliases.iter() {
        let stringfied = format!("{value}\n");
        //println!("{stringfied}");
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
    println!("Successfully parsed OpenAI OpenAPI spec:\n - {} schemas\n - {} aliases\n\n", schemas.len(), aliases.len());


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

    Ok(())
}
