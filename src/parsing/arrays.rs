use super::{
    Data, Alias, parse,

    enums::parse_enum,
    objects::parse_object,

    super::data::objects::FieldValue
};

use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use saphyr::Yaml;
use anyhow::{Context, Result, bail};

pub(super) fn parse_array (
    global_yaml: &Yaml,

    schemas: &mut BTreeMap<String, Data>,
    aliases: &mut BTreeMap<String, Alias>,
    wanted_by: String,

    key: &str,
    property_key: &str,
    property_value: &Yaml
) -> Result<FieldValue> {
    println!("Parsing key {key} (property key {property_key}) as an array: {property_value:#?}");

    // First, check if it's a `oneOf` or `allOf` array
    if property_value["items"]["oneOf"].as_vec().is_some() {
        let field_type_key = format!(
            "{}Items",
            key
        );

        // Parse the object
        parse_enum(
            global_yaml,
            schemas,
            aliases,
            wanted_by,
            field_type_key.as_str(),
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
        println!("Finished recursively adding `oneOf` object {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key));
    }
    if property_value["items"]["allOf"].as_vec().is_some() {
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
            wanted_by,
            field_type_key.as_str(),
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
        println!("Finished recursively adding `allOf` object {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key));
    }

    // Second, check if the array type is a direct external reference
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
            wanted_by.clone(),
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

    // Lastly, check if it's an array of objects or enums
    if property_value["items"]["properties"].as_hash().is_some() {
        let field_type_key = format!(
            "{}{}Item", 
            key,
            property_key.to_case(Case::UpperCamel)
        );

        // Recursively add the object as `keyPropertyKey`
        parse_object(
            global_yaml,
            schemas,
            aliases,
            wanted_by,
            field_type_key.as_str(),
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the object {field_type_key}"))?;
        println!("Finished recursively adding object field {field_type_key}, continuing object {key}");

        return Ok(FieldValue::Array(field_type_key))
    } else if property_value["items"]["enum"].as_vec().is_some() {
        let field_type_key = format!(
            "{}Item",
            key
        );

        // Parse the enum
        parse_enum(
            global_yaml,
            schemas,
            aliases,
            wanted_by,
            field_type_key.as_str(),
            &property_value["items"]
        )
            .with_context(|| format!("Couldn't parse the enum {field_type_key}"))?;
        println!("Finished recursively adding enum {field_type_key}, continuing object {key}");

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
                wanted_by,
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
                wanted_by,
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
