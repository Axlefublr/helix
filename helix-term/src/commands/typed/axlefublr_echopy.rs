use helix_core::command_line::Args;

use crate::{compositor, ui::PromptEvent};

pub fn echopy(cx: &mut compositor::Context, args: Args, event: PromptEvent) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }

    let output = args.into_iter().fold(String::new(), |mut acc, arg| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(&arg);
        acc
    });

    match cx.editor.registers.write(
        cx.editor.config().default_yank_register,
        vec![output.clone()],
    ) {
        Ok(_) => cx.editor.set_status(output),
        Err(err) => cx.editor.set_error(err.to_string()),
    }

    Ok(())
}
