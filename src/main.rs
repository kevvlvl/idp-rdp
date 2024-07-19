use pico_args::Arguments;

mod inputs;

fn main() {
    println!("START --- Rusty Development Platform");

    let args = Arguments::from_env();

    inputs::args::read_cli_args(args);

    println!("END --- Rusty Development Platform");
}
