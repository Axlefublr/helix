use core::fmt;
use std::path::PathBuf;

use anyhow::{anyhow, ensure};
use axleharp::HarpConnection;
use helix_core::{Position, Selection};
use helix_view::{
    document::DEFAULT_LANGUAGE_NAME,
    editor::{Action, HarpRelativity},
};

use crate::{
    commands::{self, Context},
    compositor::{self},
    ui::PromptEvent,
};

const UNPATHED_BUFFER_ERROR: &str = "harp: current buffer doesn't have a path";

fn harp_replace(
    cx: &mut Context,
    section: String,
    register: String,
    mut values: Vec<String>,
) -> anyhow::Result<()> {
    let mut harp = HarpConnection::build()?;
    let entry = harp.entry_mut(section, register);
    entry.clear();
    entry.append(&mut values);
    harp.save()?;
    cx.editor.set_status("harped!");
    Ok(())
}

fn harp_get_one(
    cx: &mut Context,
    section: String,
    register: String,
    action: impl FnOnce(&mut Context, &String) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let mut harp = HarpConnection::build()?;
    let values = harp.entry_mut(section, register.clone());
    ensure!(!values.is_empty(), "harp `{}` unset", register);
    ensure!(
        values.len() == 1,
        "incorrect amount of values in harp register ({}) (dev's mistake, not yours)",
        values.len()
    );
    let value = values.first().unwrap();
    action(cx, value)
}

fn harp_get(
    cx: &mut Context,
    expected: usize,
    section: String,
    register: String,
    action: impl FnOnce(&mut Context, &mut Vec<String>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let mut harp = HarpConnection::build()?;
    let values = harp.entry_mut(section, register.clone());
    ensure!(!values.is_empty(), "harp `{}` unset", register);
    ensure!(
        values.len() == expected,
        "incorrect amount of values in harp register ({}) (dev's mistake, not yours)",
        values.len()
    );
    action(cx, values)
}

#[derive(Copy, Clone)]
enum Reciprocation {
    Get,
    Set,
}

impl fmt::Display for Reciprocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "get",
                Self::Set => "set",
            }
        )
    }
}

impl Reciprocation {
    fn toggle(self) -> Self {
        match self {
            Self::Get => Self::Set,
            Self::Set => Self::Get,
        }
    }
}

fn harp_resolve(
    cx: &mut Context,
    prompt: String,
    relativity: HarpRelativity,
    reciprocation: Reciprocation,
    section: String,
    action: impl FnOnce(&mut Context, Reciprocation, String, String) -> anyhow::Result<()> + 'static,
) {
    cx.editor
        .set_status(format!("{} {} ({})", prompt, reciprocation, relativity));
    cx.on_next_key(move |cx, event| {
        let pressed = event.key_sequence_format();
        match &pressed[..] {
            "j" => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Global,
                reciprocation,
                section,
                action,
            ),
            "l" => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Buffer,
                reciprocation,
                section,
                action,
            ),
            "k" => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Directory,
                reciprocation,
                section,
                action,
            ),
            ";" => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Filetype,
                reciprocation,
                section,
                action,
            ),
            "<space>" | "'" => harp_resolve(
                cx,
                prompt,
                relativity,
                reciprocation.toggle(),
                section,
                action,
            ),
            "<esc>" => (),
            other => {
                let finalized_section = match relativity {
                    HarpRelativity::Global => section,
                    HarpRelativity::Buffer => {
                        let (_, doc) = current!(cx.editor);
                        let Some(path) = &doc.path() else {
                            cx.editor.set_error(UNPATHED_BUFFER_ERROR);
                            return;
                        };
                        format!("{}_{}", section, path.display())
                    }
                    HarpRelativity::Directory => format!(
                        "{}_{}",
                        section,
                        helix_stdx::env::current_working_dir().display()
                    ),
                    HarpRelativity::Filetype => {
                        let (_, doc) = current!(cx.editor);
                        let language_name = doc.language_name().unwrap_or(DEFAULT_LANGUAGE_NAME);
                        format!("{}!{}", section, language_name)
                    }
                };
                if let Err(err) = action(cx, reciprocation, finalized_section, other.into()) {
                    cx.editor.set_error(err.to_string());
                };
            }
        }
    })
}

pub fn harp_file(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp file".into(),
        cx.editor.config().harp.file,
        Reciprocation::Get,
        "harp_files".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    cx.editor
                        .open(PathBuf::from(value).as_ref(), Action::Replace)
                        .unwrap();
                    Ok(())
                })
            } else {
                let current_buffer_path = doc!(cx.editor)
                    .path()
                    .ok_or_else(|| anyhow!(UNPATHED_BUFFER_ERROR))?
                    .display()
                    .to_string();
                harp_replace(cx, section, register, vec![current_buffer_path])
            }
        },
    )
}

pub fn harp_relative_file(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp relative file".into(),
        cx.editor.config().harp.relative_file,
        Reciprocation::Get,
        "harp_relative_files".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    cx.editor
                        .open(PathBuf::from(value).as_ref(), Action::Replace)
                        .unwrap();
                    Ok(())
                })
            } else {
                let path = doc!(cx.editor)
                    .path()
                    .to_owned()
                    .ok_or_else(|| anyhow!("harp: current buffer doesn't have a path"))?;
                let path = path.clone();
                let path = path
                    .strip_prefix(helix_stdx::env::current_working_dir())
                    .unwrap_or(&path);
                harp_replace(cx, section, register, vec![path.display().to_string()])
            }
        },
    );
}

pub fn harp_cwd(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp cwd".into(),
        cx.editor.config().harp.cwd,
        Reciprocation::Get,
        "harp_dirs".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    helix_stdx::env::set_current_working_dir(value).unwrap();
                    cx.editor.set_status(format!("harp: cwd is now {}", value));
                    Ok(())
                })
            } else {
                let path = helix_stdx::env::current_working_dir();
                harp_replace(cx, section, register, vec![path.display().to_string()])
            }
        },
    );
}

pub fn harp_search(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp search".into(),
        cx.editor.config().harp.search,
        Reciprocation::Get,
        "harp_searches".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    cx.editor.registers.push('/', value.to_owned())?;
                    crate::commands::search_next(cx);
                    Ok(())
                })
            } else {
                let search: String = cx
                    .editor
                    .registers
                    .read('/', cx.editor)
                    .and_then(|mut search| search.next())
                    .ok_or_else(|| anyhow!("harp: register / is empty"))?
                    .to_string();
                harp_replace(cx, section, register, vec![search])
            }
        },
    );
}

pub fn harp_register(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp register".into(),
        cx.editor.config().harp.register,
        Reciprocation::Get,
        "harp_registers".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    cx.editor
                        .registers
                        .write(
                            cx.editor.config().default_yank_register,
                            vec![value.clone()],
                        )
                        .map_err(|_| anyhow!("harp: couldn't write to default register"))?;
                    if cx.editor.mode == helix_view::document::Mode::Insert {
                        crate::commands::collapse_selection(cx);
                        crate::commands::paste_before(cx);
                    } else {
                        cx.editor.set_status(format!("harp: get `{}`", value));
                    }
                    Ok(())
                })
            } else {
                let contents = cx
                    .editor
                    .registers
                    .read(cx.editor.config().default_yank_register, cx.editor)
                    .ok_or_else(|| anyhow!("harp: default register is unset"))?
                    .collect::<Vec<_>>()
                    .join("\n");
                harp_replace(cx, section, register, vec![contents])
            }
        },
    );
}

pub fn harp_command(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp command".into(),
        cx.editor.config().harp.command,
        Reciprocation::Get,
        "harp_commands".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, section, register, |cx, value| {
                    match cx.editor.registers.last(':', cx.editor) {
                        Some(last_reg) => {
                            // history is not uniqued, up arrowing quickly becomes hell
                            if last_reg != value.as_str() {
                                let _ = cx.editor.registers.push(':', value.clone());
                            }
                        }
                        None => {
                            // it is more important to execute the command than to write it to history
                            let _ = cx.editor.registers.push(':', value.clone());
                        }
                    }
                    // stolen from
                    // helix-term/src/commands/typed.rs
                    commands::execute_command_line(
                        &mut compositor::Context {
                            editor: cx.editor,
                            jobs: cx.jobs,
                            scroll: None,
                        },
                        value.as_str(),
                        PromptEvent::Validate,
                    )?;
                    Ok(())
                })
            } else {
                let contents = cx
                    .editor
                    .registers
                    .read(':', cx.editor)
                    .and_then(|mut commands| commands.next())
                    .ok_or_else(|| anyhow!("harp: command register is unset"))?
                    .to_string();
                harp_replace(cx, section, register, vec![contents])
            }
        },
    );
}

pub fn harp_mark(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp mark".into(),
        cx.editor.config().harp.mark,
        Reciprocation::Get,
        "harp_marks".into(),
        |cx, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get(cx, 3, section, register, |cx, values| {
                    let mut iter = values.iter_mut();
                    // these FIVE unwraps are here because I'm banking on the set action to set everything validly
                    // if the user fucks up their harp.jsonc, it's entirely their fault
                    let path = iter.next().unwrap();
                    cx.editor
                        .open(PathBuf::from(&path).as_ref(), Action::Replace)
                        .unwrap();
                    let (view, doc) = current!(cx.editor);
                    let text = doc.text().slice(..);
                    let line = iter.next().unwrap().parse().unwrap();
                    let column = iter.next().unwrap().parse().unwrap();
                    let position =
                        helix_core::pos_at_coords(text, Position::new(line, column), true);
                    doc.set_selection(view.id, Selection::point(position));
                    Ok(())
                })
            } else {
                let (view, doc) = current!(cx.editor);
                let text = doc.text().slice(..);
                let range = doc.selection(view.id).primary();
                let position = range.cursor(text);
                let Position {
                    row: line,
                    col: column,
                } = helix_core::coords_at_pos(text, position);
                let path = doc
                    .path()
                    .ok_or_else(|| anyhow!(UNPATHED_BUFFER_ERROR))?
                    .display()
                    .to_string();
                harp_replace(
                    cx,
                    section,
                    register,
                    vec![path, line.to_string(), column.to_string()],
                )
            }
        },
    );
}
