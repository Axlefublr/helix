use helix_core::{command_line::Args, Selection, SmallVec, Tendril, Transaction};
use helix_stdx::rope::RopeSliceExt;
use helix_view::{Document, ViewId};

use crate::{
    commands::{
        shell_impl,
        typed::{
            buffer_close, force_buffer_close, force_quit, force_write_buffer_close, quit,
            write_buffer_close,
        },
    },
    compositor,
    ui::PromptEvent,
};

pub fn random(cx: &mut compositor::Context, _args: Args, event: PromptEvent) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }

    let scrolloff = cx.editor.config().scrolloff;
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);

    let selection = doc.selection(view.id);

    let mut fragments: Vec<_> = selection
        .slices(text)
        .map(|fragment| fragment.chunks().collect())
        .collect();

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    fragments.shuffle(&mut rng);

    let transaction = Transaction::change(
        doc.text(),
        selection
            .into_iter()
            .zip(fragments)
            .map(|(s, fragment)| (s.from(), s.to(), Some(fragment))),
    );

    doc.apply(&transaction, view.id);
    doc.append_changes_to_history(view);
    view.ensure_cursor_in_view(doc, scrolloff);

    Ok(())
}

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

pub fn write_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path().is_none() && cx.editor.documents().count() <= 1 {
        quit(cx, args, event)
    } else {
        write_buffer_close(cx, args, event)
    }
}

pub fn force_write_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path().is_none() && cx.editor.documents().count() <= 1 {
        force_quit(cx, args, event)
    } else {
        force_write_buffer_close(cx, args, event)
    }
}

pub fn buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path().is_none() && cx.editor.documents().count() <= 1 {
        quit(cx, args, event)
    } else {
        buffer_close(cx, args, event)
    }
}

pub fn force_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path().is_none() && cx.editor.documents().count() <= 1 {
        force_quit(cx, args, event)
    } else {
        force_buffer_close(cx, args, event)
    }
}

pub fn buffer_delete_file(
    cx: &mut compositor::Context,
    args: Args,
    event: PromptEvent,
) -> anyhow::Result<()> {
    let Some(path) = doc!(&cx.editor).path() else {
        cx.editor.set_error("buffer has no filepath");
        return Ok(());
    };
    let path = path.clone();
    force_buffer_close(cx, args, event)?;
    let _ = std::fs::remove_file(path); // not sure why, but this succeeds and also errors out, lol
    Ok(())
}

pub fn custom_formatter(doc: &mut Document, view_id: ViewId) {
    if !helix_stdx::env::binary_exists("helix-piper") {
        return;
    }
    let selection = doc.selection(view_id);
    let text = doc.text().slice(..);

    let output = {
        // in reality we super don't need a shell, but I can't be bothered to figure out how to ~~idiomatically~~ helixomatically call a Command and pipe the text into it; plus if I write something up like that myself, I feel that it will be less maintainable, not more
        // the problem with this, is that stderr is also considered valid output.
        match shell_impl(
            &["sh".into(), "-c".into()],
            "helix-piper",
            Some(text.into()),
        ) {
            Ok(mut output) => {
                if !text.ends_with("\n") && output.ends_with('\n') {
                    output.pop();
                    if output.ends_with('\r') {
                        output.pop();
                    }
                }
                output
            }
            Err(_) => {
                return;
            }
        }
    };

    let from = 0usize;
    let to = text.len_chars();

    let changes = Vec::from(&[(from, to, Some(output))]);
    let transaction =
        Transaction::change(doc.text(), changes.into_iter()).with_selection(Selection::new(
            SmallVec::from(selection.ranges()),
            selection.primary_index(),
        ));
    doc.apply(&transaction, view_id);
}
