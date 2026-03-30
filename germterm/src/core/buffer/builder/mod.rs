pub mod row;

use crate::{
    cell::Cell,
    core::{
        buffer::{builder::row::BuilderRow, Buffer},
        draw::{Position, Size},
    },
};

#[doc(hidden)]
pub struct BuilderBuffer {
    pub rows: &'static [BuilderBufferItem],
    pub size: Size,
}

#[allow(unused)]
impl BuilderBuffer {
    const fn new(items: &'static [BuilderBufferItem]) -> Self {
        Self {
            rows: items,
            size: Self::size(items),
        }
    }

    const fn size(items: &'static [BuilderBufferItem]) -> Size {
        let width = Self::width(items);
        let height = Self::height(items);

        Size::new(width, height)
    }

    const fn height(items: &'static [BuilderBufferItem]) -> u16 {
        let mut i = 0;
        let mut height = 0;
        while i < items.len() {
            match items[i] {
                BuilderBufferItem::Empty(n) => {
                    height += n;
                }
                BuilderBufferItem::Row(_) => {
                    height += 1;
                }
            }
            i += 1;
        }

        assert!(
            (u16::MAX as usize) >= height as usize,
            "Unable to determine height of buffer"
        );

        height
    }

    const fn width(items: &'static [BuilderBufferItem]) -> u16 {
        let mut i = 0;
        let mut width = None;
        while i < items.len() {
            if let Some(c) = items[i].consumed() {
                if let Some(width) = width {
                    assert!(width == c, "Rows of a build buffer must be the same length");
                }

                width = Some(c);
            }

            i += 1;
        }

        if let Some(w) = width {
            w
        } else {
            panic!("Unable to determine width of buffer");
        }
    }
}

/// An item in a builder buffer, representing either a row of cells or skipped lines.
#[doc(hidden)]
pub enum BuilderBufferItem {
    /// Skip `n` rows (vertical space).
    Empty(u16),
    /// A row of cells.
    Row(BuilderRow),
}

impl BuilderBufferItem {
    pub const fn consumed(&self) -> Option<u16> {
        match self {
            BuilderBufferItem::Empty(_) => None,
            BuilderBufferItem::Row(builder_row) => Some(builder_row.consumed),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! builder_buffer_item {
    (empty($n:literal)) => {
        BBI::Empty($n)
    };
    ($($row_stuff:tt)*) => {{
        BBI::Row($crate::builder_row!($($row_stuff)*))
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! builder_buffer_internal  {
    (@munched[$($munched:tt)*] [$($row_content:tt)+] $(, $( $rest:tt)*)?) => {{
        $crate::builder_buffer_internal!{@munched[$($munched)* $crate::builder_buffer_item!{$($row_content)+}, ] $($($rest)*)?}
    }};
    (@munched[$($munched:tt)*] $buffer_item_ident:ident$(($($buffer_item_args:tt)*))? $(, $($rest:tt)*)?) => {{
        $crate::builder_buffer_internal!{@munched[$($munched)* $crate::builder_buffer_item!{$buffer_item_ident$(($($buffer_item_args)*))?}, ] $($($rest)*)?}
    }};
    (@munched[$($finished:tt)*]) => {{
        const {
            BB::new(&[$($finished)*])
        }
    }};
}

/// Creates a compile-time buffer layout from rows of cells.
///
/// # Syntax
///
/// Rows are comma-separated, with each row containing items:
/// - `"text"` or `cell("text")` - cell with default style
/// - `cell("text", style)` - cell with custom style
/// - `empty(n)` - skip n columns
///
/// # Examples
///
/// ```
/// const BUF: BuilderBuffer = builder_buffer!(
///     ["Header", empty(5), "Value"],
///     empty(1),
///     [cell("A", Style::EMPTY), "B", "C"],
/// );
/// ```
#[macro_export]
macro_rules! builder_buffer{
    ($($tokens:tt)+) => {{
        #[allow(unused)]
        use $crate::{
            core::buffer::builder::{
                BuilderBuffer as BB, BuilderBufferItem as BBI,
                row::{
                    BuilderRowItem as BRI,
                    BuilderRowItem::{Cell as cell, Empty as empty},
                },
            },
            style::Style as ST,
        };

        const BUILT_BUFFER: BB = const { $crate::builder_buffer_internal!{@munched[] $($tokens)+} };

        BUILT_BUFFER
    }};
}

/// Renders a [`BuilderBuffer`] into a [`Buffer`].
///
/// # Panics
///
/// Panics if the buffer layout exceeds the target buffer bounds.
///
/// # Examples
///
/// ```
/// let mut fb = FlatBuffer::new(layout.size);
/// build(&layout, &mut fb);
/// ```
#[doc(hidden)]
pub fn build(bb: &BuilderBuffer, buf: &mut dyn Buffer) {
    let mut cursor = Position::ZERO;
    let sz = buf.size();
    bb.rows.iter().for_each(|row| match row {
        BuilderBufferItem::Empty(n) => {
            // Both these are checked in [`BuilderBuffer::new`] these are extra assertions just in case.
            cursor.y = cursor
                .y
                .checked_add(*n)
                .expect("Cursor height should never be greater than u16::MAX");
            assert!(
                cursor.y < sz.height,
                "Cursor skipped rows should never exceed area height"
            );
        }
        BuilderBufferItem::Row(r) => {
            let items = (r.items)();
            items.into_iter().for_each(|item| {
                match item {
                    row::BuilderRowItem::Cell(s, style) => {
                        buf.set_cell(cursor, &Cell::new(s, style));
                        cursor.x = cursor
                            .x
                            .checked_add(1)
                            .expect("Cursor width should never be greater than u16::MAX");
                        assert!(
                            cursor.x <= sz.width,
                            "Cursor skipped rows should never exceed area width"
                        )
                    }
                    row::BuilderRowItem::Empty(n) => {
                        cursor.x = cursor
                            .x
                            .checked_add(n)
                            .expect("Cursor width should never be greater than u16::MAX");
                        assert!(
                            cursor.x <= sz.width,
                            "Cursor skipped rows should never exceed area width"
                        )
                    }
                };
            });

            cursor.y = cursor
                .y
                .checked_add(1)
                .expect("Cursor height should never be greater than u16::MAX");
            cursor.x = 0;
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::core::buffer::{builder::BuilderBuffer, flat::FlatBuffer, Buffer};
    use crate::core::draw::Position;
    use crate::style::Style;

    #[test]
    fn simple() {
        use crate::cell::Cell;

        const MY_BUFFER: BuilderBuffer = builder_buffer!(
            ["a", "b", empty(1)],
            empty(3),
            [empty(1), "b", cell("c", Style::EMPTY)],
            ["a1", "b", "c"],
            ["a2", "b", "c"],
            ["a3", "b", "c"],
            ["a4", "b", "c"],
            ["a5", "b", "c"]
        );

        let mut fb = FlatBuffer::new(MY_BUFFER.size);
        super::build(&MY_BUFFER, &mut fb);

        assert_eq!(fb.get_cell(Position::new(0, 0)).as_str(), "a");
        assert_eq!(fb.get_cell(Position::new(1, 0)).as_str(), "b");
        assert_eq!(fb.get_cell(Position::new(2, 0)), &Cell::EMPTY);

        assert_eq!(fb.get_cell(Position::new(1, 4)).as_str(), "b");
        assert_eq!(fb.get_cell(Position::new(2, 4)).as_str(), "c");

        assert_eq!(fb.get_cell(Position::new(0, 5)).as_str(), "a1");
        assert_eq!(fb.get_cell(Position::new(0, 6)).as_str(), "a2");
        assert_eq!(fb.get_cell(Position::new(0, 7)).as_str(), "a3");
        assert_eq!(fb.get_cell(Position::new(0, 8)).as_str(), "a4");
        assert_eq!(fb.get_cell(Position::new(0, 9)).as_str(), "a5");
    }
}
