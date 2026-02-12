//! Color types and helpers.
//!
//! This module provides `u32`-packed [`Color`] and [`ColorRgb`] types, as well as
//! gradient support along with sampling and LUT-based linear interpolation.
//!
//!
//! ## Colors
//!
//! - [`Color`] stores RGBA in a single `u32` (`0xRRGGBBAA`).
//! - [`ColorRgb`] stores RGB without alpha (`0xRRGGBB00`).
//!
//! The library is built with alpha blending support as one of it's core features,
//! which is why [`Color`] sees considerably more use compared to [`ColorRgb`].
//!
//! ## Gradients
//!
//! - [`GradientStop`] represents a single stop in a color gradient.
//! - [`ColorGradient`] stores a sequence of stops and can be sampled along
//!   a normalized `0.0..=1.0` range using [`sample_gradient`].
//!
//! ## Interpolation
//!
//! - [`lerp`] allows fast linear interpolation between two [`Color`]s.

use std::sync::Arc;

pub static BLEND_ALPHA_MULT: [[u8; 256]; 256] = {
    let mut lut = [[0u8; 256]; 256];
    let mut ta = 0;
    while ta < 256 {
        let mut ba = 0;
        while ba < 256 {
            // ba * (1 - ta/255) rounded
            let ta_f = ta as f32 / 255.0;
            let ba_f = ba as f32;
            let result = ba_f * (1.0 - ta_f);
            lut[ta as usize][ba as usize] = (result + 0.5) as u8;
            ba += 1;
        }
        ta += 1;
    }
    lut
};

pub static MUL_DIV_255: [[u8; 256]; 256] = {
    let mut lut = [[0u8; 256]; 256];
    let mut a = 0;
    while a < 256 {
        let mut b = 0;
        while b < 256 {
            // (a * b) / 255 rounded
            let result = (a as f32 * b as f32) / 255.0;
            lut[a as usize][b as usize] = (result + 0.5) as u8;
            b += 1;
        }
        a += 1;
    }
    lut
};

pub static RECIPROCAL_255_OVER_X: [u16; 256] = {
    let mut lut = [0u16; 256];
    let mut x = 1;
    while x < 256 {
        let recip = 255.0 / x as f32;
        lut[x] = (recip * 256.0 + 0.5) as u16;
        x += 1;
    }
    lut
};

pub static LERP_LUT_A: [[u8; 256]; 256] = {
    let mut lut: [[u8; 256]; 256] = [[0u8; 256]; 256];
    let mut channel_value: usize = 0;
    while channel_value < 256 {
        let mut t_value: usize = 0;
        while t_value < 256 {
            let scaled: usize = channel_value * (255 - t_value);
            let rounded: usize = scaled + 128;
            let final_value: u8 = (rounded / 255) as u8;
            lut[channel_value][t_value] = final_value;
            t_value += 1;
        }
        channel_value += 1;
    }
    lut
};

pub static LERP_LUT_B: [[u8; 256]; 256] = {
    let mut lut: [[u8; 256]; 256] = [[0u8; 256]; 256];
    let mut channel_value: usize = 0;
    while channel_value < 256 {
        let mut t_value: usize = 0;
        while t_value < 256 {
            let scaled: usize = channel_value * t_value;
            let rounded: usize = scaled + 128;
            let final_value: u8 = (rounded / 255) as u8;
            lut[channel_value][t_value] = final_value;
            t_value += 1;
        }
        channel_value += 1;
    }
    lut
};

/// A packed RGBA color stored in an `u32`.
///
/// Layout: `0xRR_GG_BB_AA`
///
/// # Examples
///
/// ```rust
/// use germterm::color::Color;
///
/// let color = Color::new(255, 0, 0, 255);
/// assert_eq!(color, Color::RED);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const CLEAR: Self = Self(0x00_00_00_00);
    pub const WHITE: Self = Self(0xFF_FF_FF_FF);
    pub const DARK_GRAY: Self = Self(0xA9_A9_A9_FF);
    pub const LIGHT_GRAY: Self = Self(0xD3_D3_D3_FF);
    pub const BLACK: Self = Self(0x00_00_00_FF);
    pub const RED: Self = Self(0xFF_00_00_FF);
    pub const GREEN: Self = Self(0x00_FF_00_FF);
    pub const BLUE: Self = Self(0x00_00_FF_FF);
    pub const YELLOW: Self = Self(0xFF_FF_00_FF);
    pub const CYAN: Self = Self(0x00_FF_FF_FF);
    pub const TEAL: Self = Self(0x00_80_80_FF);
    pub const VIOLET: Self = Self(0x7F_00_FF_FF);
    pub const PINK: Self = Self(0xFF_C0_CB_FF);
    pub const ORANGE: Self = Self(0xFF_A5_00_FF);
    pub const DARK_GREEN: Self = Self(0x08_48_08_FF);

    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32))
    }

    #[inline]
    pub fn r(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    #[inline]
    pub fn a(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    #[inline]
    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }

    #[inline]
    pub fn with_alpha(&self, a: u8) -> Self {
        Color((self.0 & 0xFFFF_FF00) | a as u32)
    }

    #[inline]
    pub fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        let r: f32 = ((self.0 >> 24) & 0xFF) as f32 / 255.0;
        let g: f32 = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let b: f32 = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let a: f32 = (self.0 & 0xFF) as f32 / 255.0;
        (r, g, b, a)
    }

    #[inline]
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::new(
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
            (a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

/// A packed RGB color stored in an `u32`.
///
/// Layout: `0xRR_GG_BB_00`
///
/// This struct is intended to be used in cases
/// where the alpha channel is not applicable.
pub struct ColorRgb(u32);

impl ColorRgb {
    pub const WHITE: Self = Self(0xFF_FF_FF_FF);
    pub const DARK_GRAY: Self = Self(0xA9_A9_A9_FF);
    pub const LIGHT_GRAY: Self = Self(0xD3_D3_D3_FF);
    pub const BLACK: Self = Self(0x00_00_00_FF);
    pub const RED: Self = Self(0xFF_00_00_FF);
    pub const GREEN: Self = Self(0x00_FF_00_FF);
    pub const BLUE: Self = Self(0x00_00_FF_FF);
    pub const YELLOW: Self = Self(0xFF_FF_00_FF);
    pub const CYAN: Self = Self(0x00_FF_FF_FF);
    pub const TEAL: Self = Self(0x00_80_80_FF);
    pub const VIOLET: Self = Self(0x7F_00_FF_FF);
    pub const PINK: Self = Self(0xFF_C0_CB_FF);
    pub const ORANGE: Self = Self(0xFF_A5_00_FF);
    pub const DARK_GREEN: Self = Self(0x08_48_08_FF);

    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRgb(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8))
    }

    #[inline]
    pub fn r(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }
}

impl From<ColorRgb> for Color {
    fn from(color: ColorRgb) -> Self {
        Color::new(color.r(), color.g(), color.b(), 255)
    }
}

/// A single stop in a [`ColorGradient`].
///
/// Each stop specifies a position `t` in the normalized range `0.0..=1.0`
/// and a [`Color`] at that position. Gradients are created by interpolating
/// between multiple stops.
#[derive(Clone)]
pub struct GradientStop {
    pub t: f32,
    pub color: Color,
}

impl GradientStop {
    /// Creates a new gradient stop.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use germterm::color::{GradientStop, Color};
    /// let stop = GradientStop::new(0.5, Color::RED);
    /// ```
    pub fn new(t: f32, color: Color) -> Self {
        GradientStop { t, color }
    }
}

/// A simple n-color gradient.
///
/// Stores a sequence of color stops [`GradientStop`] that can be sampled
/// along a normalized range `t` (0.0..=1.0) to produce interpolated colors.
///
/// Construct a `ColorGradient` via [`ColorGradient::new`] and sample colors
/// using [`sample_gradient`] or other helper functions.
///
/// The gradient is internally reference-counted [`Arc`] so it can be
/// cheaply cloned and shared.
#[derive(Clone)]
pub struct ColorGradient {
    pub stops: Arc<Vec<GradientStop>>,
}

impl ColorGradient {
    /// Creates a new color gradient from a vec or slice of [`GradientStop`]s.
    ///
    /// # Panics
    /// - If `stops` is empty.
    ///
    /// # Notes
    /// - `stops` should be in their intended visual order for the gradient to behave as expected.
    /// - When evaluating the gradient, `t` values are expected to be within `0.0..=1.0`.
    pub fn new(stops: Vec<GradientStop>) -> Self {
        assert!(!stops.is_empty(), "Gradient must have at least 1 stop");

        ColorGradient {
            stops: Arc::new(stops),
        }
    }
}

/// Samples a color from a `ColorGradient` at a normalized position `t`.
///
/// `t` should be in the range `0.0..=1.0`. Values outside this range are clamped.
///
/// # Example
///
/// ```rust,no_run
/// # use germterm::color::{ColorGradient, GradientStop, Color, sample_gradient};
/// let gradient = ColorGradient::new(vec![
///     GradientStop::new(0.0, Color::RED),
///     GradientStop::new(1.0, Color::BLUE),
/// ]);
/// let color = sample_gradient(&gradient, 0.75);
/// ```
#[inline]
pub fn sample_gradient(gradient: &ColorGradient, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    if gradient.stops.len() == 1 {
        return gradient.stops[0].color;
    }

    for window in gradient.stops.windows(2) {
        let a = &window[0];
        let b = &window[1];

        if t >= a.t && t <= b.t {
            let local_t = (t - a.t) / (b.t - a.t);
            return lerp(a.color, b.color, local_t);
        }
    }

    gradient.stops.last().unwrap().color
}

/// Linearly interpolates between two [`Color`]s.
///
/// Computes a color between `a` and `b` using the parameter `t`,
/// where `t = 0.0` returns `a` and `t = 1.0` returns `b`.
///
/// Values outside `0.0..=1.0` are clamped to this range.
///
/// # Example
///
/// ```rust,no_run
/// # use germterm::color::{Color, lerp};
/// let purple = lerp(Color::RED, Color::BLUE, 0.5);
/// ```
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    let clamped_t: f32 = t.clamp(0.0, 1.0);
    let t_scaled: u8 = (clamped_t * 255.0).round() as u8;

    let (a_r, a_g, a_b, a_a) = a.rgba();
    let (b_r, b_g, b_b, b_a) = b.rgba();

    let out_r: u8 =
        LERP_LUT_A[a_r as usize][t_scaled as usize] + LERP_LUT_B[b_r as usize][t_scaled as usize];
    let out_g: u8 =
        LERP_LUT_A[a_g as usize][t_scaled as usize] + LERP_LUT_B[b_g as usize][t_scaled as usize];
    let out_b: u8 =
        LERP_LUT_A[a_b as usize][t_scaled as usize] + LERP_LUT_B[b_b as usize][t_scaled as usize];
    let out_a: u8 =
        LERP_LUT_A[a_a as usize][t_scaled as usize] + LERP_LUT_B[b_a as usize][t_scaled as usize];

    Color::new(out_r, out_g, out_b, out_a)
}

#[inline]
pub(crate) fn blend_source_over(bottom: Color, top: Color) -> Color {
    let (tr, tg, tb, ta) = top.rgba();
    let (br, bg, bb, ba) = bottom.rgba();

    let alpha_mult = BLEND_ALPHA_MULT[ta as usize][ba as usize] as u16;
    let out_a = ta as u16 + alpha_mult;

    if out_a == 0 {
        return Color::CLEAR;
    }

    #[inline]
    fn compute_channel(tc: u8, bc: u8, ta: u8, alpha_mult: u16, out_a: u16) -> u8 {
        // numerator = tc * ta + bc * alpha_mult
        let tc_ta = MUL_DIV_255[tc as usize][ta as usize] as u16 * 255;
        let bc_alpha = MUL_DIV_255[bc as usize][alpha_mult as usize] as u16 * 255;
        let numerator = tc_ta + bc_alpha;

        // out_c = (numerator * 255) / out_a
        let recip = RECIPROCAL_255_OVER_X[out_a as usize] as u32;
        let result = ((numerator as u32 * recip) + (1 << 7)) >> 8;

        (result >> 8) as u8
    }

    let out_r = compute_channel(tr, br, ta, alpha_mult, out_a);
    let out_g = compute_channel(tg, bg, ta, alpha_mult, out_a);
    let out_b = compute_channel(tb, bb, ta, alpha_mult, out_a);

    Color::new(out_r, out_g, out_b, out_a as u8)
}
