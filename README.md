! ATTENTION !
This software is still under development and most likely it isn't able to do
most of described below yet.

If you have any ideas or suggestions, please, open an issue describing what you
want. I'll be happy to make this software suited for other people too, even
though I design it mainly according to my own needs

# dotmake (Makefile-like dotfiles installation manger)

It will allow you to:

- automate installing configuration to a new system;
- split configuration to separated parts;
- add dependency links inside configuration parts;
- add bootstrap rules for any configuration part, including automated packets
  installation for different package managers and any scripts;
- automatically install disro-independent package managers (i.e. linuxbrew, pip,
  cargo, stack, etc.);
- automate installing from source;
- automatically backup replaced config files after creating soft links to
  installed configuration;
- automate adding new files to tracked configuration (move a given file to
  dotfiles directory and crate soft link pointing at it);

## Installation

As long as there is no stable release, you can build `dotm` binary from source
using [rustup](https://rustup.rs/) (Don't forget to add `~/.cargo/bin` into your
`PATH`):

```sh
$ cargo install --git https://github.com/fimmind/dotmake
```

Another place for installation (e.g. `~/.local/bin`) can also be specified, if
you don't want to add `~/.cargo/bin` into your `PATH` for some reason. Remember
that cargo only allows to install into folders named `bin` and that you have to
specify directory containing `bin`, not the path to `bin` folder itself:

```sh
$ cargo install --git https://github.com/fimmind/dotm --root ~/.local
```

## Usage

***maybe out-of-date.*** See `dotm --help`
```shell
$ dotm --help
dotm 0.1.0
Dotfiles installation manager

USAGE:
    dotm [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help         Prints help information
    -y, --noconfirm    Use default values for confirmation dialogues
    -V, --version      Prints version information

OPTIONS:
    -D, --distro <ID>     Specify distribution id to use [default: <current_distro>]
    -d, --dotdir <DIR>    Set a custom dotfiles directory [env: DOTM_DOTFILES_DIR=]  [default: ./]

SUBCOMMANDS:
    add        Move files to dotfiles directory and create symbolic links pointing at them
    dotdir     Print path Dotfiles' directory
    exec       Perform specified actions for a given rule
    help       Prints this message or the help of the given subcommand(s)
    install    Perform installation of given rules
    pkg        Install package(s) by a given package manager
```

## TODO Configuration

## LICENCE

MIT
