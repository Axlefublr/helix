# This is my helix fork

To use it, you would have to build this repository from source.

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
