use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub dest: Option<String>,
    #[arg(short = 'a')]
    pub add_flag: bool,
    #[arg(short = 'e')]
    pub edit_flag: bool,
    #[arg(short = 'r')]
    pub remove_flag: bool,
}