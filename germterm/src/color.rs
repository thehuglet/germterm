use std::sync::Arc;

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

/// A packed RGBA color stored as a single `u32`.
///
/// Layout:
/// `0xRR_GG_BB_AA`
///
/// # Examples
///
/// ```
/// let color = Color::new(255, 0, 0, 255);
/// assert_eq!(color, Color::RED);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const WHITE: Self = Color(0xFF_FF_FF_FF);
    pub const DARK_GRAY: Self = Color(0xA9_A9_A9_FF);
    pub const LIGHT_GRAY: Self = Color(0xD3_D3_D3_FF);
    pub const BLACK: Self = Color(0x00_00_00_FF);
    pub const RED: Color = Color(0xFF_00_00_FF);
    pub const GREEN: Color = Color(0x00_FF_00_FF);
    pub const BLUE: Color = Color(0x00_00_FF_FF);
    pub const YELLOW: Color = Color(0xFF_FF_00_FF);
    pub const CYAN: Color = Color(0x00_FF_FF_FF);
    pub const TEAL: Color = Color(0x00_80_80_FF);
    pub const LIME: Color = Color(0x00_FF_00_FF);
    pub const VIOLET: Color = Color(0x7F_00_FF_FF);
    pub const PINK: Color = Color(0xFF_C0_CB_FF);
    pub const ORANGE: Color = Color(0xFF_A5_00_FF);
    pub const DARK_GREEN: Color = Color(0x08_48_08_FF);
    pub const CLEAR: Color = Color(0x00_00_00_00);

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

#[derive(Clone)]
pub struct GradientStop {
    pub t: f32,
    pub color: Color,
}

impl GradientStop {
    pub fn new(t: f32, color: Color) -> Self {
        GradientStop { t, color }
    }
}

#[derive(Clone)]
pub struct ColorGradient {
    pub stops: Arc<Vec<GradientStop>>,
}

impl ColorGradient {
    /// # SAFETY
    /// - There must be at least 1 stop.
    /// - `stops` must be in the intended visual order.
    /// - `t` should be in 0.0..=1.0.
    pub fn new(stops: Vec<GradientStop>) -> Self {
        assert!(!stops.is_empty(), "Gradient must have at least 1 stop");

        ColorGradient {
            stops: Arc::new(stops),
        }
    }
}

#[inline]
pub fn blend_source_over(bottom: Color, top: Color) -> Color {
    let (br, bg, bb, ba) = bottom.rgba_f32();
    let (tr, tg, tb, ta) = top.rgba_f32();

    let out_a = ta + ba * (1.0 - ta);
    if out_a <= 0.0 {
        return Color::CLEAR;
    }

    let out_r = (tr * ta + br * ba * (1.0 - ta)) / out_a;
    let out_g = (tg * ta + bg * ba * (1.0 - ta)) / out_a;
    let out_b = (tb * ta + bb * ba * (1.0 - ta)) / out_a;

    Color::from_f32(out_r, out_g, out_b, out_a)
}

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

    // # SAFETY
    // `ColorGradient::new` requires at least 1 stop to be present
    gradient.stops.last().unwrap().color
}

pub fn lerp(color_a: Color, color_b: Color, t: f32) -> Color {
    let clamped_t: f32 = t.clamp(0.0, 1.0);
    let t_scaled: u8 = (clamped_t * 255.0).round() as u8;

    let (a_r, a_g, a_b, a_a) = color_a.rgba();
    let (b_r, b_g, b_b, b_a) = color_b.rgba();

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
