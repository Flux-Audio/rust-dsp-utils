/// maps x such that: x in [a, b] -> y in [c, d]
pub fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
    return (y_max - y_min) / (x_max - x_min) * (x - x_min) + y_min;
}

/// maps x such that: x in [a, b] -> y in [0, 1]
pub fn map_normal(x: f32, x_min: f32, mut x_max: f32) -> f32 {
    x_max = x_max*0.999_999_9 + 0.000_000_1;  // prevent division by zero
    // precondition: x in [a, b]
    // debug_assert!(a <= x);
    // debug_assert!(x <= b);
    return 1.0/(x_max - x_min)*(x - x_min);
}

/// Crossfade between two values, i.e. linear interpolation.
/// The crossfading parameter is clamped between 0 and 1.
/// This function is inlined for hot use inside of interpolation algorithms.
#[inline(always)]
pub fn x_fade(a: f32, x: f32, b: f32) -> f32 {
    let x_clamp = x.clamp(0.0, 1.0);
    a*(1.0 - x_clamp) + b*x_clamp
}

/// Gives two coefficients for pre/post-gain with equal total gain.
/// # Examples
/// ```rust
/// use rust_dsp_utils::utils::math::pre_post_gains;
/// let drive = 0.5;    // positive values mean pre gain and post cut
/// let (pre, post) = pre_post_gains(drive);
/// assert!(pre > 1.0);
/// assert!(post < 1.0);
/// assert!(2.0*pre*post == 2.0);
/// ```
pub fn pre_post_gains(x: f32) -> (f32, f32) {
    if x < 0.0 {
        (1.0 / (1.0 - x), 1.0 - x)
    } else {
        (1.0 + x, 1.0 / (1.0 + x))
    }
}