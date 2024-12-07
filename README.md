# Setting expectations

This is a strictly *personal* fork.
I arbitrarily add, remove, and modify various parts of helix **with no warning or backup**.
Everything I change is documented in this readme.

Normally I explain *only* the “what”.
To see the “why”, you can expand the “Reasoning” section right after the “what” explanation (if present).
This way, you don't waste your time parsing the whats from the whys, making it easier to skip reading sections that you don't care about / the reasoning of which is self-evident to you.

I rebase on upstream helix master every time I see a new feature I want from it.

I do not vibecode. All of my patches are written manually. \
I can't speak as absolutely for PRs that I merge that are not mine; if they're *clearly* AI, I obviously won't merge them. But I might miss the less clear cut cases.

Join my [discord server](https://discord.gg/bgVSg362dK) if you want to follow the development of the fork.
If you want to contact me on discord without joining a server, my username is `axlefublr`.

# Commit semantics

The commits are a series of *patches* that I apply on top of helix upstream, rather than reflecting the *history* of my changes.
When I make a change, I'm likely to *edit* (amend) an already existing commit, rather than add a new one.

You might not like all of my modifications, so I don't expect you to install the fork “whole”.
You may decide to drop some commits, or only include some commits. \
To make it easier for you to relate a change to the commit it's stored in, every section in this readme will have a “Commit semantics” section.

But on the high level, commits are grouped into sections, that more or less reflect the sections of this readme.
The groups are made with ‘conventional commits’-like prefixes.

Generally, any one commit can be yanked on its own, but this is a rule of thumb, not a promise.
If a commit is out of its expected section, this probably means that it depends on the commit right before it.

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

Be careful though, as hard resetting might lose you features you like, that I ended up deleting on my end. \
You may be able to retrieve them from one of the backup branches, but I wouldn't trust that in your shoes.

# Merged PRs

|Author|Link|Title|
|------|----|-----|
|@scdailey    |[9483](https://github.com/helix-editor/helix/pull/9483)  |Implement Kakoune's A-S: Select first and last chars|
|@pantos9000  |[10576](https://github.com/helix-editor/helix/pull/10576)|Add same_line and anchored movements|
|@EpocSquadron|[9843](https://github.com/helix-editor/helix/pull/9843)  |Indent text object|
|@omentic     |[14121](https://github.com/helix-editor/helix/pull/14121)|Add support for moving selected lines up and down|
|@useche      |[11700](https://github.com/helix-editor/helix/pull/11700)|Add per view search location and total matches to statusline|
|@rotmh       |[14093](https://github.com/helix-editor/helix/pull/14093)|Scrolling instead of paging in pickers|
|@kfatyuip    |[14072](https://github.com/helix-editor/helix/pull/14072)|Implement auto-scrolling bufferline|
|@atomicptr   |[13113](https://github.com/helix-editor/helix/pull/13113)|Differentiate buffers with same name by progressively adding prior path components|
|me✌️         |[13650](https://github.com/helix-editor/helix/pull/13650)|`:buffer-nth` command|
|me✌️         |[15187](https://github.com/helix-editor/helix/pull/15187)|`goto_hover` action|
|me✌️         |[15088](https://github.com/helix-editor/helix/pull/15088)|`select_character` action|
|me✌️         |[15188](https://github.com/helix-editor/helix/pull/15188)|`enable-diagnostics` option|
|me✌️         |[15199](https://github.com/helix-editor/helix/pull/15199)|`paste_before_all`, `paste_after_all` actions|
|me✌️         |[15266](https://github.com/helix-editor/helix/pull/15266)|Better ↑↓ history handling in prompts|
|me✌️         |[15271](https://github.com/helix-editor/helix/pull/15271)|Prompt history deduplication|
|@jamel       |[15309](https://github.com/helix-editor/helix/pull/15309)|Add cursor position persistence across sessions|
|@RoloEdits   |[15573](https://github.com/helix-editor/helix/pull/15573)|Breadcrumbs|
|me✌️         |[16316](https://github.com/helix-editor/helix/pull/16316)|User-defined snippets|
|me✌️         |[16354](https://github.com/helix-editor/helix/pull/16354)|:char displays the unicode category as well|
<!-- |@pascalkuthe |[14544](https://github.com/helix-editor/helix/pull/14544)|Implement file-watching based on filesentry| -->

Big shoutout to these people!

I will not be explaining the additions / changes of these PRs at all; I expect you to click the links and read on your own, if you're interested.

<details>
<summary>Commit semantics</summary>

A commit with a pr in it may be in one of the other sections, or on its own, but will for sure have the following in the commit message: `<pr-number>/<shortened-title>/<author>`. \
*Unless* it's my pr, in which case I might have forgotten to update the commit where it rests.
But if I didn't forget, the format will be `<shortened-title>/<pr-number>` (note how the author is omitted here, cause it's me 😼).

</details>

<!-- ━━ -->

Here's another table, of PRs I used as a starting point, but ultimately changed most of the code of.

|Author          |Link                                                       |Title                                       |
|----------------|-----------------------------------------------------------|--------------------------------------------|
|@nik-rev        |[12204](https://github.com/helix-editor/helix/pull/12204)  |New `statusline.merge-with-commandline` option|
|@oxcrow         |[13053](https://github.com/helix-editor/helix/pull/13053)  |Add local search in buffer (fuzzy and regex)|

*This* is more so a shoutout than a link for you to check as documentation; so the features that came from these will be explained later separately.

# Unique changes

## Internals

Commit semantics: `dog:` prefix.

Default tab width (visual) 4 -> 3. \
This only applies to the `text` filetype, and only if helix didn't notice a different indentation from the context of the file.

`-` is now also considered a word character.

The `scrolloff` option is applied only vertically, not also horizontally.

Searching from normal mode targets from the *cursor position*, not the entire selection.

No more startup “Loaded n files.” message.

Opening a *directory* rather than a file, either by passing it on the commandline or with `:open`, will run `global_search` on that directory instead of a file picker.

Autosave on focus lost only works if you aren't in insert mode.

<details>
<summary>Reasoning</summary>

It's very frequent that as I'm writing some text, I leave off with a space at the end of the line; anticipating to write yet another word.
Then I switch to a different window for one reason or another, switch back to helix, and now need to retype that space again.
This annoys me *so much* that without this change autosave on focus lost is genuinely unusable to me 😭

</details>

Writing doesn't print a message to the statusline.

<details>
<summary>Reasoning</summary>

In my config I have autosave on leaving insert mode, and on opening a new line in insert mode.
The message gets annoying QUICK!

</details>

## Changed behavior of actions

Commit semantics: `beh(<action>):` prefix.

`indent` works even on empty lines.

<details>
<summary>Reasoning</summary>

It's incredibly common for me to start a new line, think “nope, this is not the intentation I want”, and want to fix it *immediately*.
Offensively, helix went out of its way to prevent you from being able to do so??!

</details>

`insert_tab` inserts the tab character always, not whatever the indent string is.

`signature_help` *is* actually closed when you press <kbd>Escape</kbd> to exit insert mode.

`save_selection` is silent: doesn't print “Selection saved to jumplist” to the messages line.

`goto_file` also understands the `:line:col` syntax.

## Changed behavior of commands

Commit semantics: `beh(:<command>):` prefix.

`:char` renders each codepoint as `1f404` instead of `U+1f404`.

`:write-buffer-close`, `:write-buffer-close!`, `:buffer-close`, `:buffer-close!` have a new `--focus-right` flag.
With it, instead of activating the most *recently accessed* other buffer after closing the current one, the buffer *directionally on the right* will be focused.

The former two commands now only write the buffer if there's a reason to: unsaved changes or the buffer isn't for sure formatted (you'll see below).

`:write-all`, `:write-all!`, `:write-quit-all`, `:write-quit-all!` have a new flag `--unmodified`, which makes sure to write *every*¹ buffer, not only those with modifications. They become similar to calling `:write` on every buffer that you have open.

<details>
<summary>Reasoning</summary>

With autosave enabled, you really don't want the formatters to run.
Maybe you haven't finished writing that one function or expression, and the formatter “fixing” things for you just because you switched to a different window would be very annoying.
However with autosave, buffers don't *stay* modified for the formatters to act up on when you finally decide to write them. \
So what is possible in this position, is that all of your buffers are saved when you run `:write-quit-all`, and files *stay* unformatted even though you expect them to be.
Making your `:write-all` hotkeys write every buffer makes sure you can't (ish) get in a situation where you push unformatted code.

</details>

¹ Since the usecase is making sure formatters run on already saved files, there is an optimization.
Separately from modified state, `forsure_formatted` is also tracked. \
Writing to a particular file (including with `:write-all`) without *disabling* autoformatting sets its `forsure_formatted` to true.
A particular *language* may have its autoformatting disabled, but if you have autoformatting set to true globally and don't `--no-format` in your actual write command, that counts as “without disabling” as well. \
Buffers start out with `forsure_formatted` set to false when first opened, and it also resets to false when you `:reload` that file. \
In other words, unknown contents are assumed to not be formatted, and are only considered formatted when the file doesn't have unsaved changes AND its last write was a formatting one.

## Scratch buffers

Commit semantics: `scratch:` prefix.

Scratch buffers are now more ephemeral in nature.
They participate in what happens *on write* ([global formatter](#global-formatter) for example) but a write is never attempted.
Unless the scratch buffer was given a name via `:e somename` or `:w somename`.

So in normal usage, `:write-buffer-close` will close a scratch buffer even if it has unsaved changes, and `:quit` will successfully do so if the buffer with unsaved changes is just a scratch buffer.

`:write-all` doesn't error with “cannot write a buffer without a filename” when there are scratch buffers with unsaved changes.

## Editor

Commit semantics: `editor:` prefix.

Dot repeat is disabled.

<details>
<summary>Reasoning</summary>

The <kbd>.</kbd> hotkey is actually hardcoded, and is *not* mappable to something else without editing the source, fun fact!
It's also much less needed / useful in helix, so I found myself never using it, so I don't bring back this functionality anywhere, it's just removed 🤷‍♀️

</details>

<!-- ⚠️ -->

## Prompt

Commit semantics: `prompt:` prefix.

Command mode, pickers, etc.

<kbd>ctrl+v</kbd> pastes the contents of your `default-yank-register`. A shortcut to <kbd>ctrl+r "</kbd>, basically.

<kbd>ctrl+x</kbd> yanks your current input to `default-yank-register`. If it's empty but there is a history suggestion, copies that instead.

<kbd>alt+o</kbd> toggles surrounding your input with `\b` (toggling word bounds) \
`something` → `\bsomething\b` → `something` → …

<kbd>alt+i</kbd> toggles case sensitivity. Case sensitive → Case insensitive → Unspecified (meaning, falls back on smartcase if you have it enabled etc). \
`something` → `(?-i)something` → `(?i)something` → `something`

<kbd>shift+Insert</kbd>¹ toggles `.` matching the newline character. \
`something` → `(?s)something` → `something` \
Because there is no mechanic to affect this regex flag in helix, you only have specified and not specified, as not specified is equivalent to `(?-s)`.

¹ I did tell you that this is a personal fork. I know that this hotkey makes no sense to a normal human, but it does for me 😋😼

<kbd>Insert</kbd> types `.*?`.

<kbd>Ctrl+o</kbd> types `\n`.

<kbd>Alt+j</kbd> types `[\w-]+`.

## Picker

Commit semantics: `picker:` prefix.

All pickers now always take up the entire screen (while still showing the statusline), rather than 90%. \
Thanks to @satoqz for figuring out how to do this! :D

Borders and padding are removed to make the ui more dense.

<kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> that used to scroll by half a page are now removed.
The <kbd>PageUp</kbd> and <kbd>PageDown</kbd> hotkeys are changed to be <kbd>shift+PageUp</kbd> and <kbd>shift+PageDown</kbd>.

<details>
<summary>Reasoning</summary>

<kbd>ctrl+u</kbd> overrides the incredibly useful “delete until the start of the line” mapping. \
Being unable to delete the entire text you've typed in at once, in a picker for example, is very painful.

The page hotkeys suddenly getting a modifier (<kbd>Shift</kbd>) is to make it possible for shiftless page keys to scroll the preview; this is currently not yet implemented, and might never will, but I don't want to develop a muscle memory that I intend to break.

</details>

Movement, such as <kbd>↓</kbd>/<kbd>↑</kbd>/<kbd>shift+PageUp</kbd>/<kbd>shift+PageDown</kbd> no longer wraps: if you are at the final result in the picker and try to go down, nothing happens.

Due to <kbd>ctrl+v</kbd> now pasting, the hotkey to open the result in a vertial split is moved from <kbd>ctrl+v</kbd> to <kbd>ctrl+m</kbd>.

By default, <kbd>Home</kbd> and <kbd>End</kbd> jump to the first / last result, in the list of results. \
This once again overshadows very useful hotkeys, that let you go to the start/end of the line (your picker input) :/ \
Now, <kbd>ctrl+Home</kbd> and <kbd>ctrl+End</kbd> are used instead.

<kbd>alt+m</kbd> inserts ` %column `, where `column` is the next non-primary column name.

## Menu

Commit semantics: `menu:` prefix.

Also known as the suggestion box.

The selected entry is always kept in the middle of the popup, if possible (scrolloff = 99).

↑ing at the first suggestion doesn't move you, ↓ing at the last suggestion doesn't move you either.

<kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> that used to scroll by half a page are now removed.
The <kbd>PageUp</kbd> and <kbd>PageDown</kbd> hotkeys are changed to be <kbd>shift+PageUp</kbd> and <kbd>shift+PageDown</kbd>.

<kbd>Tab</kbd> **doesn't** cycle through suggestions.

<kbd>Escape</kbd> fucking works. Pressing escape will CANCEL the selected completion (DUH!) and exit insert mode.

<kbd>Space</kbd>, <kbd>,</kbd>, <kbd>Right</kbd>, <kbd>End</kbd> will accept the currently selected suggestion and also do their usual action (if any is bound for the last two).
Note that by default when a suggestion box is open, no suggestion is *selected*. You select one by moving down and up yourself.

## Popup

Commit semantics: `popup:` prefix.

<kbd>ctrl+d</kbd> and <kbd>ctrl+u</kbd> that used to scroll by half a page are now removed. \
Here finally the <kbd>PageUp</kbd> and <kbd>PageDown</kbd> are *not* removed.

<details>
<summary>Reasoning</summary>

Previously, the same hotkeys were used to scroll the popup *and* to scroll the menu (completion box) from where you got that popup (documentation) to appear to begin with. But now you can use <kbd>PageUp</kbd> and <kbd>PageDown</kbd> to scroll the documentation yet use <kbd>shift+PageUp</kbd> and <kbd>shift+PageDown</kbd> to scroll the completions list. \
...Theoretically 👀 I'm pretty sure this doesn't yet work correctly, but the “hotkey infrastructure” is now in play for me to make it work in the future.

</details>

## New options

Commit semantics: `option:` prefix with the name of the option afterwards.

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

## New actions

(except [harp](#harp-integration), which is explained in a later section)

Commit semantics: `action:` prefix, with the name of the action afterwards.

### continue_last_insert
When you enter normal mode from insert mode, your selections are saved.
Use this action to jump to the saved selections, and start insert mode there.

### steal_char_above, steal_char_below
Intended for insert mode. Type the character right above/below you (in terms of column).
```
auburn pairs honey
▌
entrepreneurs accompanied comparative
```
With `▌` representing the cursor, `steal_char_above` would first type `a`, then `u`, then `b`, etc. \
`steal_char_below` would first type `e`, then `n`, then `t`, etc.

Once the above/below character is a newline / there is no character at all, nothing is typed. So you can safely hold the key to completely retype the above/below line if you want to.

```
chronicle assessments identified syndrome imaging
	appreciate elementary macintosh musician
sharp perspectives petroleum hearing
```
With the cursor placed at the beginning of the line after indentation but before the first character, using the actions results in the following character being typed:
|line|above|below|
|-   |-    |-    |
|1   |❌   |a    |
|2   |c    |s    |
|3   |a    |❌   |

Indentation is ignored, essentially.

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

`---` is ignored and not included in that tree — you can't consistently guess what level it actually implies, the heuristics I tried end up making your tree all fucked up in some markdown documents. \
Instead, you can use an amount of `━`s to express a titleless section marker, that does show up in your tree. \
`━` means level 1, `━━` means level 2, etc until 6, just like `#` headings.

Both `#` headings and `━` sections are included even if you commented them out with `<!--` + `-->`, so you can add a section for *yourself* for navigation, without including it in the final rendered markdown. \
Matter of fact I'm doing this in this readme you're reading right now! The `---` that separates my fork's readme from upstream's readme, has a `<!-- ━ -->` right below it.

## New commands
Typable command that you execute from command mode.

Commit semantics: `command:` prefix with the name of the command afterwards.

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

## Autopairs

Commit semantics: `autopairs:` prefix.

By default, `mi(` and `mi)` are equivalent: both will select `(this_text)`. \
Now, `mi(` will select `(this_text)` and `mi)` will select `)this_text(`. \
This new behavior is shared across all the other bracket pairs.

Typing the closing delimiter (if it's the same as the opening delimiter) no longer skips over it — you simply insert another closing delimiter. \
`(|)` → *types `)`* → `()|` \
`"|"` → *types `"`* → `""|""`

## Textobjects

Commit semantics: `textobjects:` prefix.

`goto_next_paragraph` is symmetric with `goto_prev_paragraph`. Instead of grabbing trailing empty lines after the paragraph, it stops at the *end* of the paragraph.

All text objects except word, WORD, paragraph force the selection *backwards*, **if** your selection range changes.

`goto_next_` treesitter-based actions all rotate the selection backwards. Never have I wanted to go to the next function, only to stare at its ass.

### sibling navigation

Commit semantics: `beh(select_…_sibling)` prefix (literally, `…` is not a placeholder).

`select_next_sibling`, `select_prev_sibling`, `extend_next_sibling`, `extend_prev_sibling` are reworked significantly. In my below explanations I'll always be saying “next”, but read it as “previous or next”.

The behavior of upstream is: try to go to the next sibling.
If that fails, expand to the parent and try to go to the next sibling from there.
Repeat this process until you either succeed going to the next sibling or fail to expand to upper parent (because you're selecting the entire file).

Now, sibling movement NEVER automatically expands you to the parent.
If going to the next sibling fails, nothing happens.

<details>
<summary>Implementation detail</summary>

Okay technically you *may* get expanded to the parent, but ONLY if the selection range of that parent is *exactly* the same.

Taking this code,
```rust
let sibling = match direction {
    Direction::Forward => current.next_sibling(),
    Direction::Backward => current.prev_sibling(),
};
```
And running `:tree-sitter-subtree` on `Direction::Backward => current.prev_sibling(),`, you get this:
```scheme
(match_arm
  pattern: (match_pattern
    (scoped_identifier
      path: (identifier) "::"
      name: (identifier))) "=>"
  value: (call_expression
    function: (field_expression
      value: (identifier) "."
      field: (field_identifier))
    arguments: (arguments "(" ")")) ",")
```
Running `:tree-sitter-subtree` on `Direction::Backward`, (the highest selection you can get without selecting the entire branch, via using `expand_selection`), is:
```scheme
(scoped_identifier
  path: (identifier) "::"
  name: (identifier))
```

The `match_pattern` is a sibling to `call_expression`, but `scoped_identifier` is *not* a sibling to `call_expression`. \
So if we try to `select_next_sibling`, that would normally fail.

They are different nodes, but the actual *selection range* of both `match_pattern` and `scoped_identifier` is exactly the same!
That's why parent expansion *can* happen automatically, but only when it doesn't change the selection range.

</details>

<details>
<summary>Reasoning</summary>

As I already eluded to, upstream sibling movement often leads you to overselecting.
Often overselecting up to the *entire file*, with frustratingly no way to undo that selection change.

Because of this, sibling navigation feels stressful: one wrong step and BOOM you're in a completely wrong spot from where you were just now. \
In my fork, *you* are expected to expand to parent (`expand_selection`) when you want to be targetting a higher element than you are currently targetting.
Thanks to this promise, you can gleefully spam sibling navigation in for example an *array*, and not have to worry about escaping out of the array and unexpectedly selecting the entire file.

</details>

Second, unnamed nodes (`,`s between values in an array for example) and “extra” nodes (comments) are *skipped* in sibling navigation.
<details>
<summary>Reasoning</summary>

A common usecase I have, which was previously unusable: in a class (or `impl` block I guess), I select the entire definition of a function, and then try to use sibling navigation¹ to look through all the other functions² in that class.
Except in all the languages where this concept applies, doccomments above the function count as a sibling, *and* each line comment is its own separate node; so I go from one function to another and then need to manually jump through like 25 separate nodes until I meet another function (what I was trying to get to, to begin with). \
Now all of those comments will be skipped, and you can reliably travel across all the *important* things on the same semantical level.

As for skipping unnamed nodes: commas in between arguments inside of brackets count as an equal sibling to the arguments.
So while by all accounts you don't give two shits about the commas, you'll have to sift through them normally.

Even more crucially: the `extend_` variants of sibling navigation give you a really pleasant way to select a couple of values and then reposition them easily, using actions like `rotate_selection_contents_forward`, `rotate_selection_contents_backward`, `reverse_selection_contents`.
But upstream, you would be creating those extra selections on *commas* as well; which makes the entire ux really ass.

¹ Note that doing `goto_next_function` and then continuing to traverse via `repeat_last_motion` is ass because you'll often be landing on closures *inside* of functions, rather than necessarily the next outer function consistently.

</details>

To explain the last crucial change, I'll introduce two terms.
“Treesitter Node Selection” is a selection that 1:1 correlates to some treesitter node; not smaller or bigger than it.
For example in this array:
```toml
key = ['pursuant', 'perfume', 'remaining']
```
selecting `'remaining'` is a valid TNS (Treesitter Node Selection) — it correlates to the `(string)` treesitter node. \
However, selecting `'remain` is NOT a valid TNS — it is only partially selecting some treesitter node.
In this case that “some” is the `(string)` still. \
But if you were selecting `, 'remain`, this would be an invalid TNS that is partially selecting the node of the entire array (`(array)`).

Second: a valid TNS is not necessarily a *desired* TNS.
We skip unnamed nodes and comments, so if a TNS is of an unnamed node or a comment, it is an “undesired TNS” (even if a valid one).

Okay so: sibling navigation from invalid TNS is now more natural.

If your selection is not a valid TNS, a treesitter node is selected from your *cursor* (not selection). \
If after that the TNS is not a *desired* one, you continue on forwards until you're on a *desired* TNS. \
If there are no future desired TNS, you aren't moved at all; you are left at your cursor getting turned into a valid (but undesired) TNS.

If your selection is not a valid TNS *and* your cursor is on *whitespace*, it gets moved forwards until it's on non-whitespace, and that node is selected.
Similarly if it is *undesired*, you will get moved to the next desired TNS, if any exist.

If your selection was a valid TNS to begin with, sibling navigation works like you're used to (except that it now skips undesired nodes and doesn't autoexpand into parent on failure, of course).

Take a look at the screen recording where I showcase these behaviors:

https://github.com/user-attachments/assets/7be93d4a-6f12-47c5-b752-bfbdea7ac78c

Sibling navigation no longer forces you into any particular direction. It will retain the direction of the selection that you used to have before calling the action; but defaulting on backwards for 0-width selections (this is normally impossible).

<details>
<summary>Recommendation</summary>

Make your `expand_selection` hotkey be this instead:
```toml
C-k = ['expand_selection', 'ensure_selections_forward', 'flip_selections']
```
When you're going upwards, the most useful info is the *start* of the symbol that you're now selecting.
“What block / closure / function / class am I in?”

With your `expand_selection` hotkey ensuring a backwards selection for you, you get your *sibling navigation* to be backwards-forced by default as well.
“Why not backward-force sibling navigation too?” will be made clear in the [`shrink_selection`](#shrinkselection) section.

</details>

### shrink_selection

Commit semantics: `beh(shrink_selection)` prefix.

When you `shrink_selection` right after `expand_selection`, you essentially “undo” your selection change, coming back to whatever element you were selecting previously (upstream behavior).
However when it can't “undo”, or it doesn't make sense to, it will drop you onto the *first* child of your TNS, even if your selection is forward, rather than backward.

In my fork, in that “can't undo” situation, `shrink_selection` *cares* about your selection direction.
If it's forwards, you'll be placed on the *last* child. Otherwise, on the first child. \
In both cases, first/last *desired* child (lol).
So if you're shrinking from the `{}`s of an `if`, you'll land at the first statement in that body, instead of on the `{` (or the last statement instead of the `}`).

`shrink_selection`, when invoked from select mode, *forces* ignoring the undo behavior, and makes sure to place you on the first/last child.

---

The below three sections are changes for `select_textobject_around` (<kbd>ma</kbd>) and `select_textobject_inner` (<kbd>mi</kbd>):

### hotkeys

Commit semantics: is named `textobjects: hotkeys`.

|Key|Means|
|-|-|
|<kbd>j</kbd>/<kbd>`</kbd>|<kbd>`</kbd>¹|
|<kbd>k</kbd>/<kbd>_</kbd>|<kbd>_</kbd>¹|
|<kbd>i</kbd>/<kbd>*</kbd>|<kbd>*</kbd>¹|
|<kbd>,</kbd>/<kbd>&lt;</kbd>|<kbd>&lt;</kbd>¹|
|<kbd>.</kbd>/<kbd>&gt;</kbd>|<kbd>&gt;</kbd>¹|
|<kbd>q</kbd>|<kbd>‘</kbd>¹|
|<kbd>w</kbd>|<kbd>{</kbd>¹|
|<kbd>e</kbd>|<kbd>}</kbd>¹|
|<kbd>r</kbd>|<kbd>’</kbd>¹|
|<kbd>a</kbd>|<kbd>“</kbd>¹|
|<kbd>s</kbd>|<kbd>(</kbd>¹|
|<kbd>d</kbd>|<kbd>)</kbd>¹|
|<kbd>f</kbd>|<kbd>”</kbd>¹|
|<kbd>z</kbd>|<kbd>«</kbd>¹|
|<kbd>x</kbd>|<kbd>[</kbd>¹|
|<kbd>c</kbd>|<kbd>]</kbd>¹|
|<kbd>»</kbd>|<kbd>»</kbd>¹|
|<kbd>W</kbd>/<kbd>{</kbd>|word|
|<kbd>Q</kbd>/<kbd>‘</kbd>|big WORD|
|<kbd>C</kbd>/<kbd>]</kbd>|type|
|<kbd>F</kbd>/<kbd>”</kbd>|function|
|<kbd>A</kbd>/<kbd>“</kbd>|argument|
|<kbd>V</kbd>/<kbd>»</kbd>|comment|
|<kbd>R</kbd>/<kbd>’</kbd>|test|
|<kbd>E</kbd>/<kbd>}</kbd>|entry|
|<kbd>X</kbd>/<kbd>[</kbd>|html tag|
|<kbd>S</kbd>/<kbd>(</kbd>|paragraph|
|<kbd>D</kbd>/<kbd>)</kbd>|indentation|
|<kbd>m</kbd>|closest pair|
|<kbd>g</kbd>|(git) change|

¹ Also works in `surround_…` series of actions.

<details>
<summary>Reasoning</summary>

You heard me.
You can no longer use `mi(` to select inside of brackets; you *must* use `mis` instead.
As `mi(` will now select inside of paragraph.

The selfishness of this change is a great example of why I am explicit about the personal-ness of this fork, and why I try to help you find the commits holding particular changes, so that you could edit them or drop them.

I have a kanata layer that contains all brackets and “brackets”. When I hold <kbd>i</kbd>,
|1|2|3|4|
|-|-|-|-|
|<kbd>q</kbd>|<kbd>w</kbd>|<kbd>e</kbd>|<kbd>r</kbd>|
|<kbd>a</kbd>|<kbd>s</kbd>|<kbd>d</kbd>|<kbd>f</kbd>|
|<kbd>z</kbd>|<kbd>x</kbd>|<kbd>c</kbd>|<kbd>v</kbd>|

turn into

|1|2|3|4|
|-|-|-|-|
|<kbd>‘</kbd>|<kbd>{</kbd>|<kbd>}</kbd>|<kbd>’</kbd>|
|<kbd>“</kbd>|<kbd>(</kbd>|<kbd>)</kbd>|<kbd>”</kbd>|
|<kbd>«</kbd>|<kbd>[</kbd>|<kbd>]</kbd>|<kbd>»</kbd>|

So via this change I get to skip holding <kbd>i</kbd> to mean a bracket, being able to *just* press its key instead.
But then, I gotta move the actual (treesitter and such) textobjects somewhere else. \
The option of uppercase letters is entirely for you; in my case, I get much better sequentiality from something else.

<kbd>i↓a</kbd> and <kbd>i↓f</kbd> are my hotkeys to go to the next X textobject.
I hold down <kbd>i</kbd> and then press <kbd>a</kbd> or <kbd>f</kbd>; not having to *release* <kbd>i</kbd> and instead just press another key right afterwards is *incredible*. \
I can hold down <kbd>i</kbd> and then two taps of <kbd>f</kbd> (which ends up resolving into <kbd>”</kbd>) moves me to the next function.
And *that* is much nicer to spam repeatedly.

</details>

### whichkey

Commit semantics: is named `textobjects: debloat whichkey ui`.

<img width="250" height="463" alt="Image" src="https://github.com/user-attachments/assets/bc6961ea-f302-45d9-89dc-2bb6ef76ae78" />

<details>
<summary>Reasoning</summary>

Considering that it's now the only way you can express brackets, it would make sense for me to add their keys into the whichkey menu.
However ☝️ I don't want to look at them!

To begin with, *this* particular whichkey menu isn't filtered out by the `whichkey` option (that I have disabled) simply because I can never quite remember all of the options I have for textobjects.
Even after two years of using helix, I'm still not making full use of textobjects.

On the opposite hand, brackets I use all the time, and they're distinctly in my muscle memory.
Seeing them in a ui that's supposed to be giving me *useful* information is really jarring for me.

Check the commit of this change; you'll see that adding extra rows is really trivial.

</details>

### repeating

You can now **repeat** the textobject selection, for all text objects except: word, WORD, paragraph, (git) change, and literal characters without a unique closing pair (`"` or `a` for example).

> [!NOTE]
> `«` or `»` denotes the cursor position, `“` or `”` the anchor position (the other end of the selection)
> `█` means that the cursor position and anchor position are the same (1-width selection)

Let's first look at how the default / upstream helix works.
```rust
fn magic(text: &str) -> usize {
    let mage = |inner_text: &str| {
        inner_text.len()█
    };

    mage(text)
}
```
Here, you select around function (<kbd>maf</kbd>)
```rust
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
```rust
fn magic(text: &str) -> usize {
    let mage = |inner_text: &str| {
        inner_text.len()█
    };

    mage(text)
}
```
<kbd>maf</kbd>
```rust
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
```rust
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

When you use inside (<kbd>mi</kbd>) instead of around (<kbd>ma</kbd>), the “reach” is *two* characters to the left, instead of one (so that the expanding behavior works there too).

## Statusline

Commit semantics: `statusline:` prefix.

The statusline is replaced with command mode, when you invoke it. \
The messages *line* is removed altogether; use the `message` statusline element (probably in the middle of your statusline) to replace it.

The functionality of the messages line that shows you the keys you are pressing in chorded mappings, as well as the workspace distrust symbol, is removed, to make the above more viable.

The padding around the center segment of the statusline is changed from 1 → 0.

<details>
<summary>Reasoning</summary>

Most statusline elements already include compulsory surrounding spaces, so you can reasonably rely on other elements to provide this center segment padding for you.
The reason I'm removing it is since messages now write into the `message` element in the center (this is a strong expectation, your only other option is never seeing messages at all), I'm trying to minimize it getting truncated.

</details>

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
Total selections are filled to at least two.

The overall element ends in two spaces when there is only 1 selection or when there is less than 10 selections; otherwise ends in one space.

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

### code-action-hint

` ⋮ ` → ` 󰌵` (who the FUCK thought `⋮` is a good idea!! in what world does it not look like a bugged lsp spinner)

### current-working-directory

Folds `/home/username` to `~`

### message

Contains what the messages line used to have.

### spinner

Includes surrounding padding spaces, on both sides.

When not present, *nothing* is printed to its statusline element, instead of a space.

<details>
<summary>Reasoning</summary>

The source code makes a great argument that making it take up its space even when not present reduces elements jumping and shifting around.
*However*, by all accounts you'll have that particular element as the first on the right segment, or the last on the left segment.
Since it appears and disappears so often, while also being *gone* most of the time, it makes no sense to put it in between some other elements, that like to stay for longer.

So I don't find this anti-shifting useful for this element.
If only upstream cared about shifting for the elements where it *matters* ffs 😮‍💨

</details>

## Bufferline

Commit semantics: `bufferline:` prefix.

In default helix, there's one space on each side of each buffer name in the bufferline. \
So the first buffer's ending space and the second buffer's starting space ends up being 2 spaces together. \
This means that there's this awkward space before the entire bufferline, instead of being flush with the absolute left of the editor. \
This change places *two* spaces *after* each buffer, making the 2-sized gap still remain, but without the awkward leading space. \
`justify-content: space-between` rather than `space-around`, basically.

Also, the index of each buffer is shown before the filename.

Scratch buffers no longer have the `[+]` indicator next to them if they have unsaved changes.

<details>
<summary>Reasoning</summary>

Most of my scratch buffers exist *to* have unsaved changes, so this is visually annoying.

</details>

## Breadcrumbs

Commit semantics: `breadcrumbs:` prefix.

1 space of padding on the left is removed.

Separator ` > ` changed to `/`.

The `/` (root) component in the path is not printed, so you don't end up with `//` when editing files outside of `$HOME`.

`editor.breadcrumb.enable` forced to `false` for scratch buffers.

## Global formatter

Commit semantics: is named `custom_formatter`.

On write, the contents of the buffer are piped into `helix-piper`¹ and are replaced with its output².

<details>
<summary>Reasoning</summary>

You're meant to have `helix-piper` as a symlink to some formatter binary that you want to run for *everything*.

Unfortunately helix doesn't have a way to:
1. set a formatter for every language
2. set more than one formatter for a given language
3. set a formatter (or lsp) for the default, "text" language

So this is why I came up with this workaround.

If you're not sure how this can be useful to you, but are excited about the possibilities, check out [my blog post](https://axlefublr.github.io/consider-sorting/) on how I automatically sort *sections* of some files.

</details>

Beware: due to implementation details, std**err** is considered valid output. \
If your formatter prints an error to stderr, you'll end up seeing your entire buffer replaced with that error message. \
This can be destructive if you use `:write-all` while not looking at that buffer, and then close helix without realizing you demolished some buffer. Food for thought.

¹ If you have it in `$PATH`, it was `chmod +x`ed, your helix `editor.auto-format` config option is set to true, *and* you **don't** specify the `--no-format` flag for the command you choose to save with. \
Normal formatters also care about language-specific configuration — whether you enable or disable autoformatting for a given language.
This global formatter feature doesn't care about that.

² If that output is different from the input text. If the input and output are the same, the document doesn't get touched.

## Harp integration

Commit semantics: is named `action: harp`.

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

# Custom queries

Check out my custom treesitter queries in [qiri](https://github.com/Axlefublr/qiri)!

Not required for the fork to work; but if you like my changes, queries are yet another helix experience improvement that I do.

---
<!-- ━ -->

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
