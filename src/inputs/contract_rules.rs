use std::fmt::{Debug, Display, Formatter};
use url::Url;
use regex::Regex;
use crate::inputs::args_contract_type::Contract;

#[derive(PartialEq)]
pub enum ContractValidationError {
    MissingProperties,
    InvalidPropertyValues
}

impl ContractValidationError {
    fn message(&self) -> &str {
        match self {
            ContractValidationError::MissingProperties => "Missing Properties",
            ContractValidationError::InvalidPropertyValues => "Invalid Property Values",
        }
    }
}

impl Display for ContractValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ContractValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub fn validate_contract(c: &Contract) -> Result<(), ContractValidationError> {

    println!("BEGIN validate_contract");

    match c.action.as_str() {

        "new" => println!("Contract of type new"),
        "update" => println!("Contract of type update"),

        _ => {
            println!("Action not recognized: {}", c.action);
            return Err(ContractValidationError::InvalidPropertyValues)
        },
    }

    let gp_url = Url::parse(c.golden_path.url.as_str()).expect("Failed to parse URL");
    println!("Parsed Golden Path URL = {}", gp_url);

    let path_regex = Regex::new(r##"^(.+)/([^/]+)$"##).unwrap();

    let Some(capt) = path_regex.captures(c.golden_path.path.as_str()) else {
        println!("{} is not a relative path!", c.golden_path.path);
        return Err(ContractValidationError::InvalidPropertyValues);
    };

    assert!(c.golden_path.branch.len() > 0);

    println!("DONE validate_contract - validated");

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::inputs::args_contract_type::{Code, CodeTool, GoldenPath};
    use super::*;

    #[test]
    fn test_valid_new_contract() {

        let valid_new_contract = Contract {
            action: "new".to_string(),
            golden_path: GoldenPath {
                url: "https://test.local/gp".to_string(),
                path: "./my_gp".to_string(),
                branch: "main".to_string(),
            },
            code: Code {
                c_type: CodeTool::GITHUB,
                url: "http://test.local/code".to_string(),
                branch: "main".to_string(),
            },
        };

        let r = validate_contract(&valid_new_contract);
        assert_eq!(Ok(()), r);
    }

    #[test]
    fn test_valid_update_contract() {

        let valid_update_contract = Contract {
            action: "update".to_string(),
            golden_path: GoldenPath {
                url: "https://test.local/gp".to_string(),
                path: "./my_gp".to_string(),
                branch: "main".to_string(),
            },
            code: Code {
                c_type: CodeTool::GITHUB,
                url: "http://test.local/code".to_string(),
                branch: "main".to_string(),
            },
        };
    }

    #[test]
    fn test_unrecognized_action_contract() {
    }
}