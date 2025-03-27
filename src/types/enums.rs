use convert_case::{Case, Casing};
use regex::{Captures, Regex};

#[derive(Debug, PartialEq)]
pub enum EnumType {
    Standard,
    OneOf,
}
#[derive(Debug)]
pub struct Enum {
    /// The name of the enum
    pub name: String,
    /// The description of the enum
    pub description: Option<String>,
    /// The values of the enum
    pub values: Vec<String>,
    /// The type of the enum
    pub enum_type: EnumType,
}
impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut body = String::new();

        // Write the enum description
        if let Some(ref description) = self.description {
            for line in description.lines() {
                body.push_str(&format!("/// {}\n", line));
            }
        }

        // Write derives and tell `serde` to convert 
        //  to `lowercase`
        body.push_str("#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]\n");
        body.push_str("#[serde(rename_all = \"lowercase\")]\n");
        body.push_str("#[serde(untagged)]\n");

        // Write the enum name
        body.push_str(&format!("pub enum {} {{\n", self.name.replace("[]", "")));

        // Write the enum values
        for value in self.values.iter() {
            // If it's a `oneOf` enum, don't adjust the value
            if self.enum_type == EnumType::OneOf {
                body.push_str(&format!("\t{},\n", value));
                continue;
            }

            let fix_numbers = Regex::new(r"(\d)-(\d)").unwrap();
            let fixed_value = fix_numbers.replace_all(value, |caps: &Captures| {
                format!("{}_{}", &caps[1], &caps[2])
            }).replace(".", "_");

            // Convert the value to `UpperCamel` case
            let converted = fixed_value
                .to_case(Case::UpperCamel)
                .replace(".", "_");

            
            let primitives = vec!(
                "String(String)",
            );
            if primitives.contains(&value.as_str()) {
                body.push_str(&format!("\t{},\n", value));
                continue;
            }

            // Check if the field starts with a number (not valid in Rust)
            if value.chars().next().expect("Can't have a zero-char enum variant, unreachable").is_numeric() {
                body.push_str("\t#[serde(rename = \"");
                body.push_str(&value);
                body.push_str("\")]\n");

                body.push_str(&format!("\tType{},\n", value));

                continue;
            }
            
            // Don't waste your breath on a value that doesn't need renaming
            if converted.to_lowercase() != *value {
                body.push_str("\t#[serde(rename = \"");
                body.push_str(&value);
                body.push_str("\")]\n");
            }

            body.push_str(&format!("\t{},\n", converted));
        }

        // Close the enum definition
        body.push_str("}");
        
        write!(f, "{}", body)
    }
}