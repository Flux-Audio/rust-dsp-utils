/// 2-pole state variable filter. Implements lowpass, highpass, notch and
/// bandpass filters with shared state.
struct Svf {
    lp_z1:    f32,
    hp_z1:    f32,
    notch_z1: f32,
    bp_z1:    f32,
}

impl Svf {
    /// Initialize filter state variables.
    pub fn new() -> Self {
        Self {
            lp_z1:    0.0,
            hp_z1:    0.0,
            notch_z1: 0.0,
            bp_z1:    0.0,
        }
    }

    /// Compute lowpass, highpass, notch and bandpass filtering of input with
    /// variable resonance and cutoff.
    pub fn filter(&mut self, input: f32, cutoff: f32, res: f32, sr: f32) -> (f32, f32, f32, f32) {
        // Pre-process
        let f = 2.0 * (std::f32::consts::PI*cutoff / sr).sin();
        let q = (1.0 - res)*2.0;

        // Filtering
        let lp    = self.bp_z1*f + self.lp_z1;
        let hp    = input - lp - q*self.bp_z1;
        let notch = hp + lp;
        let bp    = hp*f + self.bp_z1;

        // Update state:
        self.lp_z1    = lp;
        self.hp_z1    = hp;
        self.notch_z1 = notch;
        self.bp_z1    = bp;

        // return 
        return (lp, hp, notch, bp);
    }
}

/// DC offset blocking filter.
struct BlockDC {
    x_z1: f32,
    y_z1: f32,
}

impl BlockDC {
    /// Initialize filter state variables.
    pub fn new() -> Self {
        Self {
            x_z1: 0.0,
            y_z1: 0.0,
        }
    }

    /// Weak DC blocker. Use for blocking constant DC in input.
    pub fn filter_weak(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.995*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }

    /// Medium DC blocker. Use for blocking sub-sonic sound in input.
    pub fn filter_medium(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.9*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }

    /// Strong DC blocker. Use for blocking DC in feedback loops, i.e. for stabilizing
    /// an unstable feedback loop. For this application you might want to combine
    /// it with a highpass filter at approximately 18kHz.
    /// 
    /// Note also that this filter will remove some of the sub bass, so use only
    /// when strictly necessary.
    pub fn filter_strong(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.5*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }
}