use colored::{ColoredString, Colorize};
use terminal_size::{terminal_size, Width};
use textwrap::{fill, wrap};

use crate::parse::PrintOpts;

pub fn only_quote(input_text: &str, opts: &PrintOpts) {
    let (Width(term_width), _) = terminal_size().expect("Error in getting terminal size");
    let filled_text = fill(input_text, term_width as usize);
    println!("{}", opts_print(filled_text.as_str(), &opts));
}

pub fn only_image(image: &str, opts: &PrintOpts) {
    if opts.background {
        image.lines().for_each(|imline| {
            println!("{}", imline.on_black());
        });
    } else {
        println!("{}", image);
    };
}

pub fn quote_image(image: &str, quote: &str, opts: &PrintOpts) {
    let (Width(term_width), _) = terminal_size().expect("Error in getting terminal size");
    let width = term_width.checked_sub(56);

    match width {
        Some(value) => {
            let imvec = image
                .lines()
                .map(|imline| {
                    if opts.background {
                        imline.on_black()
                    } else {
                        imline.clear()
                    }
                })
                .collect::<Vec<ColoredString>>();

            let qvec = wrap(quote, value as usize);

            let imlen = imvec.len();
            let qlen = qvec.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}", imvec[i]);
                }
                for i in start..(start + qlen) {
                    println!("{} {}", imvec[i], opts_print(&qvec[i - start], &opts));
                }

                for i in (start + qlen)..imlen {
                    println!("{}", imvec[i]);
                }
            } else {
                only_image(image, &opts);
                println!();
                only_quote(quote, &opts);
            }
        }
        None => {
            println!(
                "{}",
                "Terminal width too small to print image.
                Resize terminal / reduce font or use 'cohle q'"
                    .bright_black()
            );
            only_quote(quote, &opts);
        }
    }
}

fn opts_print<'a>(text: &'a str, opts: &PrintOpts) -> ColoredString {
    let mut text = match &opts.colour[..] {
        "w" | "white" => text.white(),
        "r" | "red" => text.red(),
        "g" | "green" => text.green(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        "bk" | "black" => text.black(),
        "lw" | "bright_white" => text.bright_white(),
        "lr" | "bright_red" => text.bright_red(),
        "lg" | "bright_green" => text.bright_green(),
        "lb" | "bright_blue" => text.bright_blue(),
        "ly" | "bright_yellow" => text.bright_yellow(),
        "lm" | "bright_magenta" => text.bright_magenta(),
        "lc" | "bright_cyan" => text.bright_cyan(),
        "lbk" | "bright_black" => text.bright_black(),
        _ => panic!("Invalid colour option"),
    };
    text = if opts.bold { text.bold() } else { text };
    text = if opts.italic { text.italic() } else { text };

    text
}

pub fn list_quotes(quotes: std::str::Lines<'_>) {
    println!("Quotes and indices - \n");

    quotes.enumerate().for_each(|(ind, quote)| {
        let n = quote.len();
        // TODO
        let part = if n < 45 {
            &quote[1..n - 2]
        } else {
            &quote[1..45]
        };
        println!(r#"  {}) {}..."#, ind, &part.blue())
    });

    println!("\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote.");

    let cols = "
    r | red
    b | blue
    y | yellow
    g | green
    m | magenta
    c | cyan
    bk | black
    lr | bright_red
    lg | bright_green
    lb | bright_blue
    ly | bright_yellow
    lm | bright_magenta
    lc | bright_cyan
    lbk | bright_black
    lw | bright_white";
    println!("\nColours - \n {}", cols.bright_blue());

    println!("\n Use 'cohle -c <COLOUR>' to print quote in colour");
}
