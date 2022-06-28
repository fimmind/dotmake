!!! Attention !!!
This software is still under development and **SHOULD NOT** be considered stable

# dotmake (Makefile-like dotfiles installation manger)

When your Linux configuration files (aka. dotfiles) get comparatively large, you
may realize that you need something that will help you automatically install
them to a new system. At first this might be something as simple a small script
(or a Makefile) that creates soft links and, for example, installs the necessary
packages. But the more configuration is added to your dotfiles directory, the
larger this script becomes alarge nd at some point may end up in a complete mess
(this is what happened to me, personally.) Here is when `dotmake` is meant to
help you.

It provides a convenient way of splitting your configuration into separate
pieces (called `rules`, just like in Makefile) in order to keep everything
organized. Every rule consists of it's own list of actions that specify:

1. rule's dependencies, i.e. other rules that have to be performed prior to the
   given one;
2. a list of soft links to create;
3. a list of packages to install;
4. shell scripts to run;
5. and more actions coming soon... (hopefully)

`dotmake` also provides a couple of other convenient features, such as
[add](#dotmake-add) subcommand. It lets you easily add new files to the dotfiles
directory, with symlinks being created automatically, so that you don't have to
bother doing this manually whenever you crate a new configuration file.

Enjoy!

## Installation

Prebuilt binaries are coming soon. For now you can build `dotmake` from source
using [rustup](https://rustup.rs/) (Don't forget to add `~/.cargo/bin` into your
`PATH`):

```
$ cargo install --git https://github.com/fimmind/dotmake
```

If you don't want to add `~/.cargo/bin` into your `PATH`, you can also specify
an another place for installation (e.g. `~/.local/bin`) using `cargo`'s `--root`
option. Pay attention to the fact that cargo only allows installation into
folders named `bin`. Thus, you have to specify the directory containing `bin`,
not the path to `bin` folder itself. For example, to install `dotmake` into
`~/.local/bin`, you may run

```
$ cargo install --git https://github.com/fimmind/dotmake --root ~/.local
```

## Usage

```
$ dotmake --help
dotmake 0.1.0
Dotfiles installation manager

USAGE:
    dotmake [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help         Prints help information
    -y, --noconfirm    Use default values for confirmation dialogues
    -V, --version      Prints version information

OPTIONS:
    -D, --distro <ID>     Specify distribution id to use
    -d, --dotdir <DIR>    Set a custom dotfiles directory [env: DOTM_DOTFILES_DIR=]  [default: ./]

SUBCOMMANDS:
    add           Move a file to dotfiles directory, replacing it with a symlink
    completion    Generate a completion script for a given shell
    exec          Perform nth action of a given rule
    help          Prints this message or the help of the given subcommand(s)
    install       Perform installation of given rules
```


### Binary

`dotmake` provides a number of subcommands for various purposes:

#### `dotmake add`

Let's say you have just created a new configuration file named `~/.foobar.toml`,
tested that it works correctly and now you want to move it to your dotfiles
directory. Doing this manually would require you to type into your shell at
least two relatively long commands:

``` sh
$ mv ~/.foobar.toml ~/Dotfiles/foobar.toml
$ ln -s ~/Dotfiles/foobar.toml ~/.foobar.toml
```

which may be annoying at times and opens up vast possibilities for making typos.
`dotmake add` shortens this all to a single command:

``` sh
$ dotmake add ~/.foobar.toml -o foobar.toml
```

And that's it! You don't even have to type the path to your dotfiles directory,
in case if you have set `DOTM_DOTFILES_DIR` in your shell's configuration file
(see `--dotdir` option for details).

#### `dotmake completion`

Generate the completion script for one of the supported shells (`bash`, `fish`,
`zsh`, `powershell`, `elvish`) to the standard output. The way this script
should be used depends on your shell of choice. For `fish` this might be
something like

``` sh
$ dotmake completion fish > ~/.local/share/fish/generated_completions/dotmake.fish
```

#### `dotmake install`

This is the major command for the entire application. It takes a list of rules
to perform, resolves dependencies and then performs each of the rules and their
dependencies one by one. For more information about rules' configuration and
structure see [Configuration](#configuration).

#### `dotmake exec`

Since every rule consists of a list of separate actions, it may be useful to be
able to perform only one of them, which is exactly what this subcommand does.
Given the identifier of a rule and a number `n`, it performs the `n`-th action of the
rule (indexing from 1). For example, imagine you have a rule `foo` with the
following structure:

``` yaml
foo:
    - shell: echo actions 1
    - deps: bar buz
    - in_temp: echo $(pwd)
```

Then:
- `dotmake exec foo 1` will print `action 1` on the screen;
- `dotmake exec foo 2` will do nothing, since `deps` is treated as a normal
  action that just does nothing and only specifies the rule's dependencies;
- `dotmake exec foo 3` will print a name of the automatically created temporary
  directory;
- for any other number `dotmake exec foo n` will exit with error, since `foo`
  only has three actions.

### Configuration

More comprehensive documentation is coming soon. For now you can check out an
example configuration file [here](https://github.com/fimmind/Dotfiles/blob/master/dotm-arch.yaml).

## LICENCE

MIT
