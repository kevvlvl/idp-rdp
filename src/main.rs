use pico_args::Arguments;

mod inputs;

fn main() {
    println!("START --- Rusty Development Platform");

    inputs::args::read_inputs(Arguments::from_env());

    println!("END --- Rusty Development Platform");
}
