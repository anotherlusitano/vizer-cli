[![Latest Version]][crates.io] ![License]

[crates.io]: https://crates.io/crates/vizer-cli
[latest version]: https://img.shields.io/crates/v/vizer-cli.svg
[license]: https://img.shields.io/crates/l/vizer-cli.svg

## Installation

Clone the repository and compile from source.

```sh
git clone https://github.com/anotherlusitano/vizer-cli.git
cd vizer-cli/
cargo build --release
```

After that, you will have the program executable inside `./target/release/`

## Usage

```
A cli tool to watch movies, series and animes in Portuguese

Usage: vizer-cli [OPTIONS] <COMMAND>

Commands:
  search, -s  Search for your content
  help        Print this message or the help of the given subcommand(s)

Options:
  -e, --english        Change all the texts in the app to english
  -m, --mpv            Use MPV media player instead of VLC
  -g, --geckodriver    Use geckodriver instead of chromedriver
  -v, --vim            VIM Mode for the enthusiast
  -i, --image-preview  Enable you to see the posters as you choose them
  -h, --help           Print help
  -V, --version        Print version
```

### Examples

Searching content:

```sh
vizer-cli -s attack on titan
```

Using geckodriver and mpv:

```sh
vizer-cli -g -m -s darling
```

Displaying the text in English and using the image-preview feature:

```sh
vizer-cli -e -i -s dragões
```
