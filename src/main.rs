mod perms;
mod args;

use perms::{ FilePerm, Convertible };
use args::{ Cli, Action };
use clap::{ Parser };

fn main() {
    let args = Cli::parse();
    let absolute_value: String;
    let symbolic_value: String;
    match &args.action {
        Action::Absolute(absolute) => {
            absolute_value = String::from(&absolute.absolute);
            let sym_val = &FilePerm::from_absolute(&absolute.absolute.to_owned()).get_symbolic();
            symbolic_value = String::from(sym_val);
        },
        Action::Symbolic(symbolic) => {
            symbolic_value = String::from(&symbolic.symbolic);
            let abs_val = FilePerm::from_symbolic(&symbolic.symbolic.to_owned()).get_absolute();
            absolute_value = String::from(abs_val);
        }
    }
    println!("Absolute Value :: {}", absolute_value);
    println!("Symbolic Value :: {}", symbolic_value);
}
