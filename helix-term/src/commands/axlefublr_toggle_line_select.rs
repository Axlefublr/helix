use crate::commands::Context;

pub fn toggle_line_select(cx: &mut Context) {
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);
    if doc.selection(view.id).iter().any(|selection| {
        selection
            .slice(text)
            .chars()
            .last()
            .map(|last_char| last_char == '\n')
            .unwrap_or(false)
    }) && !doc
        .selection(view.id)
        .iter()
        .all(|selection| selection.slice(text) == "\n")
    {
        super::trim_selections(cx);
    } else {
        super::extend_to_line_bounds(cx);
    }
}
