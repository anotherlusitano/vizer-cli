<h3 align="center">
A cli tool to watch movies, series and animes in Portuguese. This tool scrapes the site <a href="https://vizertv.in">vizertv</a>
</h3>
<br>

<h1 align="center">
 Showcase
</h1>

[vizer-cli](https://github.com/user-attachments/assets/6ef85494-1937-4ee3-bc40-a3e656c6ec38)

<p>
    <b>English</b> |
    <a href="https://github.com/anotherlusitano/vizer-cli/blob/main/README-pt.md">Рortuguês</a>
</p>

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [Examples](#examples)
- [Contributing](#contributing)

## Prerequisites

For the program to work, you need two essential components: a webdriver and a media player.

For the webdriver, you can choose between:

- [geckodriver](https://github.com/mozilla/geckodriver)
- [chromedriver](https://developer.chrome.com/docs/chromedriver)

For the media player, you can choose between:

- [vlc](https://www.videolan.org/vlc/)
- [mpv](https://mpv.io/)

> By default, the program uses chromedriver and vlc.

<details><summary>Installation on Arch Linux</summary>
Here are the commands to install all the prerequisites:

```sh
# To install the default
yay -S chromedriver vlc
```

```sh
# To install the alternatives
yay -S geckodriver mpv
```

You can also install the Chromium package instead of the chromedriver package because recent versions of Chromium come with chromedriver.

```sh
sudo pacman -S chromium
```

</details>

<details><summary>Installation on Windows</summary>
Here are the links to install all the prerequisites:
<ul>
  <li>chromedriver: https://developer.chrome.com/docs/chromedriver/downloads</li>
  <li>geckodriver: https://github.com/mozilla/geckodriver/releases</li>
  <li>vlc: https://www.videolan.org/vlc/download-windows.html</li>
  <li>vlc: mpv: https://mpv.io/installation/</li>
</ul>

<b>Make sure all programs are in your PATH!</b>

</details>

To use the image-preview feature, you need to have [ueberzug](https://github.com/ueber-devel/ueberzug) installed, but you can also use [ueberzugpp](https://github.com/jstkdng/ueberzugpp) if you create an alias with the following command.

```sh
alias ueberzug='ueberzugpp'
```

> [!WARNING]
> Ueberzug doesn't support Windows, so you need something like WSL to use the image-preview feature.

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

### Examples

Search content:

```sh
vizer-cli -s attack on titan
```

Use geckodriver and mpv:

```sh
vizer-cli -g -m -s darling
```

Use text in English and the image-preview feature:

```sh
vizer-cli -e -i -s dragões
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
