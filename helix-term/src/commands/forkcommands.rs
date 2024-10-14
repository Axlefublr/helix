use crate::{
    commands::ShellBehavior,
    ui::{self, PromptEvent},
};
use std::path::PathBuf;

use axleharp::HarpReady;
use helix_view::{editor::Action, Document};

use crate::commands::Context;

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
                    .try_into()
                    .map_err(|_| {
                        format!(
                            "harp: path of register `{}` in section `{}` is not a path",
                            register, section
                        )
                    })?,
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

            let values = match HarpOutput::build("harp_files", input, HarpContract::path()) {
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
            let Some(path) = &doc.path else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };

            if let Err(msg) = harp_update(
                "harp_files",
                input,
                HarpInput {
                    path: Some(path.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harp: set success");
            };
        },
    )
}

pub fn harp_project_file_get(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp project file get:".into(),
        None,
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }

            let values = match HarpOutput::build(
                &format!(
                    "harp_files_{}",
                    helix_stdx::env::current_working_dir().display()
                ),
                input,
                HarpContract::path(),
            ) {
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

pub fn harp_project_file_set(cx: &mut Context) {
    ui::prompt(
        cx,
        "harp project file set:".into(),
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
            let Some(path) = &doc.path else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };

            if let Err(msg) = harp_update(
                &format!(
                    "harp_files_{}",
                    helix_stdx::env::current_working_dir().display()
                ),
                input,
                HarpInput {
                    path: Some(path.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harp: set success");
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

            let values = match HarpOutput::build("harp_dirs", input, HarpContract::path()) {
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

            let cwd = helix_stdx::env::current_working_dir();

            if let Err(msg) = harp_update(
                "harp_dirs",
                input,
                HarpInput {
                    path: Some(cwd.display().to_string()),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harp: set success");
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

            let values = match HarpOutput::build("harp_searches", input, HarpContract::extra()) {
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

            if let Err(msg) = harp_update(
                "harp_searches",
                input,
                HarpInput {
                    extra: Some(search),
                    ..Default::default()
                },
            ) {
                cx.editor.set_error(msg);
            } else {
                cx.editor.set_status("harp: set success");
            };
        },
    )
}

pub fn shell_replace_with_output(cx: &mut Context) {
    super::shell_prompt(
        cx,
        "replace-with-output:".into(),
        ShellBehavior::JustReplace,
    );
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
            } else {
                result.push_str(replacement);
            }
            chars.next();
        };

        if next_chr == 'p' {
            maybe_expand(
                next_chr,
                &maybe_path
                    .map(|buf| buf.display().to_string())
                    .unwrap_or_default(),
            )
        } else if next_chr == 'h' {
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
        } else if next_chr == 'w' {
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
