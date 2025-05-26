use core::fmt;
use std::path::PathBuf;

use anyhow::{anyhow, bail, ensure};
use axleharp::HarpConnection;
use helix_core::{Position, Selection};
use helix_stdx::rope::Regex;
use helix_view::{
    document::{Mode, DEFAULT_LANGUAGE_NAME},
    editor::{Action, HarpHotkeys, HarpRelativity},
    info::Info,
};

use crate::{
    commands::{self, paste_impl, Context, Paste},
    compositor::{self},
    ui::PromptEvent,
};

const UNPATHED_BUFFER_ERROR: &str = "harp: current buffer doesn't have a path";

fn harp_replace(
    cx: &mut Context,
    mut connection: HarpConnection,
    section: String,
    register: String,
    mut values: Vec<String>,
) -> anyhow::Result<()> {
    let entry = connection.entry_mut(section, register);
    entry.clear();
    entry.append(&mut values);
    connection.save()?;
    cx.editor.set_status("harped!");
    Ok(())
}

fn harp_get_one(
    cx: &mut Context,
    connection: &HarpConnection,
    section: &str,
    register: &str,
    action: impl FnOnce(&mut Context, &String) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let Some(values) = connection.entry_ref(section, register) else {
        bail!("harp `{}` unset", register);
    };
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
    connection: &HarpConnection,
    expected: usize,
    section: &str,
    register: &str,
    action: impl FnOnce(&mut Context, &Vec<String>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let Some(values) = connection.entry_ref(section, register) else {
        bail!("harp `{}` unset", register);
    };
    ensure!(
        values.len() >= expected,
        "incorrect amount of values in harp register ({}) (dev's mistake, not yours)",
        values.len()
    );
    action(cx, values)
}

fn harp_view(
    connection: &HarpConnection,
    relativity: HarpRelativity,
    title_width: usize,
    section: &str,
    mut width_figurouter: impl FnMut(&Vec<String>) -> usize,
    mut transformer: impl FnMut(&Vec<String>, HarpRelativity, usize) -> Option<String>,
) -> Vec<(String, String)> {
    let Some(map) = connection.section_ref(section) else {
        return Vec::new();
    };
    let max_width = map
        .iter()
        .map(|(_, values)| width_figurouter(values))
        .max()
        .unwrap_or(0);
    let mut the = map
        .iter()
        .filter_map(|(key, values)| {
            transformer(values, relativity, max_width)
                .map(|value| (key.to_owned(), format!("{:<title_width$}", value)))
        })
        .collect::<Vec<_>>();
    the.sort();
    the
}

#[derive(Copy, Clone, PartialEq)]
enum Reciprocation {
    Get,
    Set,
    Delete,
}

impl fmt::Display for Reciprocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "get",
                Self::Set => "set",
                Self::Delete => "del",
            }
        )
    }
}

impl Reciprocation {
    fn toggle(self) -> Self {
        match self {
            Self::Get => Self::Set,
            Self::Set => Self::Get,
            Self::Delete => Self::Get,
        }
    }

    fn delete(self) -> Self {
        Self::Delete
    }
}

#[allow(clippy::too_many_arguments)]
fn harp_resolve(
    cx: &mut Context,
    prompt: String,
    relativity: HarpRelativity,
    hotkeys: HarpHotkeys,
    reciprocation: Reciprocation,
    section: String,
    mut connection: HarpConnection,
    mut width_figurouter: impl FnMut(&Vec<String>) -> usize + 'static,
    mut transformer: impl FnMut(&Vec<String>, HarpRelativity, usize) -> Option<String> + 'static,
    action: impl FnOnce(&mut Context, HarpConnection, Reciprocation, String, String) -> anyhow::Result<()>
        + 'static,
) {
    let final_section = match relativity {
        HarpRelativity::Global => section.clone(),
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
    let title = format!("{} {} ({})", prompt, reciprocation, relativity);
    let title_width = title.len().saturating_sub(4); // key column gives us 4 assured columns
    cx.editor.autoinfo = Some(Info::new(
        title,
        &harp_view(
            &connection,
            relativity,
            title_width,
            &final_section,
            &mut width_figurouter,
            &mut transformer,
        ),
    ));
    cx.on_next_key(move |cx, event| {
        cx.editor.autoinfo = None;
        let pressed = event.key_sequence_format();
        match &pressed[..] {
            the if the == hotkeys.global => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Global,
                hotkeys,
                reciprocation,
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            the if the == hotkeys.buffer => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Buffer,
                hotkeys,
                reciprocation,
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            the if the == hotkeys.directory => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Directory,
                hotkeys,
                reciprocation,
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            the if the == hotkeys.filetype => harp_resolve(
                cx,
                prompt,
                HarpRelativity::Filetype,
                hotkeys,
                reciprocation,
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            the if the == hotkeys.switch => harp_resolve(
                cx,
                prompt,
                relativity,
                hotkeys,
                reciprocation.toggle(),
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            the if the == hotkeys.delete_all => {
                connection.section_mut(final_section).clear();
                if let Err(err) = connection.clone().save() {
                    cx.editor.set_error(err.to_string());
                };
                harp_resolve(
                    cx,
                    prompt,
                    relativity,
                    hotkeys,
                    Reciprocation::Set,
                    section,
                    connection,
                    width_figurouter,
                    transformer,
                    action,
                )
            }
            "<esc>" => (),
            the if the == hotkeys.delete => harp_resolve(
                cx,
                prompt,
                relativity,
                hotkeys,
                reciprocation.delete(),
                section,
                connection,
                width_figurouter,
                transformer,
                action,
            ),
            other => {
                if reciprocation == Reciprocation::Delete {
                    connection.section_mut(final_section).remove(other);
                    if let Err(err) = connection.clone().save() {
                        cx.editor.set_error(err.to_string());
                    };
                    harp_resolve(
                        cx,
                        prompt,
                        relativity,
                        hotkeys,
                        reciprocation.toggle(),
                        section,
                        connection,
                        width_figurouter,
                        transformer,
                        action,
                    )
                } else if let Err(err) =
                    action(cx, connection, reciprocation, final_section, other.into())
                {
                    cx.editor.set_error(err.to_string());
                }
            }
        }
    })
}

fn happy_family(path: Option<&String>) -> Option<PathBuf> {
    let the = PathBuf::from(path?);
    let mut the = the.iter().rev();
    let file_name = the.next()?;
    let Some(parent_name) = the.next() else {
        return Some(PathBuf::from(file_name));
    };
    Some(PathBuf::from(parent_name).join(file_name))
}

fn register_text(text: Option<&String>, newline_char: char, is_search: bool) -> Option<String> {
    // let mut collector = String::with_capacity(100);
    // let mut first_time = true;
    // for iter in text?.lines().map(|the| the.trim().chars()) {
    //     if !first_time {
    //         collector.push(newline_char);
    //     } else {
    //         first_time = false;
    //     }
    //     for chr in iter {
    //         collector.push(chr);
    //         if collector.len() >= 100 {
    //             break;
    //         }
    //     }
    //     if collector.len() >= 100 {
    //         break;
    //     }
    // }
    // Some(collector)
    let mut lines_so_far = 0;
    let mut lines = text?.lines();
    let display_content = lines
        .by_ref()
        .filter_map(|the| {
            lines_so_far += 1;
            let the = the.trim();
            if the.is_empty() {
                None
            } else {
                Some(the)
            }
        })
        .take(1)
        .next()?;
    let display_content = if is_search {
        display_content
            .trim_start_matches("\\b")
            .trim_end_matches("\\b")
    } else {
        display_content
    };
    if lines_so_far < 2 {
        lines_so_far += lines.next().is_some() as i32;
    }
    Some(if lines_so_far > 1 {
        format!("{}{}", display_content, newline_char)
    } else {
        display_content.to_owned()
    })
}

// -------------------------actions-------------------------
pub fn harp_file(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    harp_resolve(
        cx,
        "file".into(),
        conf.harp.file,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_files".into(),
        harp_connection,
        |_| 0,
        |values, _, _| happy_family(values.first()).map(|the| the.display().to_string()),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
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
                harp_replace(cx, connection, section, register, vec![current_buffer_path])
            }
        },
    )
}

pub fn harp_relative_file(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    harp_resolve(
        cx,
        "relative".into(),
        conf.harp.relative_file,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_relative_files".into(),
        harp_connection,
        |_| 0,
        |values, _, _| happy_family(values.first()).map(|the| the.display().to_string()),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
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
                harp_replace(
                    cx,
                    connection,
                    section,
                    register,
                    vec![path.display().to_string()],
                )
            }
        },
    );
}

pub fn harp_cwd(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    harp_resolve(
        cx,
        "cwd".into(),
        conf.harp.cwd,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_dirs".into(),
        harp_connection,
        |_| 0,
        |values, _, _| happy_family(values.first()).map(|the| the.display().to_string()),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
                    helix_stdx::env::set_current_working_dir(value).unwrap();
                    cx.editor.set_status(format!("harp: cwd is now {}", value));
                    Ok(())
                })
            } else {
                let path = helix_stdx::env::current_working_dir();
                harp_replace(
                    cx,
                    connection,
                    section,
                    register,
                    vec![path.display().to_string()],
                )
            }
        },
    );
}

pub fn harp_search(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    let newline_char = conf.whitespace.characters.newline;
    harp_resolve(
        cx,
        "search".into(),
        conf.harp.search,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_searches".into(),
        harp_connection,
        |_| 0,
        move |values, _, _| register_text(values.first(), newline_char, true),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
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
                harp_replace(cx, connection, section, register, vec![search])
            }
        },
    );
}

pub fn harp_register(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    let newline_char = conf.whitespace.characters.newline;
    harp_resolve(
        cx,
        "register".into(),
        conf.harp.register,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_registers".into(),
        harp_connection,
        |_| 0,
        move |values, _, _| register_text(values.first(), newline_char, false),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
                    let values = vec![value.clone()];
                    if cx.editor.mode == Mode::Insert {
                        let (view, doc) = current!(cx.editor);
                        paste_impl(&values, doc, view, Paste::Cursor, 1, Mode::Insert);
                        if value.contains('█') {
                            let text = doc.text().slice(..);
                            if let Some(selection) = helix_core::selection::select_on_matches(
                                text,
                                doc.selection(view.id),
                                &Regex::new("█").unwrap(),
                            ) {
                                doc.set_selection(view.id, selection);
                            } else {
                                cx.editor.set_error("nothing selected");
                            }
                            crate::commands::change_selection_noyank(cx);
                        }
                    } else {
                        cx.editor
                            .registers
                            .write(cx.editor.config().default_yank_register, values)
                            .map_err(|_| anyhow!("harp: couldn't write to default register"))?;
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
                harp_replace(cx, connection, section, register, vec![contents])
            }
        },
    );
}

pub fn harp_command(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    harp_resolve(
        cx,
        "command".into(),
        conf.harp.command,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_commands".into(),
        harp_connection,
        |_| 0,
        |values, _, _| values.first().cloned(),
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get_one(cx, &connection, &section, &register, |cx, value| {
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
                harp_replace(cx, connection, section, register, vec![contents])
            }
        },
    );
}

pub fn harp_mark(cx: &mut Context) {
    let Ok(harp_connection) = HarpConnection::build() else {
        cx.editor
            .set_error("harp connection could not be established");
        return;
    };
    let conf = cx.editor.config();
    harp_resolve(
        cx,
        "mark".into(),
        conf.harp.mark,
        conf.harp.hotkeys.clone(),
        Reciprocation::Get,
        "harp_marks".into(),
        harp_connection,
        |values| {
            happy_family(values.first())
                .map(|the| the.display().to_string().len())
                .unwrap_or(0)
        }, // path column is the middle one, so needs to be aligned
        |values, relativity, widest_path| {
            let mut iter = values.iter();
            let path = happy_family(iter.next())?;
            let line = iter.next()?;
            let line_contents = iter.nth(1).map(|the| the.to_owned()).unwrap_or_default();
            // let column = iter.next()?;
            if relativity == HarpRelativity::Buffer {
                Some(format!("{:<4} {}", line, line_contents))
            } else {
                Some(format!(
                    "{:<4} {:<widest_path$}  {}",
                    line,
                    path.display(),
                    line_contents
                ))
            }
        },
        |cx, connection, reciprocation, section, register| {
            if let Reciprocation::Get = reciprocation {
                harp_get(cx, &connection, 3, &section, &register, |cx, values| {
                    let mut iter = values.iter();
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
                let line_contents = text
                    .line(line)
                    .to_string()
                    .trim()
                    .chars()
                    .take(50)
                    .collect();
                let path = doc
                    .path()
                    .ok_or_else(|| anyhow!(UNPATHED_BUFFER_ERROR))?
                    .display()
                    .to_string();
                harp_replace(
                    cx,
                    connection,
                    section,
                    register,
                    vec![path, line.to_string(), column.to_string(), line_contents],
                )
            }
        },
    );
}
