use std::fs;
use pico_args::Arguments;
use crate::inputs::args_contract_type::Contract;

pub struct Args {
    pub dry_run: bool,
    pub cf: String,
}

pub fn read_inputs(args: Arguments) {

    let parsed_args = read_cli_args(args);
    read_contract_file(&parsed_args.cf);
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

fn read_contract_file(file_path: &String) {

    let cf_content: String = fs::read_to_string(file_path).expect("File Path not found. Unable to read contract file!");

    println!("Contract File Read. Contents = {}", cf_content);

    let _yaml: Contract = serde_yml::from_str(&cf_content).unwrap();

    println!("Contract deserialized");
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_read_cli_args() {

        let mut input_args = Vec::new();
        input_args.push(OsString::from_str("--contract-file").unwrap());
        input_args.push(OsString::from_str("my-idp.yaml").unwrap());
        input_args.push(OsString::from_str("--dry-run").unwrap());

        let a = read_cli_args(Arguments::from_vec(input_args));

        assert!(a.cf.len() > 0);
        assert!(a.dry_run);
    }
}