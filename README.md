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

`shell_replace_with_output` mappable action, that acts like `shell_pipe`, but doesn't pipe the selections into the command. So, just execute a command and replace selections with the output.

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

### Implemented PRs from upstream

* [11234](https://github.com/helix-editor/helix/issues/11234) by @Swordelf2
* [9143](https://github.com/helix-editor/helix/pull/9143) by @intarga
* [2608](https://github.com/helix-editor/helix/pull/2608) by @Philipp-M

### Command expansions

Supported in: `shell_pipe`, `shell_pipe_to`, `shell_insert_output`, `shell_append_output`, `shell_replace_with_output` and **all** `:command`s. I'm not yet sure if the latter is a good idea, but feel free to `:cd %p` if you wish /j.

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
If you want to open a file with a really long path, you will ideally want to type in the minimum amount of characters possible, to then press <kbd>Tab</kbd> to autocomplete, on *each* path component. \
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
Well, I don't have to deal with that anymore! Now it's literally just `c` and I can get to it from *anywhere* **instantly** using a file harp.

Simple, direct, fast.

Harps don't magically remove your usage of `:open` or the fuzzy search picker, they *minimize* it. \
First you get to some file using one of the two, and store it in a harp. \
Now you get a "bookmark" of the file, that lets you completely circumvent having to dance around with `:open`/fzf ever again with that file. \
The different harp types allow you to express *what* you're storing and relative to *where* you're storing it, giving you a lot of flexibility *and* speed.

Stop thinking about *how* to get to your file, let your muscle memory move you there.

### File harps

```
harp_file_set
harp_file_get
```

`harp_file_set` to store the full path of the current buffer into a harp in the `harp_files` section. \
`harp_file_get` to `:open` the file stored in the harp.

This is really useful for files that you know you want to open from *anywhere*. \
Perfect usecase is for dotfiles. \
Say you were just editing some code and opened lazygit. \
Now you realize that you want to change some setting in your lazygit config. \
But oh no! You were working on some project, and really can't be bothered to go locate the lazygit config file, so you just think "meh, I'll do that later" and forget about it. \
I always found that really annoying!

Well, if you previously stored that config file in a *file harp*, you don't need to be in this situation anymore. \
Just `harp_file_get`, change the setting you wanted, and go back to the file you were just editing in the project, continuing to use your (now reconfigured) lazygit. \
I've been using this for a while in nvim, and from personal experience, I noticed that you get to retain the "flow" state, which is *really* helpful when programming.

### Project file harps

```
harp_project_file_set
harp_project_file_get
```

`harp_project_file_set` to store the full path of the current buffer into a harp in a section\*. \
`harp_project_file_get` to `:open` the file stored in the harp.

\*The section used is built like this: it will always start with `harp_files_`, and after that string, your current working directory will be appended.

So if your current working directory is `~/prog/dotfiles` and your current buffer is `~/prog/dotfiles/colors.css`, when using a project file harp to save that buffer, you will be storing into a section named `harp_files_/home/username/prog/dotfiles`.

You can think of project file harps as file harps that are *relative to the project*.

This is pretty powerful! Normal file harps are mostly meant for files that are important to be accessible from *anywhere*. Things like config files that you may want to edit while in the middle of doing something else.

In this project, I visit `helix-term/src/commands.rs` pretty frequently. It is only ever relevant when my current working directory *is* this project, and yet with a normal file harp, I can't express that. \
I'd want to express it for this reason: I use the register name `c` for another (more commonly visited) harp already. \
I'd love to use `c` for `helix-term/src/commands.rs` too, but it's already taken, so I take the compromise of naming it `com` instead. \
This is not too bad when considered in a vacuum, but as my amount of file harps increases, the need to name them in increasingly complex ways does too.

With a project file harp, I can use `c` again! Matter of fact, I can use `c` in literally every different project I have, if I want to. There will be no conflicts, as they're stored in different sections.

When deciding between a file harp and a project file harp, ask yourself this question: "when I want to access this file, what will my current working directory generally be?".

### Cwd harps

```
harp_cwd_set
harp_cwd_get
```

`set` takes your current working directory (like from `:pwd`), and stores it in a harp in the `harp_dirs` section. \
`get` takes the stored directory, and `:cd`s into it.

Development is pretty projectual, and jumping through a bunch of commonly visited directories can be a chore. Closing and reopening helix just to fuzzy search some file somewhere is a bit too much effort.

I have a very specific example:

I use `lazygit`, and have a config for it stored in `~/prog/dotfiles/lazygit.yml`. \
I change things in it every so often, and to make myself not have to google the default config every time, I store it as a file in `~/prog/backup/default/lazygit.yml`.

This one time, let's `:cd ~/prog/backup` and use `harp_cwd_set` to store that working directory as a cwd harp named `b`. \
Next time, when I want to access that file while being in `~/prog/dotfiles`, everything gets easier!

Instead of:
* close helix
* travel to `~/prog/backup`
* open helix
* fuzzy search for `lazygit`

I just do
* `harp_cwd_get` -> `b`
* fuzzy search for `lazygit`

Quite a bit nicer! Even better than that, is that assuming we set `dotfiles` too, we can easily come *back* as well!

You might point out that a normal file harp would suffice. You would be correct! In a vaccuum, using a file harp to get to a file is optimal. \
However, that `backup` directory of mine contains a lot of useful files, and it's simply more *cost effective* (in terms of my brain memory) to just mark the directory, rather than coming up with appropriate names for each individual file. \
If I figure out that the default lazygit config, in specific, I visit often enough to warrant a file harp, I *still* benefit from the cwd harp I set, \
because *getting* to the file to then set the file harp for it still gets easier!

The more obvious usecase for this feature, of course, is if you work on a bunch of projects at the same time, you can switch between them more easily. But, like, duh.

### Search harps

```
harp_search_set
harp_search_get
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
For example, you can search for `struct HarpOutput` to match the definition of a struct you have *somewhere* in the codebase, and then save it in the `hout` search harp. \
When you get that `hout` search harp in the future, you can open the global_search picker to see `struct HarpOutput` autosuggested. \
Press <kbd>Enter</kbd> twice and blammo. \
This way, you can get to the definition of the struct in a faster way, than relying on your lsp. Especially true with rust, but generally raw text matching will happen faster than your lsp responding.

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
