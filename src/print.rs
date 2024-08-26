use colored::{ColoredString, Colorize};
use textwrap::{fill, termwidth, wrap};

use crate::parse::PrintOpts;

pub fn only_quote(input_text: &str, opts: &PrintOpts) {
    let filled_text = fill(input_text, termwidth());
    println!("{}", opts_print(filled_text.as_str(), opts));
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
    let width = termwidth().checked_sub(56);

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

            let qvec = wrap(quote, value);

            let imlen = imvec.len();
            let qlen = qvec.len();
            let start = imlen.abs_diff(qlen) / 2_usize;

            if imlen >= qlen {
                for imline in imvec.iter().take(start) {
                    println!("{}", imline);
                }

                for i in start..(start + qlen) {
                    println!("{} {}", imvec[i], opts_print(&qvec[i - start], opts));
                }

                for imline in imvec.iter().take(imlen).skip(start + qlen) {
                    println!("{}", imline);
                }
            } else {
                only_image(image, opts);
                println!();
                only_quote(quote, opts);
            }
        }
        None => {
            println!(
                "{}",
                "Terminal width too small to print image.
Resize terminal / reduce font or use 'cohle q'"
                    .bright_black()
            );
            only_quote(quote, opts);
        }
    }
}

fn opts_print(text: &str, opts: &PrintOpts) -> ColoredString {
    let mut text = match &opts.colour {
        'w' => text.bright_white(),
        'r' => text.bright_red(),
        'g' => text.bright_green(),
        'b' => text.bright_blue(),
        'y' => text.bright_yellow(),
        'm' => text.bright_magenta(),
        'c' => text.bright_cyan(),
        'k' => text.bright_black(),
        'W' => text.white(),
        'R' => text.red(),
        'G' => text.green(),
        'B' => text.blue(),
        'Y' => text.yellow(),
        'M' => text.magenta(),
        'C' => text.cyan(),
        'K' => text.black(),
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
        let part = if n < 45 {
            &quote[..n - 2]
        } else {
            &quote[..45]
        };
        println!(r#"  {}) {}..."#, ind, &part.blue())
    });

    println!("\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote.");

    let cols = "
w = bright white
r = bright red
g = bright green
b = bright blue
y = bright yellow
m = bright magenta
c = bright cyan
k = bright black
W = white
R = red
G = green
B = blue
Y = yellow
M = magenta
C = cyan
K = black
";
    println!("\nColours - \n {}", cols.bright_blue());

    println!("\n Use 'cohle -c <COLOUR>' to print quote in colour");
}
