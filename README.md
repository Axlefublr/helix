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

`shell_replace_with_output` mappable action, that acts like `shell_pipe`, but doesn't pipe the selections into the command. So, just execute a command and replace selections with the output.

`:random` command with aliases `:rnd`, `:rng` that randomizes your selections.

`whichkey` option in the `[editor]` section can be set to `true` (default) or `false`. \
If set to `false`, the infoboxes for *mappings* will not show up. \
This is different from just disabling the `auto-info` option in that you will still get the popup for `select_register`.

### Command expansions

When using actions and commands that let you execute a shell command (`:sh`, `shell_insert_output`, `shell_append_output`, `shell_pipe`, etc), you can now use command expansions.

Example usage: `:sh echo %p`.

Consider that our current working directory is `~/prog/dotfiles` and the currently open buffer is `~/prog/dotfiles/helix/config.toml`. \
Here's what all command expansions would evaluate to:
|Expansion|Output                                          |Explanation                             |
|-------- |------------------------------------------------|----------------------------------------|
|`%p`     |`/home/username/prog/dotfiles/helix/config.toml`|full path                               |
|`%h`     |`/home/username/prog/dotfiles/helix`            |"head" of the current buffer            |
|`%w`     |`/home/username/prog/dotfiles`                  |helix's working directory               |
|`%m`     |`%m`                                            |not an expansion, taken literally       |
|`%%p`    |`%p`                                            |escaped using `%%` to be taken literally|

### Harp

My favorite feature I had in nvim! \
Inspired by [`harp-nvim`](https://github.com/Axlefublr/harp-nvim), implemented using the [`harp`](https://github.com/Axlefublr/harp) library. \
I will be using concepts from `harp` in my explanation, I expect you to have read the readme of that project. \
You don't have to be familiar with `harp-nvim`, but if you are, you'll get what I'm going to be talking about easier.

You can think of "harps" as storage units. You can store some information related to the editor in a harp, to then use it later. \
Harps are *persistent*. Once you set a harp, it stays forever, until overwritten in the future (by you, again) and gets retained across helix sessions. \
Even if you have multiple helix sessions open at a time, if you set a harp in one session, it will *immediately* become available in another session.

To organize harps, there are harp "sections". \
Sections exist to organize all the different types of harps, to allow you to use one harp name across multiple different harp *types*.
Each individual harp is given its name by *you*, interactively. \
So, you can store some file in a "file harp" called `a`, and then store your latest search in a "search harp" called also `a`; these will not conflict, and let you consistently use short names for maximum efficiency (if that is your preference). \
You can either *get* a harp (take the information stored in a harp and use it somehow) or *set* a harp (take some information from the environment and store it in a harp).

#### Why are harps helpful?

Typing things in takes a long time, while being done so often.

Let's take `:open` for example. \
If you want to open a path with a really long path, you will ideally want to type in the minimum amount of characters possible, to then press <kbd>Tab</kbd> to autocomplete, on *each* path component. \
This consistently gets annoying when the string you need to type in to tab complete is long too. \
If you want to complete `~/downgrade/test.lua`, you need to type in `downg` to tab complete, because `Downloads` also exists in that directory. \
Then maybe in `downgrade`, there is also `test.py`. Now you have to type in `test.l` and at that point, it's not even worth completing.

Fuzzy searching is better but has a different issue. \
First of all, it's dependent on your current working directory: `file_picker` of helix, or `telescope` of nvim will usually open from your cwd. \
If you need to open a file from somewhere else, you're fucked.

In the case with helix, you can `:open` a directory to fuzzy search it, but at that point you're just mixing in two non-perfect methods.

The second issue with the fuzzy file picker, is the min-maxed fuzzy strings you end up creating and memorizing to get to the file you want the most efficiently (similar to the min-maxed tab complete strings in `:open`). \
Those can get pretty arbitrary; to get to `helix/generator.py` I have in my dotfiles, the fuzzy search for it is `raty`. \
Worse yet, it might change in the future, if I create a new file that matches `raty` more closely. \
Well, I don't have to deal with that anymore! Now it's literally just `c` and I can get to it from *anywhere* **instantly** with a file harp.

Simple, direct, fast.

Harps don't magically remove your usage of `:open` or the fuzzy search picker, they *minimize* it. \
First you get to some file using one of the two, and store it in a harp. \
Now you get a "bookmark" of the file, that lets you completely circumvent having to dance around with `:open`/fzf ever again with that file. \
The different harp types (only one file-structure-related harp type exists, I intend to add more) allow you to express *what* you're storing and relative to *where* you're storing it, giving you a lot of flexibility *and* speed.

Stop thinking about *how* to get to your file, let your muscle memory move you there.

#### Structure

Throughout the explanation of each upcoming harp type, I will also be giving you the section names. \
They do not come up in usage, but if you ever look at the data file (`~/.local/share/harp.yml` on linux), you will know what refers to what. \
This may also help you understand if things aren't working the way you expected them to, if you know how sections are named and made. \
Each harp type will start with the `get` mappable action and the `set` mappable action, that you can make mappings for. \
The name `set` may possibly be confused to mean "this can only be set once". \
This is not the case, `set` creates a new harp if it didn't exist before, or *updates* a harp if it already exists, so if you realize you don't need the old value of some harp, you can *override* it with a new one using the `set` action. \
After an explanation for what the harp *does*, I will explain the usecase and thought process behind creating it.

#### File harps

```
harp_file_get
harp_file_set
```

`set` to store the full path of the current buffer into a harp in the `harp_files` section. \
`get` to `:open` the file stored in the harp.

This is really useful for files that you know you want to open from *anywhere*. \
Perfect usecase is for dotfiles. \
Say you were just editing some code and opened lazygit. \
Now you realize that you want to change some setting in your lazygit config. \
But oh no! You were working on some project, and really can't be bothered to go locate the lazygit config file, so you just think "meh, I'll do that later" and forget about it. \
I always found that really annoying!

Well, if you previously stored that config file in a *file harp*, you don't need to be in this situation anymore. \
Just `harp_file_get`, change the setting you wanted, and go back to the file you were just editing in the project, continuing to use your (now reconfigured) lazygit. \
I've been using this for a while in nvim, and from personal experience, I noticed that you get to retain the "flow" state, which is *really* helpful when programming.

### Search harps

```
harp_search_get
harp_search_set
```

`set` gets your latest search (stored in the `/` register) and stores it in a harp in the `harp_searches` section. \
`get` puts the stored search into your `/` register, effectively "making a search".

A really obvious thing to want to do in an editor is to trim trailing whitespace. \
In helix it's a bit of a hassle: `%s[ \t]+$<CR>d`, where `<CR>` means <kbd>Enter</kbd>. \
Focusing on the pattern: `[ \t]+$` is just slightly too much to type in for me. \
`\s` won't work there because the final newline in the file gets matched. \
I could I guess rely on helix to append it, but that seems a bit wack to do anyway. \
`[ \t]` is too much to type in, so I might opt for just matching spaces. \
But then I might miss some rogue tab!

With search harps, I can store this search in a harp. `t`, for example. \
Now instead of having to type in `[ \t]+$`, I can press my mapping for search harps, do `t<CR>`, \
and then when I `%s`, I'll see the pattern I want as an autosuggestion. \
I can press <kbd>Enter</kbd> twice, and bob's my uncle.

You might argue that this is a pretty small binding optimization, and you might be right about that. \
For some reason though, grabbing a pattern like this still *feels* better to me than typing it in, even though it's not *that* long.

A better example would be some more complex / long pattern, that is simply unreasonable to type in, kinda ever. \
Or maybe a pattern that you will forget, but could remember the harp name of.

Because search harps just put the output into your `/` register, the searches end up as autosuggestions in a lot of places. \
For example, you can search for `.gitignore` and save it in the `g` search harp. \
When you get that `g` search harp in the future, you can open the file picker to see `.gitignore` autosuggested. \
Relative file harps (will be implemented) work better for this usecase, though.

Probably the most useful example: \
Search for `(TODO|FIXME|HACK):` and store it in a search harp. \
Now you have a very convenient way to look through TODOs of any project: just `get` the search harp for it, open `global_search`, press enter, and see all of your results.

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
