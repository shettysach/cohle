mod assets;
mod parse;
mod print;

use assets::*;
use parse::*;
use print::*;

use clap::Parser;

fn main() {
    let args = Arguments::parse();
    let mut quotes = QUOTES.lines();
    let img = IMAGES;

    match args.command {
        Some(Commands::List) => {
            list_quotes(quotes);
        }

        Some(Commands::Quote) => {
            only_quote(
                quotes
                    .nth(args.quote_index as usize)
                    .expect("Quote of index [max = 32]"),
                &args.print_opts,
            );
        }

        Some(Commands::Image) => {
            only_image(
                img.get(args.image_index as usize)
                    .expect("Image out of index [max = 1]"),
                &args.print_opts,
            );
        }

        None => {
            quote_image(
                img.get(args.image_index as usize)
                    .expect("Image out of index [max = 1]"),
                quotes
                    .nth(args.quote_index as usize)
                    .expect("Quote of index [max = 32]"),
                &args.print_opts,
            );
        }
    }
}
