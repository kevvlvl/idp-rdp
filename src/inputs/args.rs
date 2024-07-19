use pico_args::Arguments;

pub struct Args {
    pub dry_run: bool,
    pub contract_file: String,
}

pub fn read_cli_args(mut args: Arguments) {

    let args = Args {
        dry_run: args.contains(["-d", "--dry-run"]),
        contract_file: args.value_from_str(["-c", "--contract-file"]).unwrap_or("my-rdp.yaml".parse().unwrap()),
    };

    println!("Dry-Run: {}", args.dry_run);
    println!("Contract file: {}", args.contract_file);
}
