use std::borrow::Cow;

use helix_core::Transaction;

use crate::{
    commands::typed::{
        buffer_close, force_buffer_close, force_quit, force_write_buffer_close, quit,
        write_buffer_close,
    },
    compositor,
    ui::PromptEvent,
};

pub fn random(
    cx: &mut compositor::Context,
    _: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
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

pub fn echo(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }

    cx.editor.set_status(args.join(" "));
    Ok(())
}

pub fn echopy(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }

    let expansion = args.join(" ");
    match cx.editor.registers.write('+', vec![expansion.clone()]) {
        Ok(_) => cx.editor.set_status(expansion),
        Err(err) => cx.editor.set_error(err.to_string()),
    }

    Ok(())
}

pub fn write_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path.is_none() && cx.editor.documents().count() <= 1 {
        quit(cx, args, event)
    } else {
        write_buffer_close(cx, args, event)
    }
}

pub fn force_write_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path.is_none() && cx.editor.documents().count() <= 1 {
        force_quit(cx, args, event)
    } else {
        force_write_buffer_close(cx, args, event)
    }
}

pub fn buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path.is_none() && cx.editor.documents().count() <= 1 {
        quit(cx, args, event)
    } else {
        buffer_close(cx, args, event)
    }
}

pub fn force_buffer_close_or_quit(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }
    let doc = doc!(cx.editor);
    if !doc.is_modified() && doc.path.is_none() && cx.editor.documents().count() <= 1 {
        force_quit(cx, args, event)
    } else {
        force_buffer_close(cx, args, event)
    }
}

pub fn buffer_delete_file(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    let Some(path) = doc!(&cx.editor).path.clone() else {
        cx.editor.set_error("buffer has no filepath");
        return Ok(());
    };
    force_buffer_close(cx, args, event)?;
    let _ = std::fs::remove_file(path); // not sure why, but this succeeds and also errors out, lol
    Ok(())
}
