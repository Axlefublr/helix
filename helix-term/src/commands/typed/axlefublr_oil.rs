use std::iter;

use helix_core::{command_line::Args, Tendril, Transaction};
use helix_view::editor::Action;
use ignore::WalkBuilder;

use crate::{compositor, ui::PromptEvent};

pub fn oil(cx: &mut compositor::Context, _args: Args, event: PromptEvent) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc_id = cx.editor.new_file(Action::Replace);
    let doc = doc_mut!(cx.editor, &doc_id);
    let view_id = view!(cx.editor).id;
    let tendril = Tendril::from(collect_files().trim());
    let change = (0, 0, Some(tendril));
    let trans = Transaction::change(doc.text(), iter::once(change));
    doc.apply(&trans, view_id);
    // doc.text()
    Ok(())
}

fn collect_files() -> String {
    let dir = helix_stdx::env::current_working_dir();
    WalkBuilder::new(&dir)
        .hidden(false)
        .follow_links(false) // We're scanning over depth 1
        .git_ignore(true)
        .max_depth(None)
        .build()
        .filter_map(|file| {
            file.ok().and_then(|entry| {
                let path = entry.path();
                let path = path.strip_prefix(&dir).unwrap_or(path).to_path_buf();
                let path = path.into_os_string().into_string().ok()?;
                Some(path)
            })
        })
        .filter(|path| !path.is_empty())
        .fold(String::new(), |mut collector, path| {
            collector.push_str(&path);
            collector.push('\n');
            collector
        })
}
