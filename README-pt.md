<h3 align="center">
Uma ferramenta no terminal que te deixa assistir filmes, series e animes em português. Esta ferramenta faz scraping do site <a href="https://vizertv.in">vizertv</a>
</h3>
<br>

<h1 align="center">
 Demonstração
</h1>

[vizer-cli](https://github.com/user-attachments/assets/6ef85494-1937-4ee3-bc40-a3e656c6ec38)

<p>
    <b>Português</b> |
    <a href="https://github.com/anotherlusitano/vizer-cli/blob/main/README.md">English</a>
</p>

## Tabela de conteúdos

- [Pré-requesitos](#pré-requesitos)
- [Instalação](#instalação)
- [Como usar](#como-usar)
  - [Exemplos](#exemplos)
- [Contribuir](#contribuir)

## Pré-requesitos

Para este programa funcionar, precisas de ter duas coisas essenciais: um webdriver e um reprodutor de media.

Para o webdriver, podes escolher entre:

- [geckodriver](https://github.com/mozilla/geckodriver)
- [chromedriver](https://developer.chrome.com/docs/chromedriver)

Para o reprodutor de media, podes escolher entre:

- [vlc](https://www.videolan.org/vlc/)
- [mpv](https://mpv.io/)

> Por padrão, o programa usa o chromedriver e o vlc.

<details><summary>Instalação no Arch Linux</summary>
Aqui estão os comandos para instalar todos os pré-requesitos:

```sh
# Para instalar o padrão
yay -S chromedriver vlc
```

```sh
# Para instalar as alternativas
yay -S geckodriver mpv
```

Também podes instalar o pacote do Chromium em vez do pacote do chromedriver, porque as versões recentes do Chrmium trazem o chromedriver.

```sh
sudo pacman -S chromium
```

</details>

<details><summary>Instalação no Windows</summary>
Aqui os links para instalar os pré-requesitos:
<ul>
  <li>chromedriver: https://developer.chrome.com/docs/chromedriver/downloads</li>
  <li>geckodriver: https://github.com/mozilla/geckodriver/releases</li>
  <li>vlc: https://www.videolan.org/vlc/download-windows.html</li>
  <li>vlc: mpv: https://mpv.io/installation/</li>
</ul>

<b>Tem a certeza que todos os programas fazem parte das variáveis do sistema!</b>

</details>

Para usar a funcionalidade de image-preview, precisas de ter o programa [ueberzug](https://github.com/ueber-devel/ueberzug) instalado, mas também podes usar o programa [ueberzugpp](https://github.com/jstkdng/ueberzugpp) se criares uma alias com o seguinte comando.

```sh
alias ueberzug='ueberzugpp'
```

> [!WARNING]
> Ueberzug não suporta o Windows, então precisas de instalar algo como o WSL para utilizar a funcionalidade de image-preview.

## Instalação

Entra na [página de lançamentos](https://github.com/anotherlusitano/vizer-cli/releases/tag/v1.0.0) e descarrega o executável. Depois disso, podes executar o executável no terminal com o seguinte comando.

```sh
# Para o Linux precisas de primeiro dar permissão de escrita
chmod +x vizer-cli
./vizer-cli

# Windows
.\vizer-cli
```

Também podes usar o `cargo` para instala-lo.

```sh
cargo install vizer-cli
```

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
