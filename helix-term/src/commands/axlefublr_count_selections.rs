use crate::commands::Context;

pub fn count_selections(cx: &mut Context) {
    let (view, doc) = current!(cx.editor);
    let selections_count = doc.selection(view.id).len();
    cx.editor
        .set_status(format!("selections: {}", selections_count));
}
