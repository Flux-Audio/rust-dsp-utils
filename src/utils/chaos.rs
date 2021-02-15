/**
 *  Utilities for generating random numbers and noise.
 *
 *  Passing the same instance of Xoshiro256Plus will correlate all the variables
 */

/// random float in [0, 1)
/// - rng: provide random number engine Xoshiro256Plus
pub fn randf(rng: &mut Xoshiro256Plus) -> f32 {
    return rng.next_u64() as f32 / u64::MAX as f32;
}

/// random u64 in [0, max)
/// - rng: provide random number engine Xoshiro256Plus
/// - max: maximum (exclusive) value
pub fn randu(rng: &mut Xoshiro256Plus, max: u64) -> u64 {
    return rng.next_u64()%max;
}

/// brown noise (unbounded)
/// - rng: provide random number engine Xoshiro256Plus
/// - prev_a: previous accumulator output of noise_brown
/// Returns:
/// - noise sample: the actual noise
/// - accumulator: variable to pass to the next call of this function. Keeps track
///         of state across calls (for integration).
pub fn noise_brown(rng: &mut Xoshiro256Plus, prev_a: f32) -> (f32, f32) {
    let white = randf(rng) - 0.5;
    let out_a = (prev_a + (0.01*white))/1.01;   // leaky integrator
    return (out_a*3.5, out_a);  // (roughly) compensate for gain
}

/// saturated brown noise (bounded)
/// - rng: provide random number engine Xoshiro256Plus
/// - prev_a: previous output of noise_brown (for integration)
pub fn noise_brown_sat(rng: &mut Xoshiro256Plus, prev_a: f32) -> f32 {
    let white = randf(rng) - 0.5;
    return (prev_a + white).tanh();
}

// /// bernoulli gate

// /// blue noise (differentiated white noise)

// /// geiger noise (random triggers)