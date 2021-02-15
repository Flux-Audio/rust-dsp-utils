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