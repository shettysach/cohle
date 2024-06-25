## COHLE
Rust CLI that prints Rust Cohle quotes.

![Alt text](image.png)

**Install:**
```console
cargo install --git https://github.com/ShettySach/cohle
```

**Usage**: 
```console
cohle [OPTIONS] [QUOTE_INDEX] [COMMAND]
```

```
Commands:
  quote  Print only quote without image
  image  Print only image without quote
  list   Lists all the quotes and colours along with their indices
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [QUOTE_INDEX]  Index of the quote, defaults to random [default: 24]

Options:
  -m <IMAGE_INDEX>      Index of the image [default: 0]
  -c <COLOUR>           Colour of quote [default: white]
  -b, --bold            Print quote in bold
  -i, --italic          Print quote italicised
  -g, --background      Print image with black background
  -h, --help            Print help
```
