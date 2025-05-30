use anyhow::{anyhow, ensure};
use helix_core::command_line::Args;

use crate::{compositor, ui::PromptEvent};

pub fn buffer_nth(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let n: usize = args
        .first()
        .ok_or(anyhow::anyhow!("no arguments provided"))?
        .parse()
        .map_err(|_| anyhow::anyhow!("provided argument is not an integer"))?;
    ensure!(n != 0);
    let (id, _) = if args.has_flag("reverse") {
        cx.editor.documents.iter().nth_back(n - 1)
    } else {
        cx.editor.documents.iter().nth(n - 1)
    }
    .ok_or(anyhow!("buffer {n} is out of range"))?;
    cx.editor.switch(*id, helix_view::editor::Action::Replace);
    Ok(())
}
