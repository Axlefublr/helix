use helix_core::{Selection, SmallVec, Transaction};
use helix_stdx::rope::RopeSliceExt;
use helix_view::{Document, ViewId};

use crate::commands::shell_impl;

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
    // can be None somewhat arbitrarily; is a best effort basis
    if let Some(previous_contents) = text.as_str() {
        if output == previous_contents {
            return;
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
