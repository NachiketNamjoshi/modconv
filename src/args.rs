use clap::{ Parser, Subcommand, Args };

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action
}

#[derive(Subcommand)]
pub enum Action {
    Absolute(Absolute),
    Symbolic(Symbolic),
}


#[derive(Args)]
pub struct Symbolic {
    /// Convert Symbolic Permissions
    #[clap(value_parser)]
    pub symbolic: String,
}

#[derive(Args)]
pub struct Absolute {
    /// Convert Absolute Permissions
    #[clap(value_parser)]
    pub absolute: String,
}
