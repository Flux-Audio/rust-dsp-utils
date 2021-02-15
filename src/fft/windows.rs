use rustfft::num_complex::Complex;

use std::f32::consts;


/// hann window function
/// - x: input
/// - i: index
/// - l_div: reciprocal of window length
pub fn win_hann(x: Complex<f32>, i: usize, l_div: f32) -> Complex<f32> {
    let sin_i = (i as f32 * std::f32::consts::PI * l_div).sin();
    return x * sin_i * sin_i;
}

/// triangular window function
/// - x: input
/// - i: index
/// - l_div: reciprocal of window length
pub fn win_tri(x: Complex<f32>, i: usize, l_div: f32) -> Complex<f32> {
    let tri_i = 1.0 - (2.0 * i as f32 * l_div - 1.0).abs();
    return x * tri_i;
}

/// blackman window function
/// - x: input
/// - i: index
/// - l_div: reciprocal of window length
pub fn win_black(x: Complex<f32>, i: usize, l_div: f32) -> Complex<f32> {
    let a_0 = 0.426_59;
    let a_1 = 0.496_56;
    let a_2 = 0.076_849;
    let win = a_0 - a_1 * (consts::TAU * i as f32 * l_div).cos()
        + a_2 * (2.0 * consts::TAU * i as f32 * l_div).cos();
    return x * win;
}

/// nuttal window function
/// - x: input
/// - i: index
/// - l_div: reciprocal of window length
pub fn win_nutt(x: Complex<f32>, i: usize, l_div: f32) -> Complex<f32> {
    let a_0 = 0.355_768;
    let a_1 = 0.487_396;
    let a_2 = 0.144_232;
    let a_3 = 0.012_604;
    let win = a_0 - a_1 * (consts::TAU * i as f32 * l_div).cos()
        + a_2 * (2.0 * consts::TAU * i as f32 * l_div).cos()
        - a_3 * (3.0 * consts::TAU * i as f32 * l_div).cos();
    return x * win;
}

/// flat top window function
/// - x: input
/// - i: index
/// - l_div: reciprocal of window length
pub fn win_flat(x: Complex<f32>, i: usize, l_div: f32) -> Complex<f32> {
    let a_0 = 0.215_578_94;
    let a_1 = 0.416_631_58;
    let a_2 = 0.277_263_16;
    let a_3 = 0.083_578_944;
    let a_4 = 0.006_947_368;
    let win = a_0 - a_1 * (consts::TAU * i as f32 * l_div).cos()
        + a_2 * (2.0 * consts::TAU * i as f32 * l_div).cos()
        - a_3 * (3.0 * consts::TAU * i as f32 * l_div).cos()
        + a_4 * (4.0 * consts::TAU * i as f32 * l_div).cos();
    return x * win;
}