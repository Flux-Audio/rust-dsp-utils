use std::f32::consts;
pub struct RampCore{
    init_phase: f32,
    phase: f32,
    rad_per_sec: f32,
    sr: f32,
}

impl RampCore {
    pub fn new(init_phase: f32, freq: f32, sr: f32) -> Self {
        Self {
            init_phase:  init_phase.rem_euclid(consts::TAU),
            phase:       init_phase.rem_euclid(consts::TAU),
            rad_per_sec: freq*consts::TAU,
            sr:          sr,
        }
    }

    pub fn step(&mut self) -> f32 {
        let ret = self.phase;
        self.phase += self.rad_per_sec/self.sr;
        return ret;
    }

    pub fn reset(&mut self) {
        self.phase = self.init_phase;
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.rad_per_sec = freq*consts::TAU;
    }
}