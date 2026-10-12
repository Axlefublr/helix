use crate::{movement::Direction, syntax::TreeCursor, Range, RopeSlice, Selection, Syntax};

pub fn expand_selection(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    let cursor = &mut syntax.walk();

    selection.transform(|range| {
        let from = text.char_to_byte(range.from()) as u32;
        let to = text.char_to_byte(range.to()) as u32;

        let byte_range = from..to;
        cursor.reset_to_byte_range(from, to);

        while cursor.node().byte_range() == byte_range {
            if !cursor.goto_parent() {
                break;
            }
        }

        let node = cursor.node();
        let from = text.byte_to_char(node.start_byte() as usize);
        let to = text.byte_to_char(node.end_byte() as usize);

        Range::new(to, from).with_direction(range.direction())
    })
}

pub fn shrink_selection(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    select_node_impl(
        syntax,
        text,
        selection,
        |cursor, _| {
            cursor.goto_first_child();
        },
        None,
    )
}

pub fn select_next_sibling(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    use std::ops::Not;
    select_node_impl(
        syntax,
        text,
        selection,
        |cursor, needed_adjust| {
            if needed_adjust && {
                let node = cursor.node();
                node.is_named() && node.is_extra().not()
            } {
                return;
            }
            if let Some(node) = any_valid_future_siblings(cursor.node(), Direction::Forward) {
                cursor.reset_to_byte_range(node.start_byte(), node.end_byte());
            };
        },
        Some(Direction::Forward),
    )
}

pub fn select_all_siblings(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    let mut cursor = syntax.walk();
    selection.transform_iter(move |range| {
        let (from, to) = range.into_byte_range(text);
        cursor.reset_to_byte_range(from as u32, to as u32);

        if !cursor.goto_parent_with(|parent| parent.child_count() > 1) {
            return vec![range].into_iter();
        }

        select_children(&mut cursor, text, range).into_iter()
    })
}

pub fn select_all_children(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    let mut cursor = syntax.walk();
    selection.transform_iter(move |range| {
        let (from, to) = range.into_byte_range(text);
        cursor.reset_to_byte_range(from as u32, to as u32);
        select_children(&mut cursor, text, range).into_iter()
    })
}

fn select_children(cursor: &mut TreeCursor, text: RopeSlice, range: Range) -> Vec<Range> {
    let children = cursor
        .children()
        .filter(|child| child.is_named())
        .map(|child| Range::from_node(child, text, range.direction()))
        .collect::<Vec<_>>();

    if !children.is_empty() {
        children
    } else {
        vec![range]
    }
}

pub fn select_prev_sibling(syntax: &Syntax, text: RopeSlice, selection: Selection) -> Selection {
    use std::ops::Not;
    select_node_impl(
        syntax,
        text,
        selection,
        |cursor, needed_adjust| {
            if needed_adjust && {
                let node = cursor.node();
                node.is_named() && node.is_extra().not()
            } {
                return;
            }
            if let Some(node) = any_valid_future_siblings(cursor.node(), Direction::Backward) {
                cursor.reset_to_byte_range(node.start_byte(), node.end_byte());
            };
        },
        Some(Direction::Backward),
    )
}

fn adjust_malformed_treesitter_range(
    text: &RopeSlice,
    direction: Direction,
    range: Range,
) -> Option<Range> {
    use helix_stdx::rope::RopeSliceExt;

    let cursor_from = range.cursor(*text);
    let cursor_to = crate::graphemes::next_grapheme_boundary(*text, cursor_from);

    let cursor_on_whitespace = text
        .slice(cursor_from..cursor_to)
        .bytes()
        .all(|byte| byte.is_ascii_whitespace());
    if !cursor_on_whitespace {
        return Some(Range::point(cursor_from));
    }

    #[allow(clippy::collapsible_else_if)]
    if direction == Direction::Forward {
        text.slice(cursor_to..)
            .first_non_whitespace_char()
            .map(|until_land| Range::point(cursor_to + until_land))
    } else {
        text.slice(..cursor_from)
            .last_non_whitespace_char()
            .map(Range::point)
    }
}

fn any_valid_future_siblings(
    node: tree_house::tree_sitter::Node,
    direction: Direction,
) -> Option<tree_house::tree_sitter::Node> {
    use std::ops::Not;
    let mut current = node;
    loop {
        let sibling = match direction {
            Direction::Forward => current.next_sibling(),
            Direction::Backward => current.prev_sibling(),
        };
        match sibling {
            Some(sibling)
                if sibling.is_named() && sibling.is_extra().not() && sibling.is_missing().not() =>
            {
                return Some(sibling)
            }
            Some(sibling) => current = sibling,
            None => {
                let parent = current.parent()?;
                if current.start_byte() != parent.start_byte()
                    || current.end_byte() != parent.end_byte()
                {
                    return None;
                }
                current = parent
            }
        }
    }
}

fn select_node_impl<F>(
    syntax: &Syntax,
    text: RopeSlice,
    selection: Selection,
    motion: F,
    direction: Option<Direction>,
) -> Selection
where
    F: Fn(&mut TreeCursor, bool),
{
    use std::ops::Not;
    let cursor = &mut syntax.walk();

    selection.transform(|range| {
        let from = text.char_to_byte(range.from());
        let to = text.char_to_byte(range.to());

        let is_valid_node = {
            cursor.reset_to_byte_range(from as u32, to as u32);
            let node = cursor.node();
            let node_from = node.start_byte() as usize;
            let node_to = node.end_byte() as usize;
            node_from == from && node_to == to
        };

        let adjusted_range = is_valid_node
            .not()
            .then(|| {
                direction.and_then(|direction| {
                    adjust_malformed_treesitter_range(&text, direction, range)
                })
            })
            .flatten();
        let needed_adjust = adjusted_range.is_some();
        let range = adjusted_range.unwrap_or(range);

        let from = text.char_to_byte(range.from());
        let to = text.char_to_byte(range.to());

        cursor.reset_to_byte_range(from as u32, to as u32);

        motion(cursor, needed_adjust);

        let node = cursor.node();
        let from = text.byte_to_char(node.start_byte() as usize);
        let to = text.byte_to_char(node.end_byte() as usize);

        Range::new(from, to).with_direction(direction.unwrap_or_else(|| range.direction()))
    })
}
