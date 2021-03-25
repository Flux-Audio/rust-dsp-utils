/// Applies mu-law companding to a signal
/// # Parameters
/// + `x`: input signal
/// + `amount`: amount of companding
pub fn mu_law(x: f32, amount: f32) -> f32 {
    let mu = 1.0 / (1.0 - amount.clamp(0.0, 0.9999));
    x.signum() * (x.abs()*mu + 1.0).ln() / (mu + 1.0).ln()
}

/// Applies inverse mu-law companding to a signal
/// # Parameters
/// + `x`: input signal
/// + `amount`: amount of companding
pub fn inv_mu_law(x: f32, amount: f32) -> f32 {
    let mu = 1.0 / (1.0 - amount.clamp(0.0, 0.9999));
    x.signum() / mu * ((1.0 + mu).powf(x.abs()) - 1.0)
}

/// Variable hardness clipping function.
///
/// Clamps signal between -1.0 and 1.0, with hard or soft clipping, according to
/// `hardness` parameter.
/// # Parameters
/// + `x`: input signal
/// + `hardness`: hardness of the clipping.
pub fn var_clip(x: f32, hardness: f32) -> f32 {
    let k = 1.0 - hardness.clamp(0.0, 0.9999);
    x.abs() / (x.abs().powf(1.0 / k) + 0.1).powf(k) * x.signum()
}