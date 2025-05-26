use crate::commands::Context;
use crate::ui::{self, PromptEvent};
use helix_core::{selection::Range, Selection, SmallVec, Tendril, Transaction};

pub fn surround_add_tag(cx: &mut Context) {
    ui::prompt(
        cx,
        "surround with tag:".into(),
        Some('<'),
        ui::completers::none,
        move |cx, input: &str, event: PromptEvent| {
            if event != PromptEvent::Validate {
                return;
            }
            if input.is_empty() {
                return;
            }
            let (view, doc) = current!(cx.editor);
            // surround_len is the number of new characters being added.
            let mut open = Tendril::new();
            open.push('<');
            open.push_str(input);
            open.push('>');
            let mut close = Tendril::new();
            close.push_str("</");
            close.push_str(input);
            close.push('>');
            let surround_len = input.len() * 2 + 5;

            let selection = doc.selection(view.id);
            let mut changes = Vec::with_capacity(selection.len() * 2);
            let mut ranges = SmallVec::with_capacity(selection.len());
            let mut offs = 0;

            for range in selection.iter() {
                changes.push((range.from(), range.from(), Some(open.clone())));
                changes.push((range.to(), range.to(), Some(close.clone())));

                ranges.push(
                    Range::new(offs + range.from(), offs + range.to() + surround_len)
                        .with_direction(range.direction()),
                );

                offs += surround_len;
            }

            let transaction = Transaction::change(doc.text(), changes.into_iter())
                .with_selection(Selection::new(ranges, selection.primary_index()));
            doc.apply(&transaction, view.id);
        },
    );
    super::exit_select_mode(cx);
}
