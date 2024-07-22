use std::fs;
use pico_args::Arguments;
use serde::Deserialize;

pub struct Args {
    pub dry_run: bool,
    pub cf: String,
}

#[derive(Debug, Deserialize)]
pub struct Contract {
    pub action: String,

    // FIXME: how to name
    pub golden_path: GoldenPath
}

#[derive(Debug, Deserialize)]
pub struct GoldenPath {
    pub url: String,
    pub path: String,
    pub branch: String
}

pub fn read_cli_args(mut args: Arguments) {

    let args = Args {
        dry_run: args.contains(["-d", "--dry-run"]),
        cf: args.value_from_str(["-c", "--contract-file"]).unwrap_or("my-rdp.yaml".parse().unwrap()),
    };

    println!("Dry-Run: {}", args.dry_run);
    println!("Contract file: {}", args.cf);

    read_contract_file(&args.cf);
}

fn read_contract_file(file_path: &String) {

    let cf_content: String = fs::read_to_string(file_path).expect("File Path not found. Unable to read contract file!");

    println!("Contract File Read. Contents = {}", cf_content);

    let yaml: Contract = serde_yml::from_str(&cf_content).unwrap();
}