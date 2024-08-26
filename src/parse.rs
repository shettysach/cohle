use clap::{Args, Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(about = format!(
    "{} {}",
    "Cohle:".blue().bold().underline(),
    "Rust CLI that prints Rust Cohle quotes."
))]
pub struct Arguments {
    /// Index of the quote, defaults to random
    #[arg(default_value_t = fastrand::u8(0..32))]
    pub quote_index: u8,

    /// Index of the image
    #[arg(short = 'm', default_value_t = 0)]
    pub image_index: u8,

    #[command(flatten)]
    pub print_opts: PrintOpts,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print only quote without image
    Quote,

    /// Print only image without quote
    Image,

    /// Lists all the quotes and colours along with their indices
    List,
}

#[derive(Args)]
pub struct PrintOpts {
    /// Colour of quote
    #[arg(short = 'c', default_value_t = 'w')]
    pub colour: char,

    /// Print quote in bold
    #[arg(short = 'b', long = "bold", default_value_t = false)]
    pub bold: bool,

    /// Print quote italicised
    #[arg(short = 'i', long = "italic", default_value_t = false)]
    pub italic: bool,

    /// Print image with black background
    #[arg(short = 'g', long = "background", default_value_t = false)]
    pub background: bool,
}
