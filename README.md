<h3 align="center">
A cli tool to watch movies/series/animes in portuguese. This tool scrapes the site <a href="https://vizer.in">vizer</a>
</h3>
<br>

<h1 align="center">
 Showcase
</h1>

[vizer-cli](https://github.com/anotherlusitano/vizer-cli/assets/108989500/9e4d75a4-7f92-4cb2-acf0-73c3f7d7d93d)

> [!WARNING]  
> This project is not finished. Don't have any high expectations.

## Table of Contents

- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

## Dependencies

> [!IMPORTANT]
> For the program to work, you need two essential things: a webdriver and a media player.
>
> You also need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build the project.

By default, vizer-cli uses [chromedriver](https://developer.chrome.com/docs/chromedriver) and [vlc](https://www.videolan.org/vlc/), but you can also use [geckodriver](https://github.com/mozilla/geckodriver/releases) and [mpv](https://mpv.io/) as an alternative.

You can install the dependencies on Arch Linux using the following command.

```sh
yay -S chromedriver vlc
```

To use the image-preview feature, you need to have [ueberzug](https://github.com/ueber-devel/ueberzug) installed, but you can also use [ueberzugpp](https://github.com/jstkdng/ueberzugpp) if you create an alias with the following command.

```sh
alias ueberzug='ueberzugpp'
```

> [!TIP]
> If you have Chromium installed, you don't need to install chromedriver because it's already installed

## Installation

For now, the only installation method is to compile from source.

```sh
git clone https://github.com/anotherlusitano/vizer-cli.git
cd vizer-cli/
cargo build --release
```

After that, you will have the program executable in `./target/release/`

## Usage

```
CLI tool to watch movies/series/animes in portuguese

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

## Contributing

Contributions are always welcome! To contribute, please follow these steps:

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Use `cargo fmt` and fix all `clippy` warnings
5. Push your changes to your fork
6. Submit a pull request

Or, if you have any features ideas, go to the issues page and post them there
