use crate::commands::{extend_to_line_bounds, trim_selections, Context};
use crate::{
    commands::typed,
    compositor,
    ui::{self, PromptEvent},
};
use axleharp::HarpReady;
use helix_core::{
    selection::Range, shellwords::Shellwords, Selection, SmallVec, Tendril, Transaction,
};
use helix_view::{document::DEFAULT_LANGUAGE_NAME, editor::Action, Document};
use std::{borrow::Cow, path::PathBuf};

#[derive(Default)]
pub struct HarpOutput {
    path: Option<PathBuf>,
    // line: Option<i32>,
    // column: Option<i32>,
    extra: Option<String>,
}
#[derive(Default)]
pub struct HarpInput {
    path: Option<String>,
    line: Option<i32>,
    column: Option<i32>,
    extra: Option<String>,
}
#[derive(Default)]
pub struct HarpContract {
    path: bool,
    line: bool,
    column: bool,
    extra: bool,
}
impl HarpContract {
    pub fn path() -> Self {
        HarpContract {
            path: true,
            ..Default::default()
        }
    }
    pub fn extra() -> Self {
        HarpContract {
            extra: true,
            ..Default::default()
        }
    }
}
impl HarpOutput {
    pub fn build(section: &str, register: &str, contract: HarpContract) -> Result<Self, String> {
        let harp = HarpReady::build().unwrap();
        let entry = harp
            .get(
                section,
                register,
                contract.path,
                contract.line,
                contract.column,
                contract.extra,
            )
            .map_err(|_| {
                format!(
                    "harp: register `{}` doesn't exist in section `{}`",
                    register, section,
                )
            })?;
        let path: Option<PathBuf> = if contract.path {
            Some(
                entry
                    .path
                    .clone()
                    .ok_or(format!(
                        "harp: path of register `{}` in section `{}` is empty",
                        register, section
                    ))?
                    .into(),
            )
        } else {
            None
        };
        let extra: Option<String> = if contract.extra {
            Some(entry.extra.clone().ok_or(format!(
                "harp: extra of register `{}` in section `{}` is empty",
                register, section
            ))?)
        } else {
            None
        };
        Ok(Self {
            path,
            // line: None,
            // column: None,
            extra,
        })
    }
}
pub fn harp_update(section: &str, register: &str, values: HarpInput) -> Result<(), String> {
    let mut harp = HarpReady::build().unwrap();
    if harp
        .update(
            section.into(),
            register.into(),
            values.path,
            values.line,
            values.column,
            values.extra,
        )
        .is_err()
    {
        Err(format!(
            "harp: couldn't update register {} in section {}",
            register, section
        ))
    } else {
        Ok(())
    }
}
fn eval_harp_relativity(
    cx: &mut compositor::Context,
    section_name: &str,
    register_input: &str,
) -> Option<(String, String)> {
    // relative to buffer
    if register_input == ","
        || register_input == "."
        || register_input == ";"
        || register_input == "'"
    {
        if let Err(msg) = harp_update(
            &format!("{}!!relativity", &section_name),
            "current",
            HarpInput {
                extra: Some(register_input.to_owned()),
                ..Default::default()
            },
        ) {
            cx.editor.set_error(msg);
        } else {
            cx.editor
                .set_status(format!("update section's relativity to {}", register_input));
        };
        None
    } else if let Some(register_input) = register_input.strip_prefix(',') {
        let (_, doc) = current!(cx.editor);
        let Some(path) = &doc.path else {
            cx.editor
                .set_error("harp: current buffer doesn't have a path");
            return None;
        };
        let section_name = format!("{}_{}", section_name, path.display());
        Some((section_name, register_input.to_owned()))
    // relative to project
    } else if let Some(register_input) = register_input.strip_prefix('.') {
        let section_name = format!(
            "{}_{}",
            section_name,
            helix_stdx::env::current_working_dir().display()
        );
        Some((section_name, register_input.to_owned()))
    // relative to filetype
    } else if let Some(register_input) = register_input.strip_prefix(';') {
        let (_, doc) = current!(cx.editor);
        let language_name = doc.language_name().unwrap_or(DEFAULT_LANGUAGE_NAME);
        let section_name = format!("{}!{}", section_name, language_name);
        Some((section_name, register_input.to_owned()))
    // not relative to anything (global)
    } else if let Some(register_input) = register_input.strip_prefix('\'') {
        Some((section_name.to_owned(), register_input.to_owned()))
    } else {
        let relativity = HarpOutput::build(
            &format!("{}!!relativity", &section_name),
            "current",
            HarpContract::extra(),
        )
        .unwrap_or_default()
        .extra
        .unwrap_or_else(|| String::from("'"));
        eval_harp_relativity(
            cx,
            section_name,
            &format!("{}{}", relativity, register_input),
        )
    }
}
pub fn harp_file_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp file get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_files", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::path()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            cx.editor
                .open(&values.path.unwrap(), Action::Replace)
                .unwrap();
        },
    )
}
pub fn harp_file_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp file set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let (_, doc) = current!(cx.editor);
            let Some(current_buffer_path) = doc.path.to_owned() else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_files", input)
            else {
                return;
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    path: Some(current_buffer_path.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}
pub fn harp_relative_file_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp relative file get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_relative_files", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::path()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            cx.editor
                .open(&values.path.unwrap(), Action::Replace)
                .unwrap();
        },
    )
}
pub fn harp_relative_file_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp relative file set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let (_, doc) = current!(cx.editor);
            let Some(path) = doc.path.to_owned() else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };
            let path = path
                .strip_prefix(helix_stdx::env::current_working_dir())
                .unwrap_or(&path);
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_relative_files", input)
            else {
                return;
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    path: Some(path.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}
pub fn harp_cwd_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp cwd get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) = eval_harp_relativity(cx, "harp_dirs", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::path()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            helix_stdx::env::set_current_working_dir(values.path.clone().unwrap()).unwrap();
            cx.editor.set_status(format!(
                "harp: cwd is now {}",
                values.path.unwrap().display()
            ));
        },
    )
}
pub fn harp_cwd_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp cwd set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let is_cwd_relative = input.starts_with('.');
            let Some((section_name, register_input)) = eval_harp_relativity(cx, "harp_dirs", input)
            else {
                return;
            };
            let path = if is_cwd_relative {
                let (_, doc) = current!(cx.editor);
                let Some(buffer_path) = &doc.path else {
                    cx.editor
                        .set_error("harp: current buffer doesn't have a path");
                    return;
                };
                let Some(buffer_head) = buffer_path.parent() else {
                    cx.editor
                        .set_error("harp: current buffer doesn't have a parent directory");
                    return;
                };
                buffer_head.into()
            } else {
                helix_stdx::env::current_working_dir()
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    path: Some(path.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}
pub fn harp_search_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp search get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_searches", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::extra()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            match cx
                .editor
                .registers
                .write('/', vec![values.extra.clone().unwrap()])
            {
                Ok(_) => cx
                    .editor
                    .set_status(format!("harp: set search to `{}`", values.extra.unwrap())),
                Err(err) => cx.editor.set_error(err.to_string()),
            }
        },
    )
}
pub fn harp_search_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp search set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let search: String = {
                let Some(mut search) = cx.editor.registers.read('/', cx.editor) else {
                    cx.editor.set_error("harp: register / is empty");
                    return;
                };
                match search.next() {
                    Some(search) => search.to_string(),
                    None => {
                        drop(search);
                        cx.editor.set_error("harp: register / is empty");
                        return;
                    }
                }
            };
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_searches", input)
            else {
                return;
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    extra: Some(search),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}
pub fn harp_register_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp register get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_registers", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::extra()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            match cx.editor.registers.write(
                cx.editor.config().default_yank_register,
                vec![values.extra.clone().unwrap()],
            ) {
                Ok(_) => cx
                    .editor
                    .set_status(format!("harp: get `{}`", values.extra.unwrap())),
                Err(_) => cx
                    .editor
                    .set_error("harp: couldn't write to default register"),
            }
        },
    )
}
pub fn harp_register_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp register set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some(values) = cx
                .editor
                .registers
                .read(cx.editor.config().default_yank_register, cx.editor)
            else {
                cx.editor.set_error("harp: default register is unset");
                return;
            };
            let register_contents = values.collect::<Vec<_>>().join("\n");
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_registers", input)
            else {
                return;
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    extra: Some(register_contents),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}
pub fn harp_command_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp command get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_commands", input)
            else {
                return;
            };
            let values =
                match HarpOutput::build(&section_name, &register_input, HarpContract::extra()) {
                    Ok(values) => values,
                    Err(msg) => {
                        cx.editor.set_error(msg);
                        return;
                    }
                };
            let value = values.extra.unwrap();
            let input: &str = value.as_ref();
            // it is more important to execute the command than to write it to history
            let _ = cx.editor.registers.write(':', vec![input.into()]);
            // everything beyond this point in the function is copied as-is from
            // helix-term/src/commands/typed.rs
            // definition of the command_mode function
            // if it's not 1:1, open an issue to help out :3
            let parts = input.split_whitespace().collect::<Vec<&str>>();
            if parts.is_empty() {
                return;
            }
            // If command is numeric, interpret as line number and go there.
            if parts.len() == 1 && parts[0].parse::<usize>().ok().is_some() {
                if let Err(e) = typed::goto_line_number(cx, &[Cow::from(parts[0])], event) {
                    cx.editor.set_error(format!("{}", e));
                }
                return;
            }
            // Handle typable commands
            if let Some(cmd) = typed::TYPABLE_COMMAND_MAP.get(parts[0]) {
                let shellwords = Shellwords::from(input);
                let args = shellwords.words();
                let (_, doc) = current!(cx.editor);
                let args: Vec<Cow<str>> = args
                    .iter()
                    .map(|word| Cow::Owned(expand_expansions(word, doc)))
                    .collect();
                if let Err(e) = (cmd.fun)(cx, &args[1..], event) {
                    cx.editor.set_error(format!("{}", e));
                }
            } else if event == PromptEvent::Validate {
                cx.editor
                    .set_error(format!("no such command: '{}'", parts[0]));
            }
        },
    )
}
pub fn harp_command_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp command set:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let Some(values) = cx
                .editor
                .registers
                .read(':', cx.editor)
                .and_then(|mut commands| commands.next())
            else {
                cx.editor.set_error("harp: command register is unset");
                return;
            };
            let register_contents = values.into();
            let Some((section_name, register_input)) =
                eval_harp_relativity(cx, "harp_commands", input)
            else {
                return;
            };
            if let Err(msg) = harp_update(
                &section_name,
                &register_input,
                HarpInput {
                    extra: Some(register_contents),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harped!");
            };
        },
    )
}

pub fn get_git_repo_root() -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("how the fuck do you not have git installed");
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        Some(stdout.trim().to_owned())
    } else {
        None
    }
}

pub fn expand_expansions(cmd: &str, doc: &Document) -> String {
    let maybe_path = doc.path.as_ref();
    let mut result = String::new();
    let mut chars = cmd.chars().peekable();

    while let Some(chr) = chars.next() {
        if chr != '%' {
            result.push(chr);
            continue;
        }

        let Some(&next_chr) = chars.peek() else {
            // % as last character in string
            result.push(chr);
            continue;
        };

        let mut maybe_expand = |next_chr: char, replacement: &str| {
            // if previous char was % too
            if result.ends_with('%') {
                result.push(next_chr);
            } else if next_chr.to_ascii_lowercase() == next_chr {
                result.push_str(replacement);
            } else {
                let replacement = {
                    let home = std::env::var("HOME").ok();
                    if let Some(home) = home {
                        replacement.replace(&home, "~")
                    } else {
                        replacement.into()
                    }
                };
                result.push_str(&replacement);
            }
            chars.next();
        };

        if next_chr.to_ascii_lowercase() == 'p' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| buf.display().to_string())
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'r' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| {
                        buf.strip_prefix(helix_stdx::env::current_working_dir())
                            .unwrap_or_else(|_| buf)
                            .display()
                            .to_string()
                    })
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'e' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| buf.extension().unwrap_or_default().to_string_lossy())
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'n' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| buf.file_name().unwrap_or_default().to_string_lossy())
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'g' {
            maybe_expand(next_chr, get_git_repo_root().unwrap_or_default().as_ref())
        } else if next_chr.to_ascii_lowercase() == 'q' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| {
                        buf.strip_prefix(get_git_repo_root().unwrap_or_default())
                            .unwrap_or_else(|_| buf)
                            .display()
                            .to_string()
                    })
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'l' {
            maybe_expand(
                next_chr,
                doc.language_name().unwrap_or(DEFAULT_LANGUAGE_NAME),
            )
        } else if next_chr.to_ascii_lowercase() == 'h' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| {
                        let mut buf = buf.clone();
                        buf.pop();
                        buf.display().to_string()
                    })
                    .unwrap_or_default(),
            )
        } else if next_chr.to_ascii_lowercase() == 'w' {
            maybe_expand(
                next_chr,
                &helix_stdx::env::current_working_dir().display().to_string(),
            )
        } else {
            result.push(chr);
        }
    }

    result
}

pub fn count_selections(cx: &mut Context) {
    let (view, doc) = current!(cx.editor);
    let selections_count = doc.selection(view.id).len();
    cx.editor
        .set_status(format!("selections: {}", selections_count));
}

pub fn toggle_line_select(cx: &mut Context) {
    let mut chars = {
        let (view, doc) = current!(cx.editor);
        let text = doc.text().slice(..);
        let chars = doc.selection(view.id).primary().slice(text).chars();
        chars
    };
    if chars.clone().all(char::is_whitespace) {
        extend_to_line_bounds(cx);
        return;
    }

    let first_char = chars.next();
    let last_char = chars.last();
    let is_trimmable = first_char.map(char::is_whitespace).unwrap_or(false)
        || last_char.map(char::is_whitespace).unwrap_or(false);
    if is_trimmable {
        trim_selections(cx);
    } else {
        extend_to_line_bounds(cx);
    }
}

pub fn surround_add_tag(cx: &mut Context) {
    ui::prompt(
        cx,
        "surround with tag:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let (view, doc) = current!(cx.editor);
            // surround_len is the number of new characters being added.
            let mut open = Tendril::new();
            open.push('<');
            open.push_str(input);
            open.push('>');
            let mut close = Tendril::new();
            close.push_str("</");
            close.push_str(input);
            close.push('>');
            let surround_len = input.len() * 2 + 5;

            let selection = doc.selection(view.id);
            let mut changes = Vec::with_capacity(selection.len() * 2);
            let mut ranges = SmallVec::with_capacity(selection.len());
            let mut offs = 0;

            for range in selection.iter() {
                changes.push((range.from(), range.from(), Some(open.clone())));
                changes.push((range.to(), range.to(), Some(close.clone())));

                ranges.push(
                    Range::new(offs + range.from(), offs + range.to() + surround_len)
                        .with_direction(range.direction()),
                );

                offs += surround_len;
            }

            let transaction = Transaction::change(doc.text(), changes.into_iter())
                .with_selection(Selection::new(ranges, selection.primary_index()));
            doc.apply(&transaction, view.id);
        },
    );
    super::exit_select_mode(cx);
}
