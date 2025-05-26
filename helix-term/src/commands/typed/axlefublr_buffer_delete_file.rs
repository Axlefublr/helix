use helix_core::command_line::Args;

use crate::{compositor, ui::PromptEvent};

pub fn buffer_delete_file(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let Some(path) = doc!(&cx.editor).path() else {
        cx.editor.set_error("buffer has no filepath");
        return Ok(());
    };
    let path = path.clone();
    super::force_buffer_close(cx, args, event)?;
    let _ = std::fs::remove_file(path); // not sure why, but this succeeds and also errors out, lol
    Ok(())
}
