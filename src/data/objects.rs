use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub enum FieldValue {
    ExternalType(String),
    Array(String),
    String,
    Integer,
    Boolean
}
impl std::fmt::Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FieldValue::ExternalType(value) => write!(f, "{}", value.replace("[]", "")),
            FieldValue::Array(value) => write!(f, "Vec<{}>", value.replace("[]", "")),
            FieldValue::String => write!(f, "String"),
            FieldValue::Integer => write!(f, "i64"),
            FieldValue::Boolean => write!(f, "bool"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Field {
    /// The description of the field
    pub description: Option<String>,
    /// The value of the field
    pub value: FieldValue,
    /// Whether or not the field is required
    pub required: bool,
}
#[derive(Debug)]
pub struct Object {
    /// The name of the object
    pub name: String,
    /// The description of the object
    pub description: Option<String>,
    /// The properties of the object
    pub properties: BTreeMap<String, Field>,
    /// Which paths wants this object
    pub wanted_by: BTreeSet<String>,
    /// Which types this object relies on
    pub relies_on: BTreeSet<String>,
}
impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut body = String::new();

        // Write the struct description
        if let Some(ref description) = self.description {
            for line in description.lines() {
                body.push_str(&format!("/// {}\n", line));
            }
        }

        // Write derives
        if self.properties.iter().all(|(_key, value)| !value.required) {
            body.push_str("#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]\n");
        } else {
            body.push_str("#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]\n");
        }

        // Write the object name
        body.push_str(&format!("pub struct {} {{\n", self.name));

        // Write the object properties
        for (key, value) in self.properties.iter() {
            if let Some(ref description) = value.description {
                for line in description.lines() {
                    body.push_str(&format!("\t/// {}\n", line));
                }
            }

            // Fix the `type` key
            let key = if key.contains("/") || key.contains("-") {
                body.push_str("\t#[serde(rename = \"");
                body.push_str(key);
                body.push_str("\")]\n");
                key.replace("/", "_")
                    .replace("-", "_")
            } else {
                key.to_string()
            };

            let key = if key == "type" {
                "r#type"
            } else if key == "static" {
                "r#static"
            } else {
                &key
            }.replace("[]", "");

            // Check if it needs to be renamed
            

            if !value.required {
                body.push_str("\t#[serde(skip_serializing_if = \"Option::is_none\")]\n");
                body.push_str(&format!("\tpub {}: Option<{}>,\n", key, value.value));
            } else {
                body.push_str(&format!("\tpub {}: {},\n", key, value.value));
            }
        }

        // Close the struct definition
        body.push_str("}");

        write!(f, "{}", body)
    }
}