# Setting expectations

This is a strictly *personal* fork. \
I arbitrarily add, remove, and modify various parts of helix **with no warning or backup**.
If you particularly like one of my changes, I recommend saving it somewhere for yourself, as it might be gone the next time you check.

The commits are structured as a series of *patches* that I apply on top of helix upstream, rather than reflecting the *history* of my changes.
When I make a change, I'm likely to *edit* (amend) an already existing commit, rather than add a new one.

You might not like all of my modifications, so I recommend not installing the fork “whole”.
You can drop the commits that you dislike and build the fork in your preferred configuration.
Everything I change is documented in this readme.

I rebase on upstream helix master every time I see a new feature I want from it.

Join my [discord server](https://discord.gg/bgVSg362dK) if you want to follow the development of the fork.

If you want to contact me on discord without joining a server, my username is `axlefublr`.

# Installation

To use this fork, build it from source.

[Documentation on doing so](https://docs.helix-editor.com/master/building-from-source.html)

Here's my suggested installation snippet:
```sh
git clone https://github.com/Axlefublr/helix
cd helix
cargo install --path helix-term --locked
mkdir -p ~/.cargo/bin/
rm -fr ~/.cargo/bin/runtime
ln -sf $PWD/runtime ~/.cargo/bin/
```

The full path to the helix binary you'll get is `~/.cargo/bin/hx`, so you will be able to use `hx` in your shell if `~/.cargo/bin` is in your $PATH (it likely already is). \
The `helix` package on arch actually gives you the executable `helix`, rather than `hx`. Here you will get `hx` even if you are on arch.

Helix needs its `runtime/` directory for treesitter queries, grammars and themes.
I symlink the `runtime/` directory of your local copy of this fork, into the location where the `hx` binary will get installed.
Now whenever you update the fork, the `runtime/` bits will also get updated automatically.

In the future, when you want to update, you would:

```sh
git fetch origin
git reset --hard origin/master
cargo install --path helix-term --locked
```

Be careful though, as hard resetting might lose you features you like, that I ended up deleting on my end.

# This fork's changes

## Merged PRs from upstream

|Author|Link|Title|
|------|----|-----|
|@scdailey    |[9483](https://github.com/helix-editor/helix/pull/9483)  |Implement Kakoune's A-S: Select first and last chars|
|@pantos9000  |[10576](https://github.com/helix-editor/helix/pull/10576)|Add same_line and anchored movements|
|@EpocSquadron|[9843](https://github.com/helix-editor/helix/pull/9843)  |Indent text object|
|@omentic     |[14121](https://github.com/helix-editor/helix/pull/14121)|Add support for moving selected lines up and down|
|@useche      |[11700](https://github.com/helix-editor/helix/pull/11700)|Add per view search location and total matches to statusline|
|@rotmh       |[14093](https://github.com/helix-editor/helix/pull/14093)|Scrolling instead of paging in pickers|
|@yerlaser    |[14844](https://github.com/helix-editor/helix/pull/14844)|Find char with leap/easyMotion/flash style|
|@pascalkuthe |[14544](https://github.com/helix-editor/helix/pull/14544)|Implement file-watching based on filesentry|
|@kfatyuip    |[14072](https://github.com/helix-editor/helix/pull/14072)|Implement auto-scrolling bufferline|
|@atomicptr   |[13113](https://github.com/helix-editor/helix/pull/13113)|Differentiate buffers with same name by progressively adding prior path components|
|@maxsz       |[15100](https://github.com/helix-editor/helix/pull/15100)|Add buffer-close-next/previous commands|
|me✌️         |[13650](https://github.com/helix-editor/helix/pull/13650)|`:buffer-nth` command|
|me✌️         |[15187](https://github.com/helix-editor/helix/pull/15187)|`goto_hover` action|
|me✌️         |[15088](https://github.com/helix-editor/helix/pull/15088)|`select_character` action|
|me✌️         |[15188](https://github.com/helix-editor/helix/pull/15188)|`enable-siagnostics` option|
|me✌️         |[15199](https://github.com/helix-editor/helix/pull/15199)|`paste_before_all`, `paste_after_all` actions|
|me✌️         |[15266](https://github.com/helix-editor/helix/pull/15266)|Better ↑↓ history handling in prompts|
|me✌️         |[15271](https://github.com/helix-editor/helix/pull/15271)|Prompt history deduplication|

Big shoutout to these people!

I will not be explaining the additions / changes of these PRs at all; I expect you to click the links and read on your own, if you're interested.

---

Here's another table, of PRs I used as a starting point, but ultimately changed most of the code of.

|Author          |Link                                                       |Title                                       |
|----------------|-----------------------------------------------------------|--------------------------------------------|
|@nik-rev        |[12204](https://github.com/helix-editor/helix/pull/12204)  |New `statusline.merge-with-commandline` option|
|@oxcrow         |[13053](https://github.com/helix-editor/helix/pull/13053)  |Add local search in buffer (fuzzy and regex)|

*This* is more so a shoutout than a link for you to check as documentation; so the features that came from these will be explained later separately.

## Uncategorized

Tab width 4 -> 3. \
This only applies to the `text` filetype, and only if helix didn't notice a different indentation from the context of the file.

`-` is now also considered a word character.

All pickers now take up the entire screen, rather than 90%. \
Thanks to @satoqz for figuring out how to do this! :D

`indent` action now works even on empty lines.

`insert_tab` inserts the tab character always, not whatever the indent string is.

The `scrolloff` option is applied only vertically, not also horizontally.

`select_next_sibling`, `select_prev_sibling`, `extend_next_sibling`, `extend_prev_sibling` skip unnamed treesitter nodes, similar to how `select_all_children` does.

`save_selection` is now silent: doesn't print “Selection saved to jumplist” to the messages line.

`search` / `rsearch` / `search_next` / `search_prev` no longer shows “Wrapped around document”.
The thinking is that you'll be using the search count statusline element anyway.

No more startup “Loaded n files.” message.

No more status message on write.

No more status message on `:cd`.

## Scratch buffers

…are now more ephemeral in nature.

They are ignored in saving logic.
Meaning, `:write-buffer-close` will close a scratch buffer even if it has unsaved changes, and `:quit` will successfully do so if the buffer with unsaved changes is just a scratch buffer.

`:write-all` doesn't error with “cannot write a buffer without a filename” when there are scratch buffers with unsaved changes.

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

¹ If you have it in `$PATH`, and if the `auto-format` option is `true`. \
`:write --no-format` also skips this feature, expectably.

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

Scratch buffers no longer have the `[+]` indicator next to them if they have unsaved changes.
Most of my scratch buffers exist *to* have unsaved changes, so this is visually annoying.

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

Line number is right aligned to 4, column number is left aligned to 3.

```
   1:1
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

### smart-path

New element!

You current working directory is `~/fes/dot`.
|Current buffer|Resolves to|
|-|-|
|~/.local/share/magazine/f.md|~/.local/share/magazine|
|~/fes/dot/helix/generator.py|dot/helix|
|~/fes/dot/lazygit.yml|dot|

Displays the full path to the parent directory of the current buffer if it's outside of the current working directory, \
nothing if it's a scratch buffer, \
parent directory of the current buffer, with the prefix (everything before the basename of the current working directory) removed.

It's a useful element for disambiguation, that doesn't unecessarily repeat the filename that you can already see in the bufferline. And at the same time, explicitly shows the current working directory to you — something that normal relative paths implicitly hide.

## Editor

Dot repeat is disabled. I make this change in source rather than my config because the <kbd>.</kbd> is actually hardcoded, and is *not* mappable to something else without editing the source, fun fact!

When you close a buffer, if it has a path, your selection ranges are saved to `~/.cache/helix/selections.json`. \
When you open a file, your selections are restored to the ones saved in that file.

## Menu

Also known as the suggestion box.

The selected entry is always kept in the middle of the popup, if possible (scrolloff = 99).

↑ing at the first suggestion doesn't move you, ↓ing at the last suggestion doesn't move you either.

<kbd>Tab</kbd> **doesn't** cycle through suggestions.

## Hotkeys

Hover docs, pickers, menu, and possibly other overlays have <kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> hardcoded to mean "scroll by half a page down / up" \
I remove them because <kbd>ctrl+u</kbd> overrides the very useful "delete until the start of the line" mapping. \
Not being able to delete the entire text in a picker at once is very painful. \
You can use <kbd>PageDown</kbd> and <kbd>PageUp</kbd> instead.

### prompts

Command mode, pickers, etc.

<kbd>ctrl+v</kbd> pastes the contents of your `default-yank-register`. A shortcut to <kbd>ctrl+r "</kbd>, basically.

<kbd>ctrl+x</kbd> yanks your current input to `default-yank-register`. If it's empty but there is a history suggestion, copies that instead.

<kbd>alt+o</kbd> toggles surrounding your input with `\b` (toggling word bounds) \
`something` → `\bsomething\b` → `something` → …

<kbd>alt+i</kbd> toggles case sensitivity. Case sensitive → Case insensitive → Nothing. \
`something` → `(?-i)something` → `(?i)something` → `something`

### pickers

Movement, such as <kbd>↓</kbd>/<kbd>↑</kbd>/<kbd>PageUp</kbd>/<kbd>PageDown</kbd> no longer wraps: if you are at the final result in the picker and try to go down, nothing happens.

Due to <kbd>ctrl+v</kbd> now pasting, the hotkey to open the result in a vertial split is moved from <kbd>ctrl+v</kbd> to <kbd>ctrl+m</kbd>.

By default, <kbd>Home</kbd> and <kbd>End</kbd> jump to the first / last result, in the list of results. \
This once again overshadows very useful hotkeys, that let you go to the start/end of the line (your picker input) :/ \
Now, <kbd>ctrl+Home</kbd> and <kbd>ctrl+End</kbd> are used instead.

<kbd>alt+m</kbd> inserts ` %column `, where `column` is the next non-primary column name.

## Textobjects

In `select_textobject_around` (<kbd>ma</kbd>) and `select_textobject_inner` (<kbd>mi</kbd>) specifically.

Add `s` to mean paragragh, `q` to mean big WORD, `j` to mean \` (backtick), `k` to mean `_`, `i` to mean `*` (the defaults are kept too). \
Type is *moved* over to `c`, test is moved over to `t`, indentation is moved to `d`, comment is moved over to `v` (the defaults are destroyed / changed).

The whichkey ui wording is debloated:
```
┌Match inside────┐
│ w  Word        │
│ q  WORD        │
│ s  Paragraph   │
│ d  Indentation │
│ g  Change      │
│ m  Pair 󰌪      │
│ a  Argument 󰌪  │
│ e  Entry 󰌪     │
│ f  Function 󰌪  │
│ c  Type 󰌪      │
│ t  Test 󰌪      │
│ v  Comment 󰌪   │
│ x  Tag 󰌪       │
└────────────────┘
```

First, all text objects except word, WORD, paragraph force the selection *backwards*, **if** your selection range changes.

Second, you can now *repeat* the textobject selection, for all text objects except word, WORD, paragraph, (git) change, quotes `'`, `"`, \`, `*`, `_`.

> [!NOTE]
> `«` or `»` denotes the cursor position, `“` or `”` the anchor position (the other end of the selection)
> `█` means that the cursor position and anchor position are the same (1-width selection)

Let's first look at how the default / upstream helix works.
```
fn magic(text: &str) -> usize {
    let mage = |inner_text: &str| {
        inner_text.len()█
    };

    mage(text)
}
```
Here, you select around function (<kbd>maf</kbd>)
```
fn magic(text: &str) -> usize {
    let mage = “|inner_text: &str| {
        inner_text.len()
    }»;

    mage(text)
}
```
You have now selected the closure. If you try to select around function *again*, nothing happens.
When probably you've wanted to surround around the entire `magic` function this entire time.

But *in this fork*, if a textobject selection does not make your range any different¹, it extends the selection by 1 on the left, and tries again.

¹ If all that changed is the *direction* of the selection (for example due to the backward forcing), but not its *range*, this still counts as “the range didn't change”.

Let's retry the situation with this fork's changes now.
```
fn magic(text: &str) -> usize {
    let mage = |inner_text: &str| {
        inner_text.len()█
    };

    mage(text)
}
```
<kbd>maf</kbd>
```
fn magic(text: &str) -> usize {
    let mage = «|inner_text: &str| {
        inner_text.len()
    }”;

    mage(text)
}
```
We have selected the closure like before, but notably now the cursor is on the *left* of the selection. \
We can now more clearly tell what closure we are selecting, rather than the “ah yes I am selecting some `}`” that you get in upstream behavior. \
Let's <kbd>maf</kbd> another time.
```
«fn magic(text: &str) -> usize {
    let mage = |inner_text: &str| {
        inner_text.len()
    };

    mage(text)
}”
```
Since we already had the closure exactly selected, the selection extended by one character to the left, and tried again. \
We are now selecting the entire `magic` function! \
Now if we try <kbd>maf</kbd> *again*, here's what happens.

The initial “select around function” doesn't result in a selection change: we are already selecting around function (like we also were with the closure previously). \
So the selection extends by one character to the left (onto the previous line, if needed), and tries to select around function from there. \
(in this example) the `magic` function is *not* inside of another function, so that attempt doesn't result in a selection change either. \
So we restore the initial selection, which you had before you pressed the most recent <kbd>maf</kbd>. It is unmodified, even the direction is retained. \
As the user, what you see is “nothing happens” — it tried to select the another surrounding function, but since it doesn't exist, you end up just continuing to select around the `magic` function.

*Crucially*, you don't have to keep spamming <kbd>maf</kbd> over and over again: `repeat_last_motion` **works** so you can press <kbd>maf</kbd> the first time, and then keep pressing <kbd>A-.</kbd> afterwards, to continue selecting the bigger and bigger function.

I say “function” but I'm just taking it as an example. The same exact idea is applied to the other textobjects as well.
The most satisfying to use one being the indentation and “closest pair” text objects.

When you use inside (<kbd>mi</kbd>) instead of around (<kbd>ma</kbd>), the “reach” is *two* characters to the left, not one (so that the expanding behavior works there too).

As for quotes, `*`, `_`. Say you have `'something'` selected, and want to have `something` selected. In upstream, nothing would happen if you pressed <kbd>mi'</kbd>, but it *does* work in this fork.
The *how* of it working is different from all the other textobjects explained above. \
If you have a forward (cursor is on the right) selection, then it is shrunk backwards until the selection is a block cursor, *or* until you come across a `'`¹.
Then, another step left is made.
From that point, <kbd>mi'</kbd> is ran again. \
If your selection is backwards (cursor is on the left), the selection is shrunk forwards, …, <kbd>mi'</kbd> is ran again. \
‼️This “searching” step is only done if the *initial* <kbd>mi'</kbd> doesn't change your selection range.

¹ If you come across a `'` but the next character is *also* a `'`, you keep going instead of stopping. Essentially, doing <kbd>mi\*</kbd> while selecting `**something**` will result in selecting `something`.

Lastly, `goto_next_` treesitter-based actions all rotate the selection backwards. Never have I wanted to go to the next function, only to stare at its ass.

## New commands
Typable command that you execute from command mode.

### random
Randomizes the order of your selections. Has aliases `:rnd`, `:rng`.

### uniq
Only retains a selection, if its text hasn't appeared already. \
If you select each line in the following text:
```
extraordinary
extraordinary
magnificent
announced
announced
designers
desktop
```
`:uniq` will remove selections of the second `extraordinary` and second `announced`: because there was already a selection of each of them. All the other selections stay.

### echopy
Command is exactly like `:echo`, but *also* puts the result into your `default-yank-register`. Has alias `:cc`.

### run-shell-command-quiet
With aliases `?` and `shq` is exactly like `:run-shell-command`, but doesn't show the output of the command.

## New actions

(except [harp](#harp-integration), which is explained in a later section)

### continue_last_insert
When you enter normal mode from insert mode, your selections are saved.
Use this action to jump to the saved selections, and start insert mode there.

### marks
Every document has a set of selections it may store.

`mark_add` to add all current selections to that set. \
`mark_replace` to clear the set and add all current selections to it. \
`mark_apply` to retrieve all stored selections from the set, and replace the current selections with them.

Any stored selections (marks) that are out of range because the document became smaller since you added that selection, are filtered out / removed.

### extend_prev/next_sibling
*Create* new selections, unlike `select_next_sibling` and `select_prev_sibling` that only move existing ones.

### toggle_line_select
Does `extend_to_line_bounds`. If none of your selections change, it also does a `trim_selections`. \
With this action, you can use a single hotkey for both expanding and shrinking your selection.

### surround_add_tag
Prompts you with the name of an html tag, and surrounds your selections with it. \
You have `word`, type in `div`, and get `<div>word</div>`. \
The history for it is stored in the `<` register.

### registers manipulation
Usually, you retrieve a register for every action that you want to do with it, rather than getting it once and reusing it. \
`copy_register_to_yank` lets you pick a register to copy the contents of, into the default yank register. \
`copy_yank_to_register` is the opposite: pick a register to copy the default yank register contents *into*.

With these two on custom mappings, you get a much nicer register workflow I think.
I usually realize I want something to be in some register only *after* I've already copied it normally. <kbd>y</kbd>, “whoops”, <kbd>"ay</kbd>. \
But now I can more conveniently play into this: <kbd>y</kbd>, “oh right I want this in a register”, *boom*. \
On the opposite end, I can now grab the contents of some register and then continue to reuse it over and over again, rather than invoking the register picker for every action, which I always found *so* annoying that I end up never using it.

### retain_column
Is better shown with an example.
```
jenny, provider, referred, circuit,
sought, restaurant, crowd, circuit,
stopped, symposium, density, circuit,
flexible, corps, concerned, madrid,
```
Say you have this text, and want to hyphenate the words of the second and third column, to get this:
```
jenny, provider-referred, circuit,
sought, restaurant-crowd, circuit,
stopped, symposium-density, circuit,
flexible, corps-concerned, madrid,
```
We first select all the lines and use `select_character` to select all `,`s. Now we want to trim / remove all the selections that *aren't* the commas after the second words. We want to only keep the second *column* of selections. \
Once we can do that, we can `dc-` to change the `, ` to a `-` to complete the hyphenation.

So, `retain_column` is what you can use for that “keep only the nth column” part. \
You specify which column to keep with the position of your **primary selection**. So you can `rotate_selections_forward` once to get yourself onto the overall second comma (or the 6th, 10th, 14th). \
Then you call `retain_column` and it asks you *how many columns there are*. In this case there are 4, so you type `4`, press <kbd>Enter</kbd>, and blammo — you're left selecting only the second column of commas / only the commas after the second words.

Incidentally, this action can *also* be used to mean the more general “only keep every nth selection” — I made this action to make columnar selections nicer to work with, but “columns” don't have to be all on the same line to be valid.

Let's instead of doing `dc-` call `retain_column` *again* and now provide `2`.
```
jenny, provider, referred, circuit,
sought, restaurant, crowd, circuit,
stopped, symposium, density, circuit,
flexible, corps, concerned, madrid,
```
We would end up selecting only the commas after `provider` and `symposium`.
Even though the “columns” were vertical rather than horizontal in this case, the action worked just as well.

### local_search_fuzzy
Lets you fuzzy search the contents of the *current* buffer with a picker. \
You cannot open the resulting line in a split like you usually can — only the “normal” open (<kbd>Enter</kbd>) is supported. \
As an upside of this, this picker will work even if the buffer doesn't have a path.

### local_search_section
Lets you search through the list of “sections”, with a picker. \
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

First, you will will see all headings of level 1, then all of level 2, then …, etc. \
Sacrificing in-file order to get way easier visual scanning, and make searching more scalable.

## New options

All are in the `[editor]` section.

`whichkey` can be set to `true` (default) or `false`. \
If set to `false`, the infoboxes for *mappings* will not show up. \
This is different from just disabling the `auto-info` option in that you will still get the popup for `select_register` and `select_textobject_inner` / `select_textobject_around`.

`harp` subsection, that will be explained in the harp section below.

### shellmap

This option allows you to use multiple different shells ergonomically.
With this config:
```toml
[editor.shellmap]
"€" = ['nu', '-c']
"°" = ['fish', '-c']
```
I define *prefixes* that switch the current commandline to be using a different shell.

`:sh printf hello` will use the shell I have configured with `editor.shell` as usual. \
`:sh €ls | sort-by modified | first | get name` lets me use nushell directly thanks to the `€` prefix. No need to type out `nu -c`, no need to be bothered by adding a layer of quoting with `nu -c '…'`. \
`:sh °math 3 + 5` is me using fish shell's `math` builtin.

This works for *everything* that expects a shell command. `:sh`, `:pipe`, `shell_pipe`, `keep_selections`, etc etc.

You can even add programs that aren't necessarily shells:
```toml
[editor.shellmap]
"×" = ['qalc']
```
Now with `:insert-output ×3 + 5` I have an ergonomic to use calculator.

## Command expansions

First, the new ones I added.

Considering our example context...

|Thing                    |Value                                            |
|-------------------------|-------------------------------------------------|
|current buffer           |`~/prog/dotfiles/fish/abbreviations/percent.fish`|
|current working directory|`~/prog/dotfiles`                                |

...here's what the added command expansions would evaluate to:

|Expansion             |Output                                           |Explanation                             |
|----------------------|-------------------------------------------------|----------------------------------------|
|`%{relative_path}`    |`fish/abbreviations/percent.fish`                |buffer path, relative to current working directory|
|`%{buffer_parent}`    |`~/prog/dotfiles/fish/abbreviations`             |parent directory of the current buffer|
|`%{buffer_stem}`      |`percent`                                        |buffer basename without extension|

> [!CAUTION]
> The path evaluates to have `~`, instead of `/home/username` \
> This is because paths starting with `~` are accepted everywhere in helix, and when used in your shell (like with `:sh`), will be expanded as well. \
> But, in the rare case where it doesn't expand, beware of this behavior. \
> I'm taking this tradeoff to see `~/r/dot` instead of `/home/axlefublr/r/dot` in my various mappings that `:echo` expanded paths.

`%{indentation}` resolves to the string representation of the buffer's `indent-style`.

`%{selection_line_start_indentation}` is like `%{selection_line_start}`, but instead of the line number, it resolves to the string of the leading whitespace of the line.

I also change some of the default command expansions.

`file_path_absolute` renamed to `full_path` and made to fold `/home/username` to `~`. \
`current_working_directory` renamed to `working_directory` made to fold `/home/username` to `~`.

`buffer_name` is now *actually* the buffer name: \
`~/fes/dot/helix/generator.py` → `generator.py` \
While previously, what we would get would depend on what your current working directory is. Now it's just the name / basename.

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

There is another mode, that is currently only used for one action: `alt`. \
It's meant to be `get`, but that does an *alternative* behavior. \
You can get into it by pressing <kbd>/</kbd>, and leave it (go back to `get`) by pressing <kbd>Space</kbd>.

If an action only has the `get` behavior defined and you try to use `alt`, it'll effectively just act like `get`.

### File harps

```
harp_file
```

`set` takes the *current buffer*'s¹ filepath, and stores it in a harp. \
`get` takes it, and `:open`s it.

When using the buffer relativity, the most recent *other* buffer's path is taken instead.

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
`get` takes a stored search pattern, puts it back into register `/` and effectively presses <kbd>n</kbd> for you (`search_next` action). \
Note that it does `save_selection` first, so you can jump back to undo the <kbd>n</kbd>.

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
`get` executes a stored command mode command and writes it to the `:` register. \
`alt` puts a stored command mode command into a *prompt*, for you to possibly *edit* and then execute (instead of immediately executing it for you).

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
alt = "/" # switch to alt mode
delete = "<backspace>" # switch to del mode
delete-all = "<A-backspace>" # delete all registers in the current section
```

The syntax is very similar to normal hotkeys, but the more complex ones you wrap in `<>`. \
Whenever you try to `get` a harp that isn't set, you'll get an error message that will show you the name of the key that you pressed, so you can use that to figure out the name.

The default is there to be reasonable, not to be *perfect*; don't be afraid to change the hotkeys to fit you better. \
Here is my config, for example:
```toml
[editor.harp.hotkeys]
global = ","
directory = "k"
buffer = "j"
filetype = "l"
switch = "'"
alt = ";"
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
a custom renderer (similar to Emacs) using wgpu.

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
