use super::Buffer;
use crate::{color::Color, core::Position};

#[doc(hidden)]
#[track_caller]
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
