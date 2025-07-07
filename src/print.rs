use colored::{ColoredString, Colorize};
use textwrap::{fill, termwidth, wrap};

use crate::{parse::PrintOpts, CmdError};

pub fn only_quote(input_text: &str, opts: &PrintOpts) -> Result<(), CmdError> {
    let filled_text = fill(input_text, termwidth());
    let formatted_text = opts_print(filled_text.as_str(), opts)?;
    println!("{formatted_text}");
    Ok(())
}

pub fn only_image(image: &str, opts: &PrintOpts) {
    if opts.background {
        for imline in image.lines() {
            println!("{}", imline.on_black());
        }
    } else {
        println!("{image}");
    };
}

pub fn quote_image(image: &str, quote: &str, opts: &PrintOpts) -> Result<(), CmdError> {
    let (img_length, img_width) = (29_usize, 55_usize);
    let width = termwidth().checked_sub(img_width + 1);

    match width {
        Some(value) => {
            let qvec = wrap(quote, value);
            let qlen = qvec.len();
            let start = img_length.abs_diff(qlen) / 2;

            if img_length >= qlen {
                for (i, imline) in image.lines().enumerate() {
                    let imline = if opts.background {
                        imline.on_black()
                    } else {
                        imline.clear()
                    };

                    if start <= i && i < start + qlen {
                        let qline = opts_print(&qvec[i - start], opts)?;
                        println!("{imline} {qline}");
                    } else {
                        println!("{imline}");
                    }
                }

                Ok(())
            } else {
                only_image(image, opts);
                println!();
                only_quote(quote, opts)
            }
        }
        None => {
            println!(
                "{}",
                "Terminal width too small to print image.".bright_black()
            );
            only_quote(quote, opts)
        }
    }
}

fn opts_print(text: &str, opts: &PrintOpts) -> Result<ColoredString, CmdError> {
    let mut text = match opts.colour {
        'w' => Ok(text.normal()),
        'r' => Ok(text.bright_red()),
        'g' => Ok(text.bright_green()),
        'b' => Ok(text.bright_blue()),
        'y' => Ok(text.bright_yellow()),
        'm' => Ok(text.bright_magenta()),
        'c' => Ok(text.bright_cyan()),
        'k' => Ok(text.bright_black()),
        'W' => Ok(text.white()),
        'R' => Ok(text.red()),
        'G' => Ok(text.green()),
        'B' => Ok(text.blue()),
        'Y' => Ok(text.yellow()),
        'M' => Ok(text.magenta()),
        'C' => Ok(text.cyan()),
        'K' => Ok(text.black()),
        invalid => Err(CmdError::InvalidColour(invalid)),
    }?;

    if opts.bold {
        text = text.bold();
    }
    if opts.italic {
        text = text.italic();
    }

    Ok(text)
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
