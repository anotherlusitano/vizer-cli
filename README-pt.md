<h3 align="center">
Uma ferramenta no terminal que te deixa assistir filmes/series/animes em português. Esta ferramenta faz scraping do site <a href="https://vizertv.in">vizertv</a>
    <p>
        <b>Português</b> |
        <a href="https://github.com/anotherlusitano/vizer-cli/blob/main/README.md">English</a>
    </p>
</h3>
<br>

<h1 align="center">
 Demonstração
</h1>

[vizer-cli](https://github.com/anotherlusitano/vizer-cli/assets/108989500/9e4d75a4-7f92-4cb2-acf0-73c3f7d7d93d)

## Tabela de conteúdos

- [Dependências](#dependências)
- [Instalação](#instalação)
- [Como usar](#como-usar)
  - [Exemplos](#exemplos)
- [Contribuir](#contribuir)

## Dependências

> [!IMPORTANT]
> Para este programa funcionar, tu precisas de ter duas coisas essenciais: um webdriver e um reprodutor de media.
>
> Vais precisar do [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) para instalar o programa.

Por padrão, o vizer-cli usa o [chromedriver](https://developer.chrome.com/docs/chromedriver) e o [vlc](https://www.videolan.org/vlc/), mas também podes usar o [geckodriver](https://github.com/mozilla/geckodriver/releases) e o [mpv](https://mpv.io/) como alternativa.

Para instalar as dependências no Arch Linux, usa o seguinte comando.

```sh
yay -S chromedriver vlc
```

Para usar a funcionalidade de image-preview, precisas de ter o programa [ueberzug](https://github.com/ueber-devel/ueberzug) instalado, mas também podes usar o programa [ueberzugpp](https://github.com/jstkdng/ueberzugpp) se criares uma alias com o seguinte comando.

```sh
alias ueberzug='ueberzugpp'
```

> [!TIP]
> Se já tiveres o Chromium instalado, não precisas de instalar o chromedriver porque ele já vem instalado.

## Instalação

Por agora, a única forma de instalar é a compilar pela fonte.

```sh
git clone https://github.com/anotherlusitano/vizer-cli.git
cd vizer-cli/
cargo build --release
```

Depois disso, vais ter o executável do programa em `./target/release/`

## Como usar

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

### Exemplos

Procurar por conteúdo:

```sh
vizer-cli -s attack on titan
```

Usar geckodriver e mpv:

```sh
vizer-cli -g -m -s darling
```

Usar texto em inglês e usar a funcionalidade de image-preview:

```sh
vizer-cli -e -i -s dragões
```

## Contribuir

Contribuições são sempre bem-vindas! Para contribuir, por favor segue os seguintes passos:

1. Faz uma fork do repositório
2. Cria um novo ramo
3. Faz as alterações necessárias
4. Usa `cargo fmt` e resolve todos os avisos do `clippy`
5. Dá push das alterações para a tua fork
6. Submete uma pull request

Ou, se tiveres alguma ideia para uma nova funcionalidade, vai à página das issues e posta ela lá.
