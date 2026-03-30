use crate::style::Style;

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct BuilderRow {
    pub consumed: u16,
    pub items: fn() -> Box<[BuilderRowItem]>,
}

#[doc(hidden)]
pub enum BuilderRowItem {
    Cell(&'static str, Style),
    Empty(u16),
}

impl BuilderRowItem {
    const fn consumed(&self) -> u16 {
        match self {
            Self::Cell(_, _) => 1,
            Self::Empty(skipped) => *skipped,
        }
    }
}

/// Creates a [`BuilderRowItem`] from a simplified syntax.
///
/// This is a helper macro used internally by [`builder_row!`] to convert different
/// item types into `BuilderRowItem` variants. It is automatically exported so it can
/// be used in const contexts.
///
/// # Note
///
/// This macro requires that variants of [`BuilderRow`] are exported as lowercased.
///
/// # Syntax
///
/// - `empty(n)` - Creates an empty row item that skips `n` cell positions
/// - `cell("text")` - Creates a cell with the given text and [`Style::EMPTY`]
/// - `cell("text", style)` - Creates a cell with custom styling
///
/// # Examples
///
/// ```rust,ignore
/// // These are typically used within builder_row!:
/// let item1 = builder_row_item!(empty(5));           // Skip 5 positions
/// let item2 = builder_row_item!(cell("Hello"));      // Text with default style
/// let item3 = builder_row_item!(cell("Hi", STYLE));  // Text with custom style
/// ```
///
/// [`BuilderRowItem`]: crate::core::buffer::builder::row::BuilderRowItem
/// [`Style::EMPTY`]: crate::style::Style::EMPTY
#[doc(hidden)]
#[macro_export]
macro_rules! builder_row_item {
    ($s:literal) => {
        cell($s, ST::EMPTY)
    };
    (cell($s:literal)) => {
        cell($s, ST::EMPTY)
    };
    ($name:ident($($args:expr),*)) => {
        $name($($args),*)
    };
}

#[doc(hidden)]
pub const fn consumed_counter(items: &[BuilderRowItem]) -> u16 {
    let mut i = 0;
    let mut consumed = 0;
    while i < items.len() {
        consumed += items[i].consumed();
        i += 1;
    }

    consumed
}

/// Creates a [`BuilderRow`] at compile time from a list of row items.
///
/// This macro constructs a row buffer with cells and empty spaces, calculating
/// the total consumed cell count at compile time. It is designed to be used
/// in `const` or `static` contexts testing widget implementations and UI layout.
///
/// # Syntax
///
/// ```ignore
/// builder_row!(item1, item2, item3, ...)
/// ```
///
/// Where each item can be:
/// - `cell("text")` - A cell containing text with default style
/// - `cell("text", style)` - A cell with custom styling
/// - `empty(n)` - Skip `n` cell positions (creates empty space)
///
/// # Returns
///
/// Returns a [`BuilderRow`] struct containing:
/// - `consumed`: The total number of cell positions this row occupies
/// - `items`: A function that produces the array of [`BuilderRowItem`]s
///
/// # Examples
///
/// ## Basic usage with cells
/// ```rust
/// use germterm::builder_row;
///
/// const ROW: germterm::core::buffer::builder::row::BuilderRow =
///     builder_row!(cell("Hello"), cell("World"));
/// ```
///
/// ## With custom styling
/// ```rust
/// use germterm::{builder_row, style::Style};
///
/// const HIGHLIGHT: Style = Style::new().fg(germterm::color::RED);
/// const ROW: germterm::core::buffer::builder::row::BuilderRow =
///     builder_row!(cell("Normal"), cell("Important", HIGHLIGHT));
/// ```
///
/// ## With empty spaces
/// ```rust
/// use germterm::builder_row;
///
/// // Creates a row with "Left", 5 empty positions, then "Right"
/// const ROW: germterm::core::buffer::builder::row::BuilderRow =
///     builder_row!(cell("Left"), empty(5), cell("Right"));
/// ```
///
/// ## Compile-time construction
/// ```rust
/// use germterm::builder_row;
///
/// // These are evaluated at compile time with no runtime overhead
/// static STATIC_ROW: germterm::core::buffer::builder::row::BuilderRow =
///     builder_row!(cell("Static"));
/// const CONST_ROW: germterm::core::buffer::builder::row::BuilderRow =
///     builder_row!(cell("Const"));
/// ```
///
/// [`BuilderRow`]: crate::core::buffer::builder::row::BuilderRow
/// [`BuilderRowItem`]: crate::core::buffer::builder::row::BuilderRowItem
#[doc(hidden)]
#[macro_export]
macro_rules! builder_row {
    ($($row_item_ident:tt$(($($row_item_args:expr),*))?),*) => {{
        use $crate::core::buffer::builder;
        #[allow(unused_imports)]
        const CONSUMED: u16 = builder::row::consumed_counter(&[$($crate::builder_row_item!($row_item_ident$(($($row_item_args),*))?)),*]);
        fn rows() -> Box<[BRI]> {
            let items = [$($crate::builder_row_item!($row_item_ident$(($($row_item_args),*))?)),*];
            Box::new(items)
        }

       builder::row::BuilderRow {consumed: CONSUMED, items: rows}
    }};
}

#[cfg(test)]
mod tests {
    use crate::core::buffer::builder::row::BuilderRowItem::{Cell as cell};
    use crate::core::buffer::builder::row::{BuilderRow, BuilderRowItem as BRI};
    use crate::style::Style as ST;

    #[allow(unused)]
    static ROW_STATIC: BuilderRow = crate::builder_row!("a");
    #[allow(unused)]
    const ROW_CONST: BuilderRow = crate::builder_row!(cell("a", ST::EMPTY));

    #[test]
    fn build_row() {
        // This is to ensure that we can construct the values at compile time.
        //
        // Alternative implementations should not produce warnings related interior mutability with
        // the declarations below.
        static ROW_STATIC: BuilderRow = crate::builder_row!("a");
        const ROW_CONST: BuilderRow = crate::builder_row!(cell("a", ST::EMPTY));
        assert_eq!(ROW_STATIC.consumed, ROW_CONST.consumed);
    }
}
