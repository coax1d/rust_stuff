use std::env;
mod cli;
mod arg_enums;
mod config;


fn main() {
    let prog_config = cli::run();
    println!("linker_name: {} linker_type: {}",
        prog_config.get_name(),
        prog_config.get_linker_type_as_string());
}
