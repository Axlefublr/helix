use crate::commands::Context;

pub fn toggle_line_select(cx: &mut Context) {
    let (view, doc) = current_ref!(cx.editor);
    let old_selection = doc.selection(view.id).clone();
    super::extend_to_line_bounds(cx);
    let (view, doc) = current_ref!(cx.editor);
    let new_selection = doc.selection(view.id);
    if old_selection == *new_selection {
        super::trim_selections(cx);
    }
}
