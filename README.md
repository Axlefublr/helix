# This is my helix fork

And it is, first and foremost, a personal fork. \
I am one person, and will only implement features that *I* need, to minimize the cost of maintenance.

There is a small chance I'll add your requested feature, but that chance is very small; You will have more luck adding it yourself, to your own fork.

Ah yes, the wonderful plugin ecosystem of helix: just making a fork and modifying the source 💀

I will arbitrarily add breaking changes, either to the changes already made by this fork, or to core.
Autoupdating this fork is a bad idea, consequently. It would save your mental sanity to check the commits from time to time to see what changed, and decide to recompile again once there are features you want here, rather than trusting things to just continue working in the same way indefinitely.

There is one single place where all of the fork's changes are going to be listed — this readme! In the "This fork's changes" section you'll see below. Once you read it once, you'll be able to skim through it from time to time to see what changed; completely new features will appear at the bottom, sometimes in new sections.

This fork is based on helix master, not stable. \
I will be rebasing on upstream master every time I see a feature I want from there. \
Considering just how awfully slow helix development is in upstream, this might not happen often, but I *am* keeping a look at the new commits in master.

If there's a feature from master that you want to get and I haven't yet rebased on it, you can ask me in an issue, describing why you want that feature. I'm sorry for making that requirement, but I really want to minimize my effort maintaining this, so will only rebase when *I* don't need to, if it noticeably improves someone else's experience.

To conclude: this fork can be useful to you, if you want the features it implements, but I wouldn't say it's a fork you can *rely* on, like you would be able on upstream stable. One thing I can promise you though, is that I'm obscenely obsessed with configuring my editor, and so I will maintain this for as long as I use helix and as long as it doesn't have every single feature I want, lol. So the situation of the latest commit being 5 months ago, that I saw with another helix fork, is most likely not going to happen.

You can join my [discord server](https://discord.gg/bgVSg362dK) if you want to hear me glaze helix (and talk about features I'm currently implementing).

If you want to contact me on discord without joining a server, my username is `axlefublr`.

## Installation

To use this fork, you would have to build this repository from source.

First, `cd` into some place where you would like to keep this repository, so that you can easily update in the future. \
I recommend to also [read the docs](https://docs.helix-editor.com/master/building-from-source.html) on compiling helix from source, to make sure everything goes smoothly. \
Then, execute this in your (linux) shell:

```sh
git clone --depth 1 https://github.com/Axlefublr/helix
cd helix
cargo install --path helix-term --locked
mkdir -p ~/.cargo/bin/
rm -fr ~/.cargo/bin/runtime
ln -sf $PWD/runtime ~/.cargo/bin/
```

Because I make a symlink, the runtime directory that helix requires gets updated automatically. \
When files in the `runtime/` directory of this repository (your locally stored copy) change, they change in the place where helix expects them, too.

To make sure the locations match up, see the output of `helix --health`.

The full path to the binary you'll get is `~/.cargo/bin/hx`, so you will be able to use `hx` in your shell if `~/.cargo/bin` is in your $PATH (it likely already is). \
The `helix` package on arch actually gives you the executable `helix`, rather than `hx`. Here you will get `hx` even if you are on arch.

In the future, when you want to update, you would:

```sh
git pull
cargo install --path helix-term --locked
```

## This fork's changes

Things like hover docs, pickers, and possibly other various popups have <kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> hardcoded to mean "scroll by half a page down / up" \
I change <kbd>ctrl+d</kbd> to be <kbd>alt+l</kbd> and <kbd>ctrl+u</kbd> to be <kbd>alt+h</kbd>. \
I *replace* them specifically, because by default, <kbd>ctrl+u</kbd> overrides the very useful "delete until the start of the line" mapping. \
So this change effectively also adds that mapping to pickers.

Adds hotkey to toggle preview in a picker: <kbd>alt+;</kbd> (default is <kbd>ctrl+t</kbd>).

In command mode (and other tab-completable prompts), <kbd>alt+;</kbd> acts the same as <kbd>Tab</kbd>.

In prompts (command mode, pickers, etc), <kbd>alt+,</kbd> moves you to the start of the line and <kbd>alt+.</kbd> to the end, like <kbd>ctrl+a</kbd> and <kbd>ctrl+e</kbd> also do.

`:random` command with aliases `:rnd`, `:rng` that randomizes your selections.

`whichkey` option in the `[editor]` section can be set to `true` (default) or `false`. \
If set to `false`, the infoboxes for *mappings* will not show up. \
This is different from just disabling the `auto-info` option in that you will still get the popup for `select_register`.

`should-statusline` option in the `[editor]` section can be set to `false` to disable the statusline completely. \
This exists because in default helix, even if you have no statusline elements in your statusline configuration, a default set of elements is drawn, rather than removing the statusline. \
This option is designed to be disabled in your config, but you can set it on runtime as well (surprisingly). \
It will look wonky if you do, so it makes the most sense to make a hotkey to toggle the statusline when / if you suddenly realize you want to see information in it (while not caring about it in the general case).

`ephemeral-messages` option in the `[editor]` section can now be set to `true` to make status / error messages at the bottom of the UI *not* take up an entire line. Instead, they will be printed over the editor view directly.

`:echo` command lets you print a message to the messages line. Useful for when you want to give visual feedback to some mappings, or to use [command expansions](#command-expansions).

`:echopy` command is exactly like `:echo`, but *also* puts the result into your clipboard. For example, you can do `:echopy %p` to copy the full path to the current file to your clipboard (this is elaborated on [later](#command-expansions)).

All pickers now take up the entire screen, rather than 90%. Thanks to @satoqz for figuring out how to do this! :D

The `insert-final-newline` option now only inserts newline if the file is not empty.

`:run-shell-command` now also has a `:s` alias, along with `:sh`.

### Implemented PRs from upstream

* [11234](https://github.com/helix-editor/helix/issues/11234) by @Swordelf2
* [9143](https://github.com/helix-editor/helix/pull/9143) by @intarga
* [2608](https://github.com/helix-editor/helix/pull/2608) by @Philipp-M
* [11285](https://github.com/helix-editor/helix/pull/11285) by @drybalka

### Command expansions

Supported in: `shell_pipe`, `shell_pipe_to`, `shell_insert_output`, `shell_append_output` and **all** `:command`s. I'm not yet sure if the latter is a good idea, but feel free to `:cd %p` if you wish /j.

Example usage: `:sh echo %p`.

Considering our example context...

|Thing                    |Value                                            |
|-------------------------|-------------------------------------------------|
|current buffer           |`~/prog/dotfiles/fish/abbreviations/percent.fish`|
|current working directory|`~/prog/dotfiles/fish`                           |
|git repo root            |`~/prog/dotfiles`                                |

...here's what all command expansions would evaluate to:

|Expansion|Output                                                        |Explanation                             |
|---------|--------------------------------------------------------------|----------------------------------------|
|`%p`     |`/home/username/prog/dotfiles/fish/abbreviations/percent.fish`|full path                               |
|`%h`     |`/home/username/prog/dotfiles/fish/abbreviations`             |"head" of the current buffer            |
|`%w`     |`/home/username/prog/dotfiles/fish`                           |helix's working directory (can be different from the directory you started helix in)|
|`%g`     |`/home/username/prog/dotfiles`                                |git repo root                           |
|`%r`     |`abbreviations/percent.fish`                                  |filepath, relative to cwd (full path, if not inside cwd)|
|`%q`     |`fish/abbreviations/percent.fish`                             |filepath, relative to git repo root (full path, if not inside git repo root)|
|`%n`     |`percent.fish`                                                |basename of the current buffer          |
|`%e`     |`fish`                                                        |extension                               |
|`%l`     |`fish`                                                        |helix's language option (output of `:lang`)|
|`%m`     |`%m`                                                          |not an expansion, taken literally       |
|`%%p`    |`%p`                                                          |escaped using `%%` to be taken literally|

All expansions have an uppercase variant (`%P`, `%H`, `%W`, etc) that replaces `/home/username` with `~`. \
The reason they exist is because I disable the statusline, and make a bunch of mappings that use `:echo` to print a path to the statusline. \
Seeing `/home/username` all the time would feel quite bloaty. \
If you find another use for them, nice! But in the general case you'll want to use the lowercase variants.

> [!CAUTION]
> The resulting path is not escaped in any way.
> If it contains spaces, that *may* be a problem, depending on what command you're using the expansion in.
> You may want to quote expansions, in that case.

### Harp integration

Inspired by [`harp-nvim`](https://github.com/Axlefublr/harp-nvim), implemented using the [`harp`](https://github.com/Axlefublr/harp) library.

A "harp" is essentially a storage unit. It lets you store some information from the editor to then use later. \
Harps are *persistent*. Once you set a harp, it stays forever (until overwritten by you) and gets shared across helix sessions. \
Even if you have multiple helix sessions open at a time, if you set a harp in one session, it will *immediately* become available in all sessions.

Harp "sections" exist to organize multiple different sets of harps. \
If all harps were stored in a single place, that would lead to name collisions: if you set a [file harp](#file-harps) `a`, you wouldn't be able to set a [register harp](#register-harps) `a` — the latter would override the former. \
So, all harp types are stored within their own "harp section", letting you use the same harp names without name collisions.

Each harp type has two actions: `get` and `set`. \
Both of them place you into an input field to type the name of the harp into. \
`set` takes some information from the environment (for example, the current buffer's filepath), and stores it in a harp.
`get` takes that information from a harp, and applies it somehow (for example, `:open`s the stored filepath).

The main idea of all harp types, is to let you store information by aliasing it: \
Instead of typing in a long file path, search pattern, or plain text, you can *store* it under a shorter, and more convenient alias.

#### File harps

```
harp_file_set
harp_file_get
```

`set` takes the *current buffer*'s filepath, and stores it in a harp. \
`get` takes it, and `:open`s it.

#### Relative file harps

```
harp_relative_file_set
harp_relative_file_get
```

`set` takes the current buffer's filepath, and stores it in a harp. \
HOWEVER, it stores only the part of the full path, that's relative to current working directory.

Say your current buffer path is `~/prog/dotfiles/colors.css` and your current working directory is `~/prog/dotfiles`. \
If you use a normal [file harp](#file-harps), you will store the full path: `~/prog/dotfiles/colors.css`. \
If you use a *relative* file harp, you will store the *relative* path: `colors.css`.

So then, the `get` action will just open that path relatively — as if you did `:o colors.css`. This will end up opening a different file depending on your current working directory.

The design idea behind this, is to store paths that repeat in *project structures*.

Look at these paths for example: `.gitignore`, `src/main.rs`, `src/lib.rs`, `Cargo.toml`, `.git/info/exclude`, `README.md`, `CONTRIBUTING.md` \
All of these tend to repeat in a lot of projects — they're not particularly unique paths.
So it doesn't make sense to store them in normal file harps, that are *designed* for unique paths. \
Instead with relative file harps, you get to efficiently refer to "the same file", which ends up being a different *actual* file depending on your current working directory.

#### Cwd harps

```
harp_cwd_set
harp_cwd_get
```

`set` stores your current working directory in a harp, `get` `:cd`s into a stored working directory.

#### Search harps

```
harp_search_set
harp_search_get
```

`set` takes your latest search pattern from register `/` and stores it in a harp. \
`get` takes a stored search pattern, and puts it back into register `/`, effectively "making a search".

#### Register harps

```
harp_register_set
harp_register_get
```

`set` puts the contents of your default register (`"`) into a harp. \
`get` puts the stored text back into your default register (`"`)

If you use `set` while having multiple selections, they are joined into a single one with newlines.

#### Command harps

```
harp_command_set
harp_command_get
```

`set` puts your most recent command mode command (register `:`) into a harp. \
`get` executes a stored command mode command.

Supports command expansions! :3

#### Relativity

Now that you're familiar with all the harp types, let me introduce you to the feature of relativity.

Normally-named harps are "global" harps. Harps that are not relative to anything.

If a harp name starts with a `.`, it becomes relative to your current working directory. \
If starts with `,`, relative to the current buffer. \
If starts with `;`, relative to the filetype (run `:lang` to check the filetype of the current buffer).

Remember how different harp types are stored in different sections to fight against name collisions? \
The same thing happens here: "relativity" is made by appending the directory path / buffer path / filetype onto the name of each section.

This way, you can have "global" file harps, but also file harps that are specific to the current project you're working on.

Useful global searches like `(TODO|FIXME|HACK|MOVE):?`, and buffer-specific searches like `// asdf I left off here`.

Project-specific register harps, as a way to gain register session persistence, and filetype-specific register harps, that can act as a basic snippet implementation.

When using harp relativity, you may eventually notice that you *mostly* want a certain relativity for a given harp type: global searches are rarer to want compared to project local ones, for example.

You can actually change the default relativity, from "global"!

When using a harp (whether `get` or `set`, doesn't matter), if you *just* supply `.` / `,` / `;` / `'` as your harp name (without anything afterwards), you will *set* the default relativity for *that* harp type only.

`'` sets the default relativity back to "global", as you may have guessed. I omitted it above for clarity, but you can use `'` at the start of your harp names to override relativity to be global.

The workflow goes like this:
1. use a harp action
2. enter just `,`
3. now this harp action is relative to the current buffer by default. this stays forever, until you override it
4. use it again, now entering `a`
5. you used what is equivalent to `,a`, but without having to type in `,`, because you changed the default relativity
6. use it again, now entering `'a`
7. you just used the *global* relativity, overriding the (new) default of buffer-relative
8. use it again, now entering *just* `'`
9. you set default relativity *back* to "global"

How you use relativity is up to you! In some cases relativity doesn't make sense logically, but this approach lets me implement flexible functionality that *you* may, in some cases, use in ways that I didn't think of.

##### Exceptions

One of those cases is `.` relativity in cwd harps.

Cwd harps take your *current working directory* automatically, as their input. \
`.` relativity does as well. \
If you ever try to set a `.` relative cwd harp, all you'll be able to do is `:cd` into a directory that you already are in.

In other words, `.` relativity in cwd harps is useless. Useless enough for me to introduce a certain inconsistent behavior, to make the feature more useful.

If you try to use `.` relativity while doing `harp_cwd_set`, instead of taking your current working directory like it usually would, it takes the *parent* directory of the *current buffer*.

---

<div align="center">

<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="logo_dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="logo_light.svg">
  <img alt="Helix" height="128" src="logo_light.svg">
</picture>
</h1>

[![Build status](https://github.com/helix-editor/helix/actions/workflows/build.yml/badge.svg)](https://github.com/helix-editor/helix/actions)
[![GitHub Release](https://img.shields.io/github/v/release/helix-editor/helix)](https://github.com/helix-editor/helix/releases/latest)
[![Documentation](https://shields.io/badge/-documentation-452859)](https://docs.helix-editor.com/)
[![GitHub contributors](https://img.shields.io/github/contributors/helix-editor/helix)](https://github.com/helix-editor/helix/graphs/contributors)
[![Matrix Space](https://img.shields.io/matrix/helix-community:matrix.org)](https://matrix.to/#/#helix-community:matrix.org)

</div>

![Screenshot](./screenshot.png)

A [Kakoune](https://github.com/mawww/kakoune) / [Neovim](https://github.com/neovim/neovim) inspired editor, written in Rust.

The editing model is very heavily based on Kakoune; during development I found
myself agreeing with most of Kakoune's design decisions.

For more information, see the [website](https://helix-editor.com) or
[documentation](https://docs.helix-editor.com/).

All shortcuts/keymaps can be found [in the documentation on the website](https://docs.helix-editor.com/keymap.html).

[Troubleshooting](https://github.com/helix-editor/helix/wiki/Troubleshooting)

# Features

- Vim-like modal editing
- Multiple selections
- Built-in language server support
- Smart, incremental syntax highlighting and code editing via tree-sitter

Although it's primarily a terminal-based editor, I am interested in exploring
a custom renderer (similar to Emacs) using wgpu or skulpin.

Note: Only certain languages have indentation definitions at the moment. Check
`runtime/queries/<lang>/` for `indents.scm`.

# Installation

[Installation documentation](https://docs.helix-editor.com/install.html).

[![Packaging status](https://repology.org/badge/vertical-allrepos/helix.svg?exclude_unsupported=1)](https://repology.org/project/helix/versions)

# Contributing

Contributing guidelines can be found [here](./docs/CONTRIBUTING.md).

# Getting help

Your question might already be answered on the [FAQ](https://github.com/helix-editor/helix/wiki/FAQ).

Discuss the project on the community [Matrix Space](https://matrix.to/#/#helix-community:matrix.org) (make sure to join `#helix-editor:matrix.org` if you're on a client that doesn't support Matrix Spaces yet).

# Credits

Thanks to [@jakenvac](https://github.com/jakenvac) for designing the logo!
