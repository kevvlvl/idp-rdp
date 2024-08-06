use std::fs;
use pico_args::Arguments;
use crate::inputs::args_contract_type::Contract;
use crate::inputs::contract_rules::validate_contract;

pub struct Args {
    pub dry_run: bool,
    pub cf: String,
}

pub fn read_inputs(args: Arguments) {

    let parsed_args = read_cli_args(args);
    let c: Contract = read_contract_file(&parsed_args.cf);
    validate_contract(&c).expect("Failed to validate contract");
}

fn read_cli_args(mut args: Arguments) -> Args {

    let a = Args {
        dry_run: args.contains(["-d", "--dry-run"]),
        cf: args.value_from_str(["-c", "--contract-file"]).unwrap_or("my-rdp.yaml".parse().unwrap()),
    };

    println!("Dry-Run: {}", a.dry_run);
    println!("Contract file: {}", a.cf);

    return a;
}

fn read_contract_file(file_path: &String) -> Contract {

    let cf_content: String = fs::read_to_string(file_path).expect("File Path not found. Unable to read contract file!");

    println!("Contract File Read. Contents = {}", cf_content);

    let _yaml: Contract = serde_yml::from_str(&cf_content).unwrap();

    println!("Contract deserialized");

    _yaml
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;
    use std::str::FromStr;
    use crate::inputs::args_contract_type::{Code, CodeTool, GoldenPath};
    use super::*;

    #[test]
    fn test_read_valid_cli_args() {

        let mut input_args = Vec::new();
        input_args.push(OsString::from_str("--contract-file").unwrap());
        input_args.push(OsString::from_str("my-idp.yaml").unwrap());
        input_args.push(OsString::from_str("--dry-run").unwrap());

        let a = read_cli_args(Arguments::from_vec(input_args));

        assert!(a.cf.len() > 0);
        assert!(a.dry_run);
    }

    #[test]
    fn test_read_valid_new_contract() {

        let valid_contract = Contract {
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

        let yaml = serde_yml::to_string(&valid_contract).unwrap();
        assert!(yaml.len() > 0);

        let deserialized: Contract = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(valid_contract, deserialized);
    }
}