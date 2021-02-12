!!! Attention !!!
This software is still under development and **SHOULD NOT** be considered stable

# dotmake (Makefile-like dotfiles installation manger)

When your linux configuration files (aka. dotfiles) grow bigger in size, you
realize that you need something that will help you automatically install them to
a new system. At first this might be something like a simple script (or a
Makefile) that creates soft links, installs needed packages and so on. But as
you add more and more configuration to your dotfiles directory, this script gets
bigger and bigger and at some point you realize that it's just a complete mess
(at least that's what happened to me). Here is when `dotmake` comes to help you.

It gives you a convenient way of splitting your configuration into separate
pieces (called `rules`, just like in Makefile) to keep everything organized.
Every rule consists of it's own list of actions that specify:

1. rule's dependencies, i.e. other rules that have to be installed before the
   given one;
2. a list of soft links to make;
3. a list of packages to install;
4. shell scripts to run;
5. and more actions coming soon...

On the other hand it provides a couple of other convenient features, like
[add](#dotmake-add) subcommand that lets you easily add new files to dotfiles
directory automatically creating symlinks pointing at them so that you don't
have to bother yourself doing this manually every time you crate a new config
file.

Enjoy!

## Installation

Prebuilt binaries are coming soon. For now you can build `dotmake` binary from
source using [rustup](https://rustup.rs/) (Don't forget to add `~/.cargo/bin`
into your `PATH`):

```
$ cargo install --git https://github.com/fimmind/dotmake
```

You can also specify an another place for installation (e.g. `~/.local/bin`), if
you don't want to add `~/.cargo/bin` into your `PATH` for some reason. Remember
that cargo only allows to install into folders named `bin` and that you have to
specify directory containing `bin`, not the path to `bin` folder itself. For
example, to install `dotmake` into `~/.local/bin`, run:

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

Dotmake provides a couple of subbcommands for different purposes:

#### `dotmake add`

Let's say you've just created a new config file named `~/.foobar.toml`, tested
that it works and now you want to move it to your dotfiles directory. Doing this
manually would require you to type into your terminal at least two relatively
long commands like:

``` sh
$ mv ~/.foobar.toml ~/Dotfiles/foobar.toml
$ ln -s ~/Dotfiles/foobar.toml ~/.foobar.toml
```

which is pretty annoying sometimes. Using `dotmake` all this gets shortened to a
single command:

``` sh
$ dotmake add ~/.foobar.toml -o foobar.toml
```

And that's it! You don't even have to type path to your dotfiles directory, in
case if you have set `DOTM_DOTFILES_DIR` in your shell's config (see `--dotdir`
option).

#### `dotmake completion`

Generate completion script for one of the supported shells (`bash`, `fish`,
`zsh`, `powershell`, `elvish`) to stdout. The way you use it depends on concrete
shell. For `fish` this might be something like:
``` sh
$ dotmake completion fish > ~/.local/share/fish/generated_completions/dotmake.fish
```

#### `dotmake install`

This is a major command for the whole application. It takes a list of rules to
perform, resolves dependencies and then performs each of the resolved rules one
by one. For more information about rules' configuration and structure see
[Configuration](#configuration).

#### `dotmake exec`

Since every rule consists of a list of separate actions, it may sometimes be
useful to be able to perform only one of them, and this is exactly what this
subcommand does. You give it a rule identifier and a number, and it performs nth
action of the given rule (indexing from 1). For example, let's say you have a
rule `foo`:

``` yaml
foo:
    - shell: echo actions 1
    - deps: bar buz
    - in_temp: echo $(pwd)
```

And now:
- `dotmake exec foo 1` will print `action 1` on the screen;
- `dotmake exec foo 2` will do nothing, since `deps` is treated as a normal
  action that just does nothing and instead only specifies rule's dependencies;
- `dotmake exec foo 3` will print a name of automatically created temporal
  directory;
- for any other number `dotmake exec foo n` will exit with error, since `foo`
  only has three actions.

### Configuration

More comprehensive documentation is coming soon.

## LICENCE

MIT
