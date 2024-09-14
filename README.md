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

To make sure the locations match up, execute `helix --health`.

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

`shell_replace_with_output` mappable action, that acts like `shell_pipe`, but doesn't pipe the selections into the command. So, just execute a command and replace selections with the output.

### Command expansions

When using actions and commands that let you execute a shell command (`:sh`, `shell_insert_output`, `shell_append_output`, `shell_pipe`, etc), you can now use command expansions.

Consider that our current working directory is `~/prog/dotfiles` and the currently open buffer is `~/prog/dotfiles/helix/config.toml`. \
`:sh echo %p` will output `/home/username/prog/dotfiles/helix/config.toml` (full path) \
`:sh echo %h` -> `/home/username/prog/dotfiles/helix` ("head" of the current buffer) \
`:sh echo %w` -> `/home/username/prog/dotfiles` (helix's working directory)

If you want to insert `%p` literally, escape it like: `%%p`. \
`:sh echo %%p` -> `%p`.

If any other character, aside from the ones supported, comes after the `%`, you don't need to escape it, and can use it normally. \
`:sh echo %m` -> `%m`.

This behavior may change in the future, not sure if it's the nicest solution.

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
