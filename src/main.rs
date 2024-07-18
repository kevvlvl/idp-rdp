use pico_args::Arguments;

struct Args {
    dry_run: bool,
    contract_file: String,
}

fn main() {
    println!("START --- Rusty Development Platform");

    let mut args = Arguments::from_env();
    let args = Args {
        dry_run: args.contains(["-d", "--dry-run"]),
        contract_file: args.value_from_str(["-c", "--contract-file"]).unwrap_or("my-rdp.yaml".parse().unwrap()),
    };

    println!("Dry-Run: {}", args.dry_run);
    println!("Contract file: {}", args.contract_file);

    println!("END --- Rusty Development Platform");
}
