use clap::Parser;
use crate::{
    arg_enums::LinkerType,
    config::Config,
};


/// Linker Rust implementation from Linkers & Loaders.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// Linker Name
    #[clap(short, long, value_parser)]
    name: String,

    /// Linker Type
    ///
    /// - `a`: does a thing
	/// - `b`: does b thing
	/// - `c`: does c thing
    #[clap(
		long,
		value_name = "METHOD SET",
		arg_enum,
		ignore_case = true,
		default_value = "a",
		verbatim_doc_comment
	)]
    linker_type: LinkerType,
}

impl CmdArgs {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_linker_type(&self) -> LinkerType {
        self.linker_type
    }
}


pub fn run() -> Config {
    let args = CmdArgs::parse();
    let config = Config::new(&args);
    config
}