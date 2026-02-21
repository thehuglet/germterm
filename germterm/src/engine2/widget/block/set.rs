/// Common border styles for block widgets.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BorderStyle {
    /// Single thin line: ┌─┐│└┘
    #[default]
    Single,
    /// Double line: ╔═╗║╚╝
    Double,
    /// Bold / thick single line: ┏━┓┃┗┛
    Bold,
    /// Rounded corners: ╭─╮│╰╯
    Rounded,
    /// ASCII fallback: +-+|+-+|
    Ascii,
}

pub trait BlockSet {
    fn top(&self, cur: &str) -> &str;
    fn top_left(&self, cur: &str) -> &str;
    fn top_right(&self, cur: &str) -> &str;
    fn right(&self, cur: &str) -> &str;
    fn bottom(&self, cur: &str) -> &str;
    fn bottom_left(&self, cur: &str) -> &str;
    fn bottom_right(&self, cur: &str) -> &str;
    fn left(&self, cur: &str) -> &str;
}

/// A UTF-8 border glyph stored inline as a byte array.
///
/// Layout: `[b0, b1, b2, len]` where `len` is the number of valid UTF-8 bytes
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BorderBytes([u8; 4]);

/// Constructs a [`BorderBytes`] value from a string literal at compile time.
///
/// The string must be a single Unicode scalar encoded in 1–3 UTF-8 bytes.
const fn border_bytes(s: &str) -> BorderBytes {
    let b = s.as_bytes();
    let len = b.len();
    match len {
        1 => BorderBytes([b[0], 0, 0, 1]),
        2 => BorderBytes([b[0], b[1], 0, 2]),
        3 => BorderBytes([b[0], b[1], b[2], 3]),
        _ => panic!("border glyph must be less than 4 UTF-8 bytes"),
    }
}

/// Reinterprets the byte buffer stored in a field as a `&str`.
impl BorderBytes {
    fn as_str(&self) -> &str {
        let len = self.0[3] as usize;
        // SAFETY: guaranteed valid UTF-8 bytes were produced by `border_bytes`.
        unsafe { std::str::from_utf8_unchecked(&self.0[..len]) }
    }
}

/// A fixed, non-merging border character set for a single [`BorderStyle`].
///
/// Each associated constant group covers all eight border positions for one style.
/// The `cur` parameter passed to [`BlockSet`] methods is ignored.
/// `SimpleBorder` always returns the same character regardless of what is already in the cell.
pub struct SimpleBorderSet {
    top_left: BorderBytes,
    top: BorderBytes,
    top_right: BorderBytes,
    left: BorderBytes,
    right: BorderBytes,
    bottom_left: BorderBytes,
    bottom: BorderBytes,
    bottom_right: BorderBytes,
}

// we use this to avoid repetitive code
macro_rules! bb {
    ($($name:ident = $symbol:literal),+) => {
        $(
            pub const $name: BorderBytes = border_bytes($symbol);
        )+
    };
}
impl SimpleBorderSet {
    bb! {
        SINGLE_TOP_LEFT = "┌",
        SINGLE_TOP = "─",
        SINGLE_TOP_RIGHT = "┐",
        SINGLE_LEFT = "│",
        SINGLE_RIGHT = "│",
        SINGLE_BOTTOM_LEFT = "└",
        SINGLE_BOTTOM = "─",
        SINGLE_BOTTOM_RIGHT = "┘",
        DOUBLE_TOP_LEFT = "╔",
        DOUBLE_TOP = "═",
        DOUBLE_TOP_RIGHT = "╗",
        DOUBLE_LEFT = "║",
        DOUBLE_RIGHT = "║",
        DOUBLE_BOTTOM_LEFT = "╚",
        DOUBLE_BOTTOM = "═",
        DOUBLE_BOTTOM_RIGHT = "╝",
        BOLD_TOP_LEFT = "┏",
        BOLD_TOP = "━",
        BOLD_TOP_RIGHT = "┓",
        BOLD_LEFT = "┃",
        BOLD_RIGHT = "┃",
        BOLD_BOTTOM_LEFT = "┗",
        BOLD_BOTTOM = "━",
        BOLD_BOTTOM_RIGHT = "┛",
        ROUNDED_TOP_LEFT = "╭",
        ROUNDED_TOP = "─",
        ROUNDED_TOP_RIGHT = "╮",
        ROUNDED_LEFT = "│",
        ROUNDED_RIGHT = "│",
        ROUNDED_BOTTOM_LEFT = "╰",
        ROUNDED_BOTTOM = "─",
        ROUNDED_BOTTOM_RIGHT = "╯",
        ASCII_TOP_LEFT = "+",
        ASCII_TOP = "-",
        ASCII_TOP_RIGHT = "+",
        ASCII_LEFT = "|",
        ASCII_RIGHT = "|",
        ASCII_BOTTOM_LEFT = "+",
        ASCII_BOTTOM = "-",
        ASCII_BOTTOM_RIGHT = "+"
    }

    pub const SINGLE: SimpleBorderSet = SimpleBorderSet {
        top_left: Self::SINGLE_TOP_LEFT,
        top: Self::SINGLE_TOP,
        top_right: Self::SINGLE_TOP_RIGHT,
        left: Self::SINGLE_LEFT,
        right: Self::SINGLE_RIGHT,
        bottom_left: Self::SINGLE_BOTTOM_LEFT,
        bottom: Self::SINGLE_BOTTOM,
        bottom_right: Self::SINGLE_BOTTOM_RIGHT,
    };

    pub const DOUBLE: SimpleBorderSet = SimpleBorderSet {
        top_left: Self::DOUBLE_TOP_LEFT,
        top: Self::DOUBLE_TOP,
        top_right: Self::DOUBLE_TOP_RIGHT,
        left: Self::DOUBLE_LEFT,
        right: Self::DOUBLE_RIGHT,
        bottom_left: Self::DOUBLE_BOTTOM_LEFT,
        bottom: Self::DOUBLE_BOTTOM,
        bottom_right: Self::DOUBLE_BOTTOM_RIGHT,
    };

    pub const BOLD: SimpleBorderSet = SimpleBorderSet {
        top_left: Self::BOLD_TOP_LEFT,
        top: Self::BOLD_TOP,
        top_right: Self::BOLD_TOP_RIGHT,
        left: Self::BOLD_LEFT,
        right: Self::BOLD_RIGHT,
        bottom_left: Self::BOLD_BOTTOM_LEFT,
        bottom: Self::BOLD_BOTTOM,
        bottom_right: Self::BOLD_BOTTOM_RIGHT,
    };

    pub const ROUNDED: SimpleBorderSet = SimpleBorderSet {
        top_left: Self::ROUNDED_TOP_LEFT,
        top: Self::ROUNDED_TOP,
        top_right: Self::ROUNDED_TOP_RIGHT,
        left: Self::ROUNDED_LEFT,
        right: Self::ROUNDED_RIGHT,
        bottom_left: Self::ROUNDED_BOTTOM_LEFT,
        bottom: Self::ROUNDED_BOTTOM,
        bottom_right: Self::ROUNDED_BOTTOM_RIGHT,
    };

    pub const ASCII: SimpleBorderSet = SimpleBorderSet {
        top_left: Self::ASCII_TOP_LEFT,
        top: Self::ASCII_TOP,
        top_right: Self::ASCII_TOP_RIGHT,
        left: Self::ASCII_LEFT,
        right: Self::ASCII_RIGHT,
        bottom_left: Self::ASCII_BOTTOM_LEFT,
        bottom: Self::ASCII_BOTTOM,
        bottom_right: Self::ASCII_BOTTOM_RIGHT,
    };
}

impl BlockSet for SimpleBorderSet {
    fn top_left(&self, _cur: &str) -> &str {
        self.top_left.as_str()
    }
    fn top(&self, _cur: &str) -> &str {
        self.top.as_str()
    }
    fn top_right(&self, _cur: &str) -> &str {
        self.top_right.as_str()
    }
    fn left(&self, _cur: &str) -> &str {
        self.left.as_str()
    }
    fn right(&self, _cur: &str) -> &str {
        self.right.as_str()
    }
    fn bottom_left(&self, _cur: &str) -> &str {
        self.bottom_left.as_str()
    }
    fn bottom(&self, _cur: &str) -> &str {
        self.bottom.as_str()
    }
    fn bottom_right(&self, _cur: &str) -> &str {
        self.bottom_right.as_str()
    }
}
