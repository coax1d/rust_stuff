use clap::ArgEnum;

#[derive(Debug, Copy, Clone, PartialEq, ArgEnum)]
pub enum LinkerType {
    /// Does A things
    A,
    /// Does B things
    B,
    /// Does C things
    C,
}