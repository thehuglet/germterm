use std::io::Write;

use super::Buffer;
use crate::{color::Color, core::Position};

/// Dumps the contents of a buffer using a writer.
///
/// Each row is separated by a newline. Only the character of each cell is included.
pub fn dump_buffer(buffer: &dyn Buffer, writer: &mut dyn Write) -> std::io::Result<()> {
    let size = buffer.size();
    for y in 0..size.height {
        for x in 0..size.width {
            let cell = buffer.get_cell(Position::new(x, y));
            writer.write_all(cell.as_str().as_bytes())?;
        }
        if y < size.height - 1 {
            writer.write_all(b"\n")?;
        }
    }
    Ok(())
}

/// Dumps the contents of a buffer to a string.
///
/// Preallocates the estimated capacity for better performance.
pub fn dump_buffer_to_string(buffer: &dyn Buffer) -> String {
    let size = buffer.size();
    let mut result = Vec::with_capacity((size.width as usize + 1) * size.height as usize);
    let _ = dump_buffer(buffer, &mut result);
    String::from_utf8(result).unwrap_or_default()
}

#[doc(hidden)]
pub fn buf_cmp(lhs: &dyn Buffer, rhs: &dyn Buffer) -> impl Iterator<Item = (Position, CellDiff)> {
    assert_eq!(lhs.size(), rhs.size());

    let sz = lhs.size();
    (0..sz.height).flat_map(move |y| {
        (0..sz.width).filter_map(move |x| {
            let pos = Position { x, y };
            let lhs_cell = lhs.get_cell(pos);
            let lhs_style = lhs_cell.style();
            let rhs_cell = rhs.get_cell(pos);
            let rhs_style = lhs_cell.style();

            fn cmp_ret<T: Eq>(lhs: T, rhs: T) -> Option<DiffItem<T>> {
                if lhs != rhs {
                    Some(DiffItem {
                        expected: lhs,
                        found: rhs,
                    })
                } else {
                    None
                }
            }

            let cd = CellDiff {
                fg: cmp_ret(lhs_style.fg(), rhs_style.fg()),
                bg: cmp_ret(lhs_style.bg(), rhs_style.bg()),
                ch: cmp_ret(lhs_cell.as_str().into(), rhs_cell.as_str().into()),
            };

            if cd == CellDiff::default() {
                None
            } else {
                Some((pos, cd))
            }
        })
    })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct DiffItem<T> {
    pub expected: T,
    pub found: T,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct CellDiff {
    pub fg: Option<DiffItem<Option<Color>>>,
    pub bg: Option<DiffItem<Option<Color>>>,
    pub ch: Option<DiffItem<Box<str>>>,
}

#[macro_export]
macro_rules! buf_str {
    ($($line:literal),+ $(,)?) => {
        {
            use $crate::bbq_inner as b;
            b!($($line)+)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bbq_inner {
    ($single:literal) => {{$single}};
    ($first:literal $($rest:tt)*) => {{
        #[allow(unused)]
        use ::core::{concat as c, assert as a};
        #[allow(unused)]
        const L: usize = $first.len();
        b!([$first, "\n"] $($rest)*)
    }};
    ([$($loaded:literal),+] $next:literal $($rest:tt)+) => {{
        const _: () = a!(L == $next.len());
        b!([$($loaded),+, $next, "\n"] $($rest)+)
    }};
    ([$($loaded:literal),+] $last:literal) => {
        c!($($loaded),+, $last)
    };
}

// TODO: use trybuild for testing different length strings failing
#[cfg(test)]
mod tests {
    #[rustfmt::skip]
    const FOUND: &str = crate::buf_str!(
        "1234567", 
        "ABCDEFG", 
        "!@#$%^&",
    );

    const EXPECTED: &str = "1234567\nABCDEFG\n!@#$%^&";

    #[test]
    fn buffer_literal_formats_correctly() {
        assert_eq!(FOUND, EXPECTED);
    }
}
