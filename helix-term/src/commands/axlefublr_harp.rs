use std::path::PathBuf;

use axleharp::HarpReady;
use helix_view::{
    document::DEFAULT_LANGUAGE_NAME,
    editor::{Action, HarpRelativity},
};

use crate::{
    commands::{self, Context},
    compositor::{self},
    ui::PromptEvent,
};

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

fn harp_resolve(
    cx: &mut Context,
    prompt: String,
    relativity: HarpRelativity,
    section: String,
    action: impl Fn(&mut Context, String, String) + 'static,
) {
    cx.editor.set_status(format!("{} ({})", prompt, relativity));
    cx.on_next_key(move |cx, event| {
        let pressed = event.key_sequence_format();
        match &pressed[..] {
            "'" => harp_resolve(cx, prompt, HarpRelativity::Global, section, action),
            "," => harp_resolve(cx, prompt, HarpRelativity::Buffer, section, action),
            "." => harp_resolve(cx, prompt, HarpRelativity::Directory, section, action),
            ";" => harp_resolve(cx, prompt, HarpRelativity::Filetype, section, action),
            "<esc>" => (),
            other => {
                let finalized_section = match relativity {
                    HarpRelativity::Global => section,
                    HarpRelativity::Buffer => {
                        let (_, doc) = current!(cx.editor);
                        let Some(path) = &doc.path() else {
                            cx.editor
                                .set_error("harp: current buffer doesn't have a path");
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
                action(cx, finalized_section, other.into());
            }
        }
    })
}

pub fn harp_file_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp file get".into(),
        cx.editor.config().harp.file,
        "harp_files".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::path()) {
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
    harp_resolve(
        cx,
        "harp file set".into(),
        cx.editor.config().harp.file,
        "harp_files".into(),
        |cx, section, register| {
            let Some(current_buffer_path) = doc!(cx.editor).path().to_owned() else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };
            let current_buffer_path = current_buffer_path.clone();
            if let Err(msg) = harp_update(
                &section,
                &register,
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
    );
}

pub fn harp_relative_file_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp relative file get".into(),
        cx.editor.config().harp.relative_file,
        "harp_relative_files".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::path()) {
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
    );
}

pub fn harp_relative_file_set(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp relative file set".into(),
        cx.editor.config().harp.relative_file,
        "harp_relative_files".into(),
        |cx, section, register| {
            let Some(path) = doc!(cx.editor).path().to_owned() else {
                cx.editor
                    .set_error("harp: current buffer doesn't have a path");
                return;
            };
            let path = path.clone();
            let path = path
                .strip_prefix(helix_stdx::env::current_working_dir())
                .unwrap_or(&path);
            if let Err(msg) = harp_update(
                &section,
                &register,
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
    );
}

pub fn harp_cwd_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp cwd get".into(),
        cx.editor.config().harp.cwd,
        "harp_dirs".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::path()) {
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
    );
}

pub fn harp_cwd_set(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp cwd set".into(),
        cx.editor.config().harp.cwd,
        "harp_dirs".into(),
        |cx, section, register| {
            let path = helix_stdx::env::current_working_dir();
            if let Err(msg) = harp_update(
                &section,
                &register,
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
    );
}

pub fn harp_search_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp search get".into(),
        cx.editor.config().harp.search,
        "harp_searches".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::extra()) {
                Ok(values) => values,
                Err(msg) => {
                    cx.editor.set_error(msg);
                    return;
                }
            };
            match cx.editor.registers.push('/', values.extra.clone().unwrap()) {
                Ok(_) => cx
                    .editor
                    .set_status(format!("harp: set search to `{}`", values.extra.unwrap())),
                Err(err) => cx.editor.set_error(err.to_string()),
            }
        },
    );
}

pub fn harp_search_set(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp search set".into(),
        cx.editor.config().harp.search,
        "harp_searches".into(),
        |cx, section, register| {
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
                &section,
                &register,
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
    );
}

pub fn harp_register_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp register get".into(),
        cx.editor.config().harp.register,
        "harp_registers".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::extra()) {
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
    );
}

pub fn harp_register_set(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp register set".into(),
        cx.editor.config().harp.register,
        "harp_registers".into(),
        |cx, section, register| {
            let Some(values) = cx
                .editor
                .registers
                .read(cx.editor.config().default_yank_register, cx.editor)
            else {
                cx.editor.set_error("harp: default register is unset");
                return;
            };
            let register_contents = values.collect::<Vec<_>>().join("\n");
            if let Err(msg) = harp_update(
                &section,
                &register,
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
    );
}

pub fn harp_command_get(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp command get".into(),
        cx.editor.config().harp.command,
        "harp_commands".into(),
        |cx, section, register| {
            let values = match HarpOutput::build(&section, &register, HarpContract::extra()) {
                Ok(values) => values,
                Err(msg) => {
                    cx.editor.set_error(msg);
                    return;
                }
            };
            let value: String = values.extra.unwrap();
            match cx.editor.registers.last(':', cx.editor) {
                Some(last_reg) => {
                    // history is not uniqued, up arrowing quickly becomes hell
                    if last_reg != value {
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
            if let Err(err) = commands::execute_command_line(
                &mut compositor::Context {
                    editor: cx.editor,
                    jobs: cx.jobs,
                    scroll: None,
                },
                value.as_str(),
                PromptEvent::Validate,
            ) {
                cx.editor.set_error(err.to_string());
            }
        },
    );
}

pub fn harp_command_set(cx: &mut Context) {
    harp_resolve(
        cx,
        "harp command set".into(),
        cx.editor.config().harp.command,
        "harp_commands".into(),
        |cx, section, register| {
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
            if let Err(msg) = harp_update(
                &section,
                &register,
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
    );
}
