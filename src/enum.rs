use convert_case::{Case, Casing};
use regex::{Captures, Regex};

#[derive(Debug)]
pub struct Enum {
    /// The name of the enum
    pub name: String,
    /// The description of the enum
    pub description: Option<String>,
    /// The values of the enum
    pub values: Vec<String>,
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

        // Write the enum name
        body.push_str(&format!("pub enum {} {{\n", self.name));

        // Write the enum values
        for value in self.values.iter() {
            // Fix the numbers in the value
            let fix_numbers = Regex::new(r"(\d)-(\d)").unwrap();
            let fixed_value = fix_numbers.replace_all(value, |caps: &Captures| {
                format!("{}_{}", &caps[1], &caps[2])
            });

            // Convert the value to `UpperCamel` case
            let converted = fixed_value
                .to_case(Case::UpperCamel)
                .replace(".", "_");

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