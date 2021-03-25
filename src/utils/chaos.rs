/**
*  Utilities for generating random numbers and noise.
*
*  Passing the same instance of Xoshiro256Plus will correlate all the variables
*/

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

use std::f32::consts;

/// Provider of randomness. It keeps the state of all random number generators.
///
/// ***Warning:** all generators within the same instance of rng are correlated to
/// each other. If you need uncorrelated noise, you should create separate
/// instances.*
pub struct Rng {
    rng_core: Xoshiro256Plus,
    flutter_cores: Vec<f32>,
    dropout_cores: Vec<f32>,
    brown_accum: f32,
    white_prev: f32,
    sr: u32,
    rotor_1: f32,
    rotor_2: f32,
    rotor_3: f32,
    rotor_4: f32,
    rotor_5: f32,
    rotor_6: f32,
}

impl Rng {
    pub fn new(seed: u64, sr: u32) -> Self {
        let mut ret = Self {
            rng_core: Xoshiro256Plus::seed_from_u64(seed),
            flutter_cores: Vec::new(),
            dropout_cores: Vec::new(),
            brown_accum: 0.0,
            white_prev: 0.0,
            sr: sr,
            rotor_1: 0.0,
            rotor_2: 0.0,
            rotor_3: 0.0,
            rotor_4: 0.0,
            rotor_5: 0.0,
            rotor_6: 0.0,
        };

        // some generators require some fixed small random real number.
        // It is picked at initialization from this list.
        // Why this list? Because these are all approximations of irrational
        // numbers, so they produce very long periods.
        let mut cores = vec![1.5811388300841898, 1.0458250331675945, 
                             1.0716517624676405, 1.2716868718359877, 
                             1.3506364240608943, 1.7940875118009154, 
                             1.1301963325015703, 1.0807260291119114, 
                             1.3228756555322954, 1.2732217992164600, 
                             1.1473474844178637, 1.3360973954019968];
        
        // pick three cores
        ret.flutter_cores.push(cores.remove((ret.rng_core.next_u32()%12) as usize));
        ret.flutter_cores.push(cores.remove((ret.rng_core.next_u32()%11) as usize));
        ret.flutter_cores.push(cores.remove((ret.rng_core.next_u32()%10) as usize));

        // pick three cores
        ret.dropout_cores.push(cores.remove((ret.rng_core.next_u32()%9) as usize));
        ret.dropout_cores.push(cores.remove((ret.rng_core.next_u32()%8) as usize));
        ret.dropout_cores.push(cores.remove((ret.rng_core.next_u32()%7) as usize));

        return ret;
    }

    /// random float in [0, 1)
    pub fn randf(&mut self) -> f32 {
        return self.rng_core.next_u64() as f32 / u64::MAX as f32;
    }

    /// random u64 in [0, max)
    pub fn randu(&mut self, max: u64) -> u64 {
        return self.rng_core.next_u64()%max;
    }

    /// brown noise
    pub fn noise_brown(&mut self) -> f32 {
        let white = (self.randf() - 0.5) * 0.1;
        self.brown_accum = self.brown_accum*0.98 + white;   // leaky integrator
        return self.brown_accum;
    }

    /// violet noise
    pub fn noise_violet(&mut self) -> f32 {
        let white = self.randf() - 0.5;
        let violet = white - self.white_prev;
        self.white_prev = white;
        return violet;
    }

    /// pink noise
    pub fn noise_pink(&mut self) -> f32 {
        return self.randf() - 0.5 + self.noise_brown()*0.5;
    }

    /// blue noise
    pub fn blue_noise(&mut self) -> f32 {
        return self.randf() - 0.5 + self.noise_violet()*0.5;
    }

    /// random choice with probability
    /// # Parameters
    /// - `p`: probability that the function returns true 
    pub fn noise_coin(&mut self, p: f32) -> bool {
        return self.randf() < p;
    }

    // TODO: replace with filtered noise from HYSTERESIS
    /// flutter noise.
    ///
    /// use this for tape flutter.
    /// ***warning:** only call once per sample*
    pub fn flutter_noise(&mut self) -> f32 {
        self.rotor_1 += 12.0*self.flutter_cores[0]/self.sr as f32;
        self.rotor_2 += 12.0*self.flutter_cores[1]/self.sr as f32;
        self.rotor_3 += 12.0*self.flutter_cores[2]/self.sr as f32;
        if self.rotor_1 > consts::TAU {
            self.rotor_1 -= consts::TAU;
        }
        if self.rotor_2 > consts::TAU {
            self.rotor_2 -= consts::TAU;
        }
        if self.rotor_3 > consts::TAU {
            self.rotor_3 -= consts::TAU;
        }
        let ret = (self.rotor_1.sin() + self.rotor_2.sin() + self.rotor_3.sin())/3.0; 
        return ret.abs().powf(6.0) * ret.signum();
    }

    // TODO: replace with filtered noise from HYSTERESIS
    /// dropout noise.
    ///
    /// use this for cassette dropoff.
    /// ***warning:** only call once per sample*
    pub fn dropoff_noise(&mut self) -> f32 {
        self.rotor_4 += 12.0*self.dropout_cores[0]/self.sr as f32;
        self.rotor_5 += 12.0*self.dropout_cores[1]/self.sr as f32;
        self.rotor_6 += 12.0*self.dropout_cores[2]/self.sr as f32;
        if self.rotor_4 > consts::TAU {
            self.rotor_4 -= consts::TAU;
        }
        if self.rotor_5 > consts::TAU {
            self.rotor_5 -= consts::TAU;
        }
        if self.rotor_6 > consts::TAU {
            self.rotor_6 -= consts::TAU;
        }
        let ret = (self.rotor_4.sin() + self.rotor_5.sin() + self.rotor_6.sin())/3.0; 
        return ret.abs().powf(6.0);
    }

    /// geiger noise (random triggers)
    /// 
    pub fn geiger_noise(&mut self, rate: f32, random: f32) -> f32 {
        0.0
    }
}


