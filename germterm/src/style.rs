use core::mem::MaybeUninit;

use crate::color::Color;
use bitflags::bitflags;

bitflags! {
    /// Attributes that can be applied to drawn text.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Attributes: u8 {
        const BOLD          = 0b_00000001;
        const ITALIC        = 0b_00000010;
        const UNDERLINED    = 0b_00000100;
        const HIDDEN        = 0b_00001000;

        // This is the same as all of the bits in user code.
        // Internally we use this mask to filter out unknown bits form a user.
        #[doc(hidden)]
        const KNOWN = Self::BOLD.bits() | Self::ITALIC.bits() | Self::UNDERLINED.bits() | Self::HIDDEN.bits();
        // These are doc hidden as users should not use them
        #[doc(hidden)]
        const NO_FG_COLOR   = 0b_00010000;
        #[doc(hidden)]
        const NO_BG_COLOR   = 0b_00100000;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Style {
    fg: MaybeUninit<Color>,
    bg: MaybeUninit<Color>,
    // The colors are initialized if `Attributes::NO_*_COLOR` are not set.
    attributes: Attributes,
}

impl Default for Style {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        self.fg() == other.fg()
            && self.bg() == other.bg()
            && self.attributes() == other.attributes()
    }
}

impl Eq for Style {}

impl Style {
    pub const EMPTY: Self = Style {
        fg: MaybeUninit::uninit(),
        bg: MaybeUninit::uninit(),
        attributes: Attributes::from_bits_truncate(
            Attributes::NO_FG_COLOR.bits() | Attributes::NO_BG_COLOR.bits(),
        ),
    };

    pub fn new(
        fg: impl Into<Option<Color>>,
        bg: impl Into<Option<Color>>,
        attributes: Attributes,
    ) -> Self {
        Self::EMPTY
            .with_fg(fg)
            .with_bg(bg)
            .set_attributes(attributes)
    }

    #[inline]
    pub fn with_fg(mut self, fg: impl Into<Option<Color>>) -> Self {
        let c: Option<Color> = fg.into();
        if let Some(c) = c {
            self.fg.write(c);
            self.attributes.remove(Attributes::NO_FG_COLOR);
        } else {
            self.attributes |= Attributes::NO_FG_COLOR;
        }
        self
    }

    #[inline]
    pub fn fg(&self) -> Option<Color> {
        self.has_fg().then(|| unsafe { self.fg.assume_init() })
    }

    #[inline]
    pub fn has_fg(&self) -> bool {
        !self.attributes.contains(Attributes::NO_FG_COLOR)
    }

    #[inline]
    pub fn with_bg(mut self, bg: impl Into<Option<Color>>) -> Self {
        let c: Option<Color> = bg.into();
        if let Some(c) = c {
            self.bg.write(c);
            self.attributes.remove(Attributes::NO_BG_COLOR);
        } else {
            self.attributes |= Attributes::NO_BG_COLOR;
        }

        self
    }

    #[inline]
    pub fn bg(&self) -> Option<Color> {
        self.has_bg().then(|| unsafe { self.bg.assume_init() })
    }

    #[inline]
    pub fn has_bg(&self) -> bool {
        !self.attributes.contains(Attributes::NO_BG_COLOR)
    }

    #[inline]
    pub fn attributes(&self) -> Attributes {
        // We don't really care if the fg/bg bits can be read but its technically an implementation
        // detail so lets keep it that way.
        self.attributes & Attributes::KNOWN
    }

    #[inline]
    pub fn set_attributes(mut self, attributes: Attributes) -> Self {
        // Only replace the user-visible attribute bits; preserve the internal
        let color_bits = self.attributes & !Attributes::KNOWN;
        self.attributes = (attributes & Attributes::KNOWN) | color_bits;
        self
    }

    pub fn merged(self, other: Self) -> Self {
        Self::EMPTY
            .with_fg(other.fg().or(self.fg()))
            .with_bg(other.bg().or(self.bg()))
            .set_attributes(other.attributes() | self.attributes())
    }

    pub fn merge(&mut self, other: Self) {
        *self = Self::EMPTY
            .with_fg(other.fg().or(self.fg()))
            .with_bg(other.bg().or(self.bg()))
            .set_attributes(other.attributes() | self.attributes());
    }
}

// Under here are various convenience functions and macros to reduce duplication in Styleable
//
//
//
#[inline(always)]
fn keep_if(attr: Attributes, cond: bool) -> Attributes {
    Attributes::from_bits_retain(attr.bits() * cond as u8)
}
macro_rules! attr_get_set_with {
    ($name:ident, $set_name:ident, $with_name:ident, $attr_val:expr) => {
        #[inline]
        fn $name(&self) -> bool {
            self.attributes().contains($attr_val)
        }
        #[inline]
        fn $set_name(&mut self, $name: bool) {
            self.set_attributes(self.attributes() | keep_if($attr_val, $name));
        }
        #[inline]
        fn $with_name(mut self, $name: bool) -> Self {
            self.$set_name($name);
            self
        }
    };
}
macro_rules! color_get_set_with {
    ($name:ident, $set_name:ident, $with_name:ident) => {
        #[inline]
        fn $name(&self) -> Option<Color> {
            self.style().$name()
        }
        #[inline]
        fn $set_name(&mut self, $name: impl Into<Option<Color>>) {
            self.set_style(self.style().$with_name(($name).into()));
        }
        #[inline]
        fn $with_name(mut self, $name: impl Into<Option<Color>>) -> Self {
            self.$set_name($name);
            self
        }
    };
}
pub trait Stylable: Sized {
    color_get_set_with!(fg, set_fg, with_fg);
    color_get_set_with!(bg, set_bg, with_bg);

    #[inline]
    fn attributes(&self) -> Attributes {
        self.style().attributes()
    }
    #[inline]
    fn set_attributes(&mut self, attributes: Attributes) {
        let style = self.style();
        self.set_style(style.set_attributes(attributes));
    }
    #[inline]
    fn with_attributes(mut self, attributes: Attributes) -> Self {
        self.set_attributes(attributes);
        self
    }

    attr_get_set_with!(bold, set_bold, with_bold, Attributes::BOLD);
    attr_get_set_with!(italic, set_italic, with_italic, Attributes::ITALIC);
    attr_get_set_with!(
        underlined,
        set_underlined,
        with_underlined,
        Attributes::UNDERLINED
    );
    attr_get_set_with!(hidden, set_hidden, with_hidden, Attributes::HIDDEN);

    #[inline]
    fn colors(&self) -> (Option<Color>, Option<Color>) {
        (self.fg(), self.bg())
    }
    #[inline]
    fn set_colors(&mut self, fg: impl Into<Option<Color>>, bg: impl Into<Option<Color>>) {
        self.set_fg(fg);
        self.set_bg(bg);
    }
    #[inline]
    fn with_colors(self, fg: impl Into<Option<Color>>, bg: impl Into<Option<Color>>) -> Self {
        self.with_fg(fg).with_bg(bg)
    }

    fn style(&self) -> Style;
    fn set_style(&mut self, style: Style);
    #[inline]
    fn with_style(mut self, style: Style) -> Self {
        self.set_style(style);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Default

    #[test]
    fn default_has_no_fg() {
        let style = Style::default();
        assert!(!style.has_fg());
        assert!(style.fg().is_none());
    }

    #[test]
    fn default_has_no_bg() {
        let style = Style::default();
        assert!(!style.has_bg());
        assert!(style.bg().is_none());
    }

    #[test]
    fn default_has_no_attributes() {
        let style = Style::default();
        assert_eq!(style.attributes(), Attributes::empty());
    }

    // set_fg / fg / has_fg

    #[test]
    fn set_fg_with_color_enables_fg() {
        let style = Style::default().with_fg(Color::RED);
        assert!(style.has_fg());
        assert_eq!(style.fg(), Some(Color::RED));
    }

    #[test]
    fn set_fg_with_some_color_enables_fg() {
        let style = Style::default().with_fg(Some(Color::BLUE));
        assert!(style.has_fg());
        assert_eq!(style.fg(), Some(Color::BLUE));
    }

    #[test]
    fn set_fg_with_none_clears_fg() {
        let style = Style::default().with_fg(Color::RED).with_fg(None);
        assert!(!style.has_fg());
        assert!(style.fg().is_none());
    }

    #[test]
    fn set_fg_overwrites_previous_color() {
        let style = Style::default().with_fg(Color::RED).with_fg(Color::GREEN);
        assert_eq!(style.fg(), Some(Color::GREEN));
    }

    #[test]
    fn set_fg_does_not_affect_bg() {
        let style = Style::default().with_fg(Color::RED);
        assert!(!style.has_bg());
        assert!(style.bg().is_none());
    }

    // set_bg / bg / has_bg

    #[test]
    fn set_bg_with_color_enables_bg() {
        let style = Style::default().with_bg(Color::WHITE);
        assert!(style.has_bg());
        assert_eq!(style.bg(), Some(Color::WHITE));
    }

    #[test]
    fn set_bg_with_some_color_enables_bg() {
        let style = Style::default().with_bg(Some(Color::BLACK));
        assert!(style.has_bg());
        assert_eq!(style.bg(), Some(Color::BLACK));
    }

    #[test]
    fn set_bg_with_none_clears_bg() {
        let style = Style::default().with_bg(Color::WHITE).with_bg(None);
        assert!(!style.has_bg());
        assert!(style.bg().is_none());
    }

    #[test]
    fn set_bg_overwrites_previous_color() {
        let style = Style::default().with_bg(Color::WHITE).with_bg(Color::TEAL);
        assert_eq!(style.bg(), Some(Color::TEAL));
    }

    #[test]
    fn set_bg_does_not_affect_fg() {
        let style = Style::default().with_bg(Color::WHITE);
        assert!(!style.has_fg());
        assert!(style.fg().is_none());
    }

    // set_attributes / attributes

    #[test]
    fn set_attributes_bold_is_reflected() {
        let style = Style::default().set_attributes(Attributes::BOLD);
        assert_eq!(style.attributes(), Attributes::BOLD);
    }

    #[test]
    fn set_attributes_italic_is_reflected() {
        let style = Style::default().set_attributes(Attributes::ITALIC);
        assert_eq!(style.attributes(), Attributes::ITALIC);
    }

    #[test]
    fn set_attributes_underlined_is_reflected() {
        let style = Style::default().set_attributes(Attributes::UNDERLINED);
        assert_eq!(style.attributes(), Attributes::UNDERLINED);
    }

    #[test]
    fn set_attributes_hidden_is_reflected() {
        let style = Style::default().set_attributes(Attributes::HIDDEN);
        assert_eq!(style.attributes(), Attributes::HIDDEN);
    }

    #[test]
    fn set_attributes_all_known_flags_round_trip() {
        let all =
            Attributes::BOLD | Attributes::ITALIC | Attributes::UNDERLINED | Attributes::HIDDEN;
        let style = Style::default().set_attributes(all);
        assert_eq!(style.attributes(), all);
    }

    #[test]
    fn set_attributes_internal_color_bits_are_ignored() {
        // Passing the internal sentinel bits via set_attributes should have no effect on
        // the publicly visible attributes() value.
        let internal = Attributes::NO_FG_COLOR | Attributes::NO_BG_COLOR;
        let style = Style::default().set_attributes(internal);
        assert_eq!(style.attributes(), Attributes::empty());
    }

    #[test]
    fn attributes_does_not_expose_internal_color_bits() {
        // Even after setting fg/bg, attributes() must only return KNOWN bits.
        let style = Style::default()
            .with_fg(Color::RED)
            .with_bg(Color::BLUE)
            .set_attributes(Attributes::BOLD);
        let attrs = style.attributes();
        assert!(!attrs.contains(Attributes::NO_FG_COLOR));
        assert!(!attrs.contains(Attributes::NO_BG_COLOR));
        assert!(attrs.contains(Attributes::BOLD));
    }

    // combined usage

    #[test]
    fn builder_chain_fg_bg_and_attributes() {
        let style = Style::default()
            .with_fg(Color::CYAN)
            .with_bg(Color::DARK_GRAY)
            .set_attributes(Attributes::BOLD | Attributes::UNDERLINED);

        assert_eq!(style.fg(), Some(Color::CYAN));
        assert_eq!(style.bg(), Some(Color::DARK_GRAY));
        assert_eq!(
            style.attributes(),
            Attributes::BOLD | Attributes::UNDERLINED
        );
    }
}
