pub mod row;

use crate::core::buffer::builder::row::BuilderRow;

#[doc(hidden)]
pub struct BuilderBuffer<const N: usize> {
    pub rows: [BuilderRow; N],
}

#[doc(hidden)]
#[macro_export]
macro_rules! builder_buffer_item {
    (skip) => {
        None
    };
    ($($row_stuff:tt)*) => {
        Some($crate::builder_row!($($row_stuff)*))
    };
}

macro_rules! builder_buffer {
    ($([$($row_stuff:tt)*]),*) => {{
        [
            $($crate::builder_buffer_item!($($row_stuff)*)),*
        ]
    }};
}
