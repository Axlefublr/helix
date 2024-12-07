# Setting expectations

This is a passion project. And I tend to only have passion towards solving my own problems.

If you want a certain feature added, ask in an issue first — there's a good chance I'll reject a PR if I don't personally need its changes, as that turns into my responsibility to maintain.

**I will arbitrarily add breaking changes**, either to the changes already made by this fork, or to core. \
Autoupdating this fork is a bad idea, consequently. \
Instead, check what changed from time to time (the contents of this readme) to decide when you want to update.

The commit history will be optimized for less head pain. \
What does that mean?

Commits don't reflect the change history: they are a set of patches. As I change things, I'll be deleting / amending (modifying) those commits, instead of adding new ones on top. \
With this simple commit history, merge conflicts will be way easier to resolve.

In other words, don't rely on commit hashes to stay valid, as every change will likely change most of them (out of my commits, not upstream's)

This fork is based on helix master, not stable. \
I will be rebasing on upstream master every time I see a feature I want from there. \
Considering just how awfully slow helix development is in upstream, this might not happen often, but I *am* keeping a look at the new commits in master.

Every time I rebase, or make some significant changes, I'll create a backup branch that I will also push to this repo. \
I make a backup branch after confirming all my new changes work, and stop updating the previous backup branch. \
I'll keep updating the now current backup branch for changes that are small. On the next big change, I create the next backup branch, and stop updating the current one.

They're meant as a safeguard for me if something goes wrong, but also can be used by you to extract a feature that you like, that I happened to delete. \
As far as I can tell, literally nobody except me uses this fork, so if you want some specific feature to stay, tell me that it's important to you via an issue — then I'll think twice before deleting it (but still might).

Join my [discord server](https://discord.gg/bgVSg362dK) if you want to follow the development of the fork.

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

All pickers now take up the entire screen, rather than 90%. Thanks to @satoqz for figuring out how to do this! :D

In default helix, there's one space on each side of each buffer name; so the first buffer's ending space and the second buffer's starting space ends up being 2 spaces together. This means that there's this awkward space before the entire bufferline, instead of being flush with the absolute left of the editor. \
This change places *two* spaces *after* each buffer, making the 2-sized gap still remain, but without the awkward leading space. \
`justify-content: space-between` rather than `space-around`, basically.

On write, the contents of the buffer are piped into `helix-piper`¹ and are replaced with its output². \
You're meant to have `helix-piper` as a symlink to some formatter binary that you want to run for *everything*. \
Unfortunately helix doesn't have a way to:
1. set a formatter for every language
2. set more than one formatter for a given language
3. set a formatter (or lsp) for the default, "text" language \
So this is why I came up with this workaround. \
Beware: due to implementation details, std**err** is considered valid output. \
If your formatter prints an error to stderr, you'll end up seeing your entire buffer replaced with that error message. \
This can be destructive if you use `:write-all` while not looking at that buffer, and then close helix without realizing you demolished some buffer. Food for thought.

¹ If you have it in `$PATH`

² If that output is different from the input text. If the input and output are the same, the document doesn't get touched. \
**Unless** the document is "too large". Don't ask me! I don't know either! This optimization comes from the issue of doing a nothing burger write on scratch / empty buffers; if it happens to help other cases that's cool too but ultimately not the main goal. \
But if you're genuinely interested in the implementation detail, "too large" happens when the string of the document isn't laid out in contiguous memory. Which I can't imagine being practically appliable information 👍

### Implemented PRs from upstream

|Author|Link|Title|
|------|----|-----|
|@Swordelf2      |[11234](https://github.com/helix-editor/helix/issues/11234)|Add reload! and reload-all! commands|
|@scdailey       |[9483](https://github.com/helix-editor/helix/pull/9483)    |Implement Kakoune's A-S: Select first and last chars|
|@NikitaRevenco  |[12241](https://github.com/helix-editor/helix/pull/12241)  |Display long status messages as popups|
|@pantos9000     |[10576](https://github.com/helix-editor/helix/pull/10576)  |Add same_line and anchored movements|
|@jakesactualface|[5749](https://github.com/helix-editor/helix/pull/5749)    |Adds support for right-hand gutters and Scrollbar gutter item|
|@oxcrow         |[13053](https://github.com/helix-editor/helix/pull/13053)  |Add local search in buffer (fuzzy and regex)|
|@the-mikedavis  |[13206](https://github.com/helix-editor/helix/pull/13206)  |Word completion|

Big shoutout to these people!

### Changed behavior of builtin actions

`buffer_picker` shows the current buffer last.

`indent` action now works even on empty lines.

### Hardcoded mappings

Hover docs, pickers, and possibly other various popups have <kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> hardcoded to mean "scroll by half a page down / up" \
I remove them because <kbd>ctrl+u</kbd> overrides the very useful "delete until the start of the line" mapping. \
Not being able to delete the entire text in a picker at once is very painful. \
You can use <kbd>PageDown</kbd> and <kbd>PageUp</kbd> instead.

#### prompts (command mode, pickers, etc)

<kbd>ctrl+v</kbd> pastes the contents of your `default-yank-register`. A shortcut to <kbd>ctrl+r \<register\></kbd>, basically.

#### pickers

Due to <kbd>ctrl+v</kbd> now pasting, the hotkey to open the result in a vertial split is moved from <kbd>ctrl+v</kbd> to <kbd>ctrl+m</kbd>.

By default, <kbd>Home</kbd> and <kbd>End</kbd> jump to the first / last result, in the list of results. \
This once again overshadows very useful hotkeys, that let you go to the start/end of the line (your picker input) :/ \
Now, <kbd>ctrl+Home</kbd> and <kbd>ctrl+End</kbd> are used instead.

### New commands

`:random` randomizes the order of your selections. Has aliases `:rnd`, `:rng`.

`:echopy` command is exactly like `:echo`, but *also* puts the result into your `default-yank-register`. Has alias `:cc`.

`:buffer-delete-file` (with aliases `db`, `del`, `delete`) deletes the *current* buffer's real file, and also forcefully closes the buffer. You can think of it as `:sh rm %{buffer_name}` + `:buffer-close!` in a single command.

`:buffer-nth` (with alias `bi`) lets you travel to the nth buffer, out of those you have open currently. The `-r` flag counts buffers from the end, rather than from the start. \
Reason why I didn't make it just use negative numbers, is because they are attempted to be interpreted as flags, and so you have to do `-- -1`. `-r 1` is slightly nicer.

### New actions (except [harp](#harp-integration))

`count_selections` tells you how many selections you have. Only really useful if you disable the statusline.

`toggle_line_select` does `trim_selections` if any of your selections end in a newline, while not *all* of them being single column selections that only contain a newline. \
If all of your selections are single column selections that are all newlines, or some of your selections don't end in a newline, does `extend_to_line_bounds`. \
A bit of a confusing definition, but in effect this means that this action will try its best to expand all of your selections to the whole line, and will only `trim_selections` when all of your selections are whole line selections. \
The whole "single column selections that are newlines" business is needed to handle the case where you position your cursor on a newline, while intending to select that entire line.

`surround_add_tag` prompts you with the name of an html tag, and surrounds your selections with it. \
You have `word`, type in `div`, and get `<div>word</div>`. \
The history for it is stored in the `<` register.

`goto_first_selection` and `goto_last_selection` make the first / last selection to be your primary selection.

### New options

#### `[editor]` section

`whichkey` can be set to `true` (default) or `false`. \
If set to `false`, the infoboxes for *mappings* will not show up. \
This is different from just disabling the `auto-info` option in that you will still get the popup for `select_register`.

`should-statusline` can be set to `false` to disable the statusline. \
This exists because in default helix, even if you have no statusline elements in your statusline configuration, a default set of elements is drawn, rather than removing the statusline. \
This option is designed to be disabled in your config, but you can change it on runtime as well. \
It will look wonky if you do, so it makes the most sense to make a hotkey to toggle the statusline for when you need it.

`ephemeral-messages` option can be set to `true` to make status / error messages at the bottom of the UI *not* take up an entire line. Instead, they will be printed over the editor view directly.

`disable-dot-repeat` disables the hardcoded behavior of `.` to repeat the previous operation, letting you map `.` to something else. \
Keep in mind, with this option turned on, you lose the ability to dot-repeat, as you can't map something else to do dot-repeat.

`show-diagnostics` is set to `true` by default. \
Exists because you can't actually toggle diagnostics globally otherwise.

`harp` subsection, that will be explained in the harp section below.

### Extra command expansions

Considering our example context...

|Thing                    |Value                                            |
|-------------------------|-------------------------------------------------|
|current buffer           |`~/prog/dotfiles/fish/abbreviations/percent.fish`|
|current working directory|`~/prog/dotfiles`                                |

...here's what the added command expansions would evaluate to:

|Expansion             |Output                                           |Explanation                             |
|----------------------|-------------------------------------------------|----------------------------------------|
|`%{full_path}`        |`~/prog/dotfiles/fish/abbreviations/percent.fish`|full path|
|`%{working_directory}`|`~/prog/dotfiles`                                |current working directory|
|`%{relative_path}`    |`fish/abbreviations/percent.fish`                |buffer path, relative to current working directory|
|`%{buffer_parent}`    |`~/prog/dotfiles/fish/abbreviations`             |parent directory of the current buffer

> [!CAUTION]
> The path evaluates to have `~`, instead of `/home/username`
> This is because paths starting with `~` are accepted everywhere in helix, and when used in your shell (like with `:sh`), will be expanded as well.
> But, in the rare case where it doesn't expand, beware of this behavior.
> I'm taking this tradeoff to see `~/r/dot` instead of `/home/axlefublr/r/dot` in my various mappings that `:echo` expanded paths.

### Harp integration

Inspired by [`harp-nvim`](https://github.com/Axlefublr/harp-nvim), implemented using the [`harp`](https://github.com/Axlefublr/harp) library.

A "harp" is essentially a storage unit. It lets you store some information from the editor to then use later. \
Harps are *persistent*. Once you set a harp, it stays forever (until overwritten by you) and gets shared across helix sessions. \
Even if you have multiple helix sessions open at a time, if you set a harp in one session, it will *immediately* become available in all sessions.

Harp "sections" exist to organize multiple different sets of harps. \
If all harps were stored in a single place, that would lead to key collisions: if you set a [file harp](#file-harps) `a`, you wouldn't be able to set a [register harp](#register-harps) `a` — the latter would override the former. \
So, all harp types are stored within their own "harp section", letting you use the same harp keys without key collisions.

Each harp type has two actions: `get` and `set`. \
Both of them ask you to press a key¹, that will become the key of the harp. \
`set` takes some information from the environment (for example, the current buffer's filepath), and stores it in a harp.
`get` takes that information from a harp, and applies it somehow (for example, `:open`s the stored filepath).

¹ As far as I know, all keys that helix would undertand in a mapping, will also be a valid harp key. So you very much could use keys with modifiers as well, if you want to.

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

`set` stores your current working directory in a harp. \
`get` `:cd`s into a stored working directory.

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

`set` puts the contents of your default register (`"` by default, decided by `default-yank-register` option) into a harp. \
`get` puts the stored text back into your default register

If you use `set` while having multiple selections, they are joined into a single one with newlines.

#### Command harps

```
harp_command_set
harp_command_get
```

`set` puts your most recent command mode command (register `:`) into a harp. \
`get` executes a stored command mode command and writes it to the `:` register.

Supports command expansions! :3

#### Relativity

Now that you're familiar with all the harp types, let me introduce you to the feature of relativity.

Remember how different harp types are stored in different sections to fight against key collisions? \
Relativity is made by appending the directory path / buffer path / filetype onto the name of the action's section, resulting in infinite possible sections. \
This further increases the amount of your usable harps, despite them all being limited to a single keypress. \

When you use any harp action, you will see `(global)` to the right of the prompt. The word in the brackets shows you the currently active relativity, and `global` is the default relativity for all harp actions.

Press <kbd>.</kbd> to make the harp relative to your current working directory. \
Press <kbd>,</kbd> to make it relative to the current buffer. \
Press <kbd>;</kbd> to make it relative to the current buffer's filetype (run `:lang` to check the filetype of the current buffer). \
Press <kbd>'</kbd> to go back to global relativity, and not be relative to any of the above things (as in, the section name is not appended with anything).

You can press these keys to change relativity as many times as you need, until you press a different key, that will be *actually* interpreted, using the relativity you ended up arriving to. \
Alternatively, you can press <kbd>Esc</kbd> to cancel the harp action.

This way, you can have "global" file harps, but also file harps that are specific to the current project you're working on.

Useful global searches like `(TODO|FIXME|HACK|MOVE):?`, and buffer-specific searches like `// asdf I left off here`.

Project-specific register harps, as a way to gain register session persistence, and filetype-specific register harps, that can act as a basic snippet implementation.

When using harp relativity, you may eventually notice that you *mostly* want a certain relativity for a given harp type: global searches are rarer to want compared to project local ones, for example.

You can actually change the default relativity, from "global"!

The default config is the following:
```toml
[editor.harp]
command = 'global'
cwd = 'global'
file = 'global'
register = 'global'
relative_file = 'global'
search = 'global'
```

As you can see, all harp actions use the `global` relativity by default. \
The other options you have are: `buffer`, `directory`, `filetype`.

As you use harps, you will naturally find yourself using some relativity far more often than global, for some of the harp actions. \
In my experience, command harps make the most sense to be `filetype`, as I have various command harps to do language-specific actions, like running the current script, or opening documentation for the current project. \
Register harps probably should be `directory`, as it's not that common that your various yanks will be relevant to each other, across projects. \
Search harps can be very strong with `buffer`, as you can create some sort of special comments in various parts of the code, to then be able to jump to instantly — vim's "marks", but that don't break after you modify other parts of the buffer.

There's a lot of depth to harps! So play around, and find usecases that harps solve for *you*. \
In some cases relativity doesn't make sense logically¹, but this approach lets me implement flexible functionality that *you* may, in some cases, use in ways that I didn't think of.

¹ i.e. directory relative directory harps — you can't *set* a directory harp to a directory that is different from your current one; so while `get`ting could be very useful, `set` doesn't allow you to make it useful :p

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

[![Packaging status](https://repology.org/badge/vertical-allrepos/helix-editor.svg?exclude_unsupported=1)](https://repology.org/project/helix-editor/versions)

# Contributing

Contributing guidelines can be found [here](./docs/CONTRIBUTING.md).

# Getting help

Your question might already be answered on the [FAQ](https://github.com/helix-editor/helix/wiki/FAQ).

Discuss the project on the community [Matrix Space](https://matrix.to/#/#helix-community:matrix.org) (make sure to join `#helix-editor:matrix.org` if you're on a client that doesn't support Matrix Spaces yet).

# Credits

Thanks to [@jakenvac](https://github.com/jakenvac) for designing the logo!
