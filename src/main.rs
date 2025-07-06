mod error;
mod parse;
mod print;

use error::CmdError;
use parse::*;
use print::*;

use clap::Parser;

const IMAGE: &str = include_str!("../assets/image.txt");
const QUOTES: &str = include_str!("../assets/quotes.txt");

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), CmdError> {
    let args = Arguments::parse();
    let mut quotes = QUOTES.lines();

    match args.command {
        Some(Commands::List) => list_quotes(quotes),

        Some(Commands::Image) => only_image(IMAGE, &args.print_opts),

        Some(Commands::Quote) => only_quote(
            quotes
                .nth(args.quote_index as usize)
                .ok_or(CmdError::QuoteIndex(args.quote_index))?,
            &args.print_opts,
        )?,

        None => quote_image(
            IMAGE,
            quotes
                .nth(args.quote_index as usize)
                .ok_or(CmdError::QuoteIndex(args.quote_index))?,
            &args.print_opts,
        )?,
    }

    Ok(())
}
