use crate::{
    arg_enums::LinkerType,
    cli::CmdArgs,
};

pub struct Config {
    name: String,
    linker_type: LinkerType,
}

impl Config {
    pub fn new(args: &CmdArgs) -> Self {
        Self {
            name: args.get_name(),
            linker_type: args.get_linker_type(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_linker_type_as_string(&self) -> &'static str {
        match self.linker_type {
            LinkerType::A => "A",
            LinkerType::B => "B",
            LinkerType::C => "C",
        }
    }
}