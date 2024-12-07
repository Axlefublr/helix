# Setting expectations

This is, for the most part, a strictly personal fork.

**I will arbitrarily add breaking changes**, either to the changes already made by this fork, or to core. \
Autoupdating this fork is a bad idea, consequently. \
Instead, check what changed from time to time (the contents of this readme) to decide when you want to update.

The commits don't reflect the *history* of changes I've made, \
they are a series of patches I apply *on top* of helix master. \
Whenever I make some change, I will either amend (edit) an already existing relevant commit, or add a new one, not necessarily as the latest commit. You can notice how changes are grouped by category, in the commits of the fork. \
Some of my changes will seem crazy to you — drop the commits with changes you dislike, before building this fork.

I rebase on upstream helix master every time I see a new feature I want from it.

Join my [discord server](https://discord.gg/bgVSg362dK) if you want to follow the development of the fork.

If you want to contact me on discord without joining a server, my username is `axlefublr`.

# Installation

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

# This fork's changes

## Merged PRs from upstream

|Author|Link|Title|
|------|----|-----|
|@Swordelf2      |[11234](https://github.com/helix-editor/helix/issues/11234)|Add reload! and reload-all! commands|
|@scdailey       |[9483](https://github.com/helix-editor/helix/pull/9483)    |Implement Kakoune's A-S: Select first and last chars|
|@pantos9000     |[10576](https://github.com/helix-editor/helix/pull/10576)  |Add same_line and anchored movements|
|@EpocSquadron   |[9843](https://github.com/helix-editor/helix/pull/9843)    |Indent text object|
|@omentic        |[14121](https://github.com/helix-editor/helix/pull/14121)  |Add support for moving selected lines up and down|
|@useche         |[11700](https://github.com/helix-editor/helix/pull/11700)  |Add per view search location and total matches to statusline|

Big shoutout to these people!

---

Here's another table, of PRs I used as a starting point, but ultimately changed most of the code of.

|Author          |Link                                                       |Title                                       |
|----------------|-----------------------------------------------------------|--------------------------------------------|
|@oxcrow         |[13053](https://github.com/helix-editor/helix/pull/13053)  |Add local search in buffer (fuzzy and regex)|

*This* is more so a shoutout than a link for you to check as documentation; so the features will be explained later separately.

## Defaults

Tab width 4 -> 3. \
This only applies to the `text` filetype, and only if helix didn't notice a different indentation from the context of the file.

All pickers now take up the entire screen, rather than 90%. \
Thanks to @satoqz for figuring out how to do this! :D

## Behavior

`indent` action now works even on empty lines.

`insert_tab` inserts the tab character always, not whatever the indent string is.

The `scrolloff` option is applied only vertically, not also horizontally.

`select_next_sibling`, `select_prev_sibling`, `extend_next_sibling`, `extend_prev_sibling` skip unnamed treesitter nodes, similar to how `select_all_children` does.

`save_selection` is now silent: doesn't print “Selection saved to jumplist” to the messages line.

`search` / `rsearch` / `search_next` / `search_prev` no longer shows “Wrapped around document”.
The thinking is that you'll be using the search count statusline element anyway.

`:run-shell-command` no longer shows the popup if the command's output is just whitespace (there was a check for if it's *empty*, but not for if it's just *whitespace*)

## Global formatter

On write, the contents of the buffer are piped into `helix-piper`¹ and are replaced with its output². \
You're meant to have `helix-piper` as a symlink to some formatter binary that you want to run for *everything*. \
Unfortunately helix doesn't have a way to:
1. set a formatter for every language
2. set more than one formatter for a given language
3. set a formatter (or lsp) for the default, "text" language

So this is why I came up with this workaround. \
Beware: due to implementation details, std**err** is considered valid output. \
If your formatter prints an error to stderr, you'll end up seeing your entire buffer replaced with that error message. \
This can be destructive if you use `:write-all` while not looking at that buffer, and then close helix without realizing you demolished some buffer. Food for thought.

If you're not sure how this can be useful to you, but are excited about the possibilities, check out [my blog post](https://axlefublr.github.io/consider-sorting/) on how I automatically sort *sections* of some files.

¹ If you have it in `$PATH`

² If that output is different from the input text. If the input and output are the same, the document doesn't get touched. \
**Unless** the document is "too large". Don't ask me! I don't know either! This optimization comes from the issue of doing a nothing burger write on scratch / empty buffers; if it happens to help other cases that's cool too but ultimately not the main goal. \
But if you're genuinely interested in the implementation detail, "too large" happens when the string of the document isn't laid out in contiguous memory. Which I can't imagine being practically applicable information 👍

## Autopairs

By default, `mi(` and `mi)` are equivalent: both will select `(this_text)`. \
Now, `mi(` will select `(this_text)` and `mi)` will select `)this_text(`. \
This new behavior is shared across all the other bracket pairs.

If you type a space between two autopair characters, you will insert *two* spaces instead. \
`(|)` → `( | )` previous character is `(`, next character is `)`. `)` is the closing variant of `(`, so we insert two spaces. \
`(| )` → `( | )` only one space is inserted, because the next character (a space) is not the closing variant (`)`) of the previous character (`(`).

Typing the closing delimiter (if it's the same as the opening delimiter) no longer skips over it — you simply insert another closing delimiter. \
`(|)` → *types `)`* → `()|` \
`"|"` → *types `"`* → `""|""`

## Bufferline

In default helix, there's one space on each side of each buffer name in the bufferline. \
So the first buffer's ending space and the second buffer's starting space ends up being 2 spaces together. \
This means that there's this awkward space before the entire bufferline, instead of being flush with the absolute left of the editor. \
This change places *two* spaces *after* each buffer, making the 2-sized gap still remain, but without the awkward leading space. \
`justify-content: space-between` rather than `space-around`, basically.

Also, the index of each buffer is shown before the filename.

## Statusline

The messages / commandline is merged with the statusline. \
Whenever you get a message, it replaces the statusline completely.
Similarly, the statusline disapppears when you use command mode.

The functionality of the messages line that shows you the keys you are pressing in chorded mappings is removed, to make the above more viable.

Some elements in the statusline now display differently, to take up less space and/or be less jumpy:

### primary-selection-length

Filled to three characters, so that your statusline doesn't jump around as much when you select stuff.

```
chr: 9
chr:67
chr:133
```

### selections

Current selection's index is filled to how long the number of your *total* selections is, but at least two.

```
sel: 1
sel: 1/2
sel:10/22
sel: 27/333
```

### position

Line number is aligned to 4, column number is center aligned to 3.

```
   1: 1
  77:12
 326:133
4267:77
```

### position-percentage

Aligned to 3. `top` is shown instead of 0%, `bot` is shown instead of 100%. \
The percentage calculation now happens with floats instead of integers, and then rounded; thanks to this `bot` kicks in a bit earlier, rather than only on the trailing final newline in the file. \
Being on the first line in the file *guarantees* `top` (previously would be 50% in an empty file). \
Being on the last *real* line in the file *guarantees* `bot` (previously only the trailing final newline would be considered `bot` if the file is small enough)

```
top
 1%
23%
67%
bot
```

### total-line-numbers

The final trailing newline is not counted. \
In other words, a scratch buffer will start at being 1 lines long, rather than 2.

## Hardcoded mappings

Hover docs, pickers, and possibly other various popups have <kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> hardcoded to mean "scroll by half a page down / up" \
I remove them because <kbd>ctrl+u</kbd> overrides the very useful "delete until the start of the line" mapping. \
Not being able to delete the entire text in a picker at once is very painful. \
You can use <kbd>PageDown</kbd> and <kbd>PageUp</kbd> instead.

### prompts (command mode, pickers, etc)

<kbd>ctrl+v</kbd> pastes the contents of your `default-yank-register`. A shortcut to <kbd>ctrl+r "</kbd>, basically.

<kbd>alt+o</kbd> toggles surrounding your input with `\b` (toggling word bounds) \
`something` → `\bsomething\b` → `something` → …

<kbd>alt+i</kbd> toggles case sensitivity. Case sensitive → Case insensitive → Nothing. \
`something` → `(?-i)something` → `(?i)something` → `something`

### pickers

Due to <kbd>ctrl+v</kbd> now pasting, the hotkey to open the result in a vertial split is moved from <kbd>ctrl+v</kbd> to <kbd>ctrl+m</kbd>.

By default, <kbd>Home</kbd> and <kbd>End</kbd> jump to the first / last result, in the list of results. \
This once again overshadows very useful hotkeys, that let you go to the start/end of the line (your picker input) :/ \
Now, <kbd>ctrl+Home</kbd> and <kbd>ctrl+End</kbd> are used instead.

### textobjects

In `select_textobject_around` (<kbd>ma</kbd>) and `select_textobject_inner` (<kbd>mi</kbd>) specifically.

The whichkey ui wording is debloated:
```
┌Match inside────┐
│ w  Word        │
│ W  WORD        │
│ p  Paragraph   │
│ i  Indentation │
│ g  Change      │
│ m  Pair 󰌪      │
│ a  Argument 󰌪  │
│ e  Entry 󰌪     │
│ f  Function 󰌪  │
│ t  Type 󰌪      │
│ T  Test 󰌪      │
│ c  Comment 󰌪   │
│ x  Tag 󰌪       │
└────────────────┘
```

Add alias `l` for paragraph, keeping the default `p` as well, and alias `o` for big WORD, keeping the default `W` as well.

## New commands

`:random` randomizes the order of your selections. Has aliases `:rnd`, `:rng`.

`:echopy` command is exactly like `:echo`, but *also* puts the result into your `default-yank-register`. Has alias `:cc`.

`:buffer-nth` (with alias `bi`) lets you travel to the nth buffer, out of those you have open currently. The `-r` flag counts buffers from the end, rather than from the start. \
Reason why I didn't make it just use negative numbers, is because they are attempted to be interpreted as flags, and so you have to do `-- -1`. `-r 1` is slightly nicer.

`:run-shell-command-quiet` with aliases `?` and `shq` is exactly like `:run-shell-command`, but doesn't show the output of the command.

## New actions (except [harp](#harp-integration))

`extend_next_sibling` and `extend_prev_sibling` *create* new selections, unlike `select_next_sibling` and `select_prev_sibling` that only move existing ones.

`toggle_line_select` does `trim_selections` if any of your selections end in a newline, while not *all* of them being single column selections that only contain a newline. \
If all of your selections are single column selections that are all newlines, or some of your selections don't end in a newline, does `extend_to_line_bounds`. \
A bit of a confusing definition, but in effect this means that this action will try its best to expand all of your selections to the whole line, and will only `trim_selections` when all of your selections are whole line selections. \
The whole "single column selections that are newlines" business is needed to handle the case where you position your cursor on a newline, while intending to select that entire line.

`surround_add_tag` prompts you with the name of an html tag, and surrounds your selections with it. \
You have `word`, type in `div`, and get `<div>word</div>`. \
The history for it is stored in the `<` register.

`local_search_fuzzy` lets you fuzzy search the contents of the *current* buffer with a picker. \
You cannot open the resulting line in a split like you usually can — only the “normal” open (<kbd>Enter</kbd>) is supported. \
As an upside of this, this picker will work even if the buffer doesn't have a path.

`local_search_section` lets you search through the list of “sections”, with a picker. \
A section is a line like this:
```
// -----------------------some title------------------------
```
Meaning, a line that has at least 4 consecutive `-` symbols. \
In the picker, only `some title` will be shown, instead of the full line. \
Thanks to this, you have a very clear and debloated overview of all of your sections.

A subfeature of this action happens in markdown files (`:lang` == “markdown”) \
Instead of looking for `----` delimited sections, you get a tree of headings for you to search through. \
Every heading title also includes its parent headings, so you can more easily navigate more heavily nested markdown documents.

## New options

All are in the `[editor]` section.

`whichkey` can be set to `true` (default) or `false`. \
If set to `false`, the infoboxes for *mappings* will not show up. \
This is different from just disabling the `auto-info` option in that you will still get the popup for `select_register` and `select_textobject_inner` / `select_textobject_around`.

`disable-dot-repeat` disables the hardcoded behavior of `.` to repeat the previous operation, letting you map `.` to something else. \
Keep in mind, with this option turned on, you lose the ability to dot-repeat, as you can't map something else to do dot-repeat.

`show-diagnostics` is set to `true` by default. \
Exists because you can't actually toggle diagnostics globally otherwise.

`harp` subsection, that will be explained in the harp section below.

## Extra command expansions

Considering our example context...

|Thing                    |Value                                            |
|-------------------------|-------------------------------------------------|
|current buffer           |`~/prog/dotfiles/fish/abbreviations/percent.fish`|
|current working directory|`~/prog/dotfiles`                                |

...here's what the added command expansions would evaluate to:

|Expansion             |Output                                           |Explanation                             |
|----------------------|-------------------------------------------------|----------------------------------------|
|`%{full_path}`        |`~/prog/dotfiles/fish/abbreviations/percent.fish`|full path|
|`%{relative_path}`    |`fish/abbreviations/percent.fish`                |buffer path, relative to current working directory|
|`%{buffer_parent}`    |`~/prog/dotfiles/fish/abbreviations`             |parent directory of the current buffer

> [!CAUTION]
> The path evaluates to have `~`, instead of `/home/username` \
> This is because paths starting with `~` are accepted everywhere in helix, and when used in your shell (like with `:sh`), will be expanded as well. \
> But, in the rare case where it doesn't expand, beware of this behavior. \
> I'm taking this tradeoff to see `~/r/dot` instead of `/home/axlefublr/r/dot` in my various mappings that `:echo` expanded paths.

## Harp integration

[My blog post on harp](https://axlefublr.github.io/harp/) is the most complete explanation of the idea and feature. \
The following expects you to have read the blog post.

When you invoke a harp action, you get shown all the existing registers for that section in an `auto-info` (whichkey ui). \
Press <kbd>Space</kbd> to toggle between get / set. \
You can delete a register by first pressing <kbd>Backspace</kbd>, and then the key that you want to delete. \
Doing so will bring you back to `get`, rather than closing the whichkey ui.
Thanks to this, you can keep on deleting registers one after another.

If you got into the `del` mode and go “nevermind”, you can either press <kbd>Escape</kbd> to cancel the harp action, or press <kbd>Space</kbd> to go back into `get` mode.

To clear *all* of the registers in the section instead of only one at a time, press <kbd>Alt+Backspace</kbd>. \
It will delete the currently set registers and put you into `set` mode. \
This hotkey works regardless of the mode you're in (get / set / del).

Each harp action's register contents are displayed in a way where you get enough information to tell them apart, but not so much that they take up your entire screen. \
For example, file related harp actions will display `helix/languages.toml` rather than the `/home/axlefublr/fes/dot/helix/languages.toml` that is *actually* stored. \
The more surprising display decisions I'll point out in the following harp actions, when appropriate.

### File harps

```
harp_file
```

`set` takes the *current buffer*'s filepath, and stores it in a harp. \
`get` takes it, and `:open`s it.

### Relative file harps

```
harp_relative_file
```

`set` takes the current buffer's filepath, and stores it in a harp. \
HOWEVER, it stores only the part of the full path, that's relative to current working directory.

Say your current buffer path is `~/prog/dotfiles/colors.css` and your current working directory is `~/prog/dotfiles`. \
If you use a normal [file harp](#file-harps), you will store the full path: `~/prog/dotfiles/colors.css`. \
If you use a *relative* file harp, you will store the *relative* path: `colors.css`.

So then, the `get` action will just open that path relatively — as if you did `:open colors.css`. This will end up opening a different file depending on your current working directory.

### Cwd harps

```
harp_cwd
```

`set` stores your current working directory in a harp. \
`get` `:cd`s into a stored working directory.

### Search harps

```
harp_search
```

`set` takes your latest search pattern from register `/` and stores it in a harp. \
`get` takes a stored search pattern, puts it back into register `/` and effectively presses <kbd>n</kbd> for you (`search_next` action).

Some processing is done on the contents of registers when displaying them in the whichkey menu, to make them less bloated / overwhelming. \
Only the first line is shown; it is also trimmed of whitespace, and of `\b` on the start and end. \
If the search pattern is itself multiline, your `editor.whitespace.characters.newline` is going to be displayed at the end.

This is only for the whichkey *display* — the actual contents of the harp remain the same.

### Mark harps

```
harp_mark
```

`set` takes the path to the current buffer, and your primary selection's cursor position, and stores it as [file, line, column]. \
`get` takes the stored file and opens it. Then, takes the remaining [line, column] and puts your cursor into that position.

Basically like vim's marks.

In the whichkey ui, you get a line column + path column + the first 50 characters of the line. \
Note that you see what the contents were when you *set* the mark, **not** what the contents are currently. \
The thinking is that it's more useful to see what you set the mark *for*, rather than possibly seeing a useless empty line, or a line with just `}`, because it got shifted since then. \
The column column (lol) is not displayed, because it's more so visual noise than a useful indicator. \
The path is *not* shown if the relativity is `buffer`.

### Register harps

```
harp_register
```

`set` puts the contents of your default register (`"` by default, decided by `default-yank-register` option) into a harp. \
`get` puts the stored text back into your default register

If you use `set` while having multiple selections, they are joined into a single one with newlines. \
If you use `get` in insert mode, you immediately paste the result instead of putting it into your `default-yank-register`. The pasted result is the new selection.

Very niche but this lets me use register harps as snippets: \
After pasting (in insert mode only) the harp's contents, if they contain `█` (U+2588), selections are placed on all of them and <kbd>c</kbd> is pressed. \
Effectively, you can use that character in your register harps to express the position that you want to move your cursor to.

As an example: after I use a register harp (in insert mode) to retrieve (and therefore paste) the below content, I'll immediately start typing in two places at once.

```fish
function █
end
funcsave █ >/dev/null
```

The whichkey display is the same as with search harps, except no `\b` trimming occurs (the whitespace trimming still does).

### Command harps

```
harp_command
```

`set` puts your most recent command mode command (register `:`) into a harp. \
`get` executes a stored command mode command and writes it to the `:` register.

Supports command expansions! :3

### Relativity

When you use any harp action, you will see `(global)` to the right of the prompt. \
The word in the brackets shows you the currently active relativity, and `global` is the default relativity for all harp actions.

Press <kbd>.</kbd> to make the harp relative to your current working directory. \
Press <kbd>,</kbd> to make it relative to the current buffer. \
Press <kbd>;</kbd> to make it relative to the current buffer's filetype (run `:lang` to check the filetype of the current buffer). \
Press <kbd>'</kbd> to go back to global relativity.

You can press these keys to change relativity as many times as you need, until you press a different key, that will be *actually* interpreted, using the relativity you ended up arriving to. \
Alternatively, you can press <kbd>Esc</kbd> to cancel the harp action.

You can change the assumed relativity per harp action.

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
The other values you could use instead are: `buffer`, `directory`, `filetype`.

`global` is a good default, but restricting long-term. I heavily recommend changing it to something else for most harp actions. \
What exactly you can figure out depending on your workflow!

You can also configure the hotkeys. Here are the default values:
```toml
[editor.harp.hotkeys]
global = "'"
directory = "."
buffer = ","
filetype = ";"
switch = "<space>" # toggle get/set mode
delete = "<backspace>" # switch to del mode
delete-all = "<A-backspace>" # delete all registers in the current section
```

The syntax is very similar to normal hotkeys, but the more complex ones you wrap in `<>`. \
Whenever you try to `get` a harp that isn't set, you'll get an error message that will show you the name of the key that you pressed, so you can use that to figure out the name.

The default is there to be reasonable, not to be *perfect*; don't be afraid to change the hotkeys to fit you better. \
Here is my config, for example:
```toml
[editor.harp.hotkeys]
global = "k"
directory = "l"
buffer = "j"
filetype = ";"
switch = "'"
delete = "<del>"
delete-all = "<S-del>"
```

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
