use url::Url;
use regex::Regex;
use crate::inputs::args_contract_type::Contract;

pub struct ContractValidationError;

pub fn validate_contract(c: &Contract) -> Result<(), Err> {

    match c.action.as_str() {

        "new" => println!("Contract of type new"),
        "update" => println!("Contract of type update"),

        _ => {
            println!("Action not recognized: {}", c.action);

            return Err(ContractValidationError)
        },
    }

    let gp_url = Url::parse(c.golden_path.url.as_str())?;
    println!("Parsed Golden Path URL = {}", gp_url);

    let path_regex = Regex::new("^(.+)\/([^\/]+)$").unwrap();

    let Some(capt) = path_regex.captures(c.golden_path.path.as_str()) else {
        println!("{} is not a relative path!", c.golden_path.path);
        return Err(ContractValidationError);
    };

    assert!(c.golden_path.branch.len() > 0);


    return Ok()
}