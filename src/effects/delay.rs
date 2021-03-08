use std::collections::VecDeque;

use crate::utils::math;

pub struct DelayLine {
    vector: VecDeque<f32>,
    num: usize,
    head_offsets: Vec<f32>,
    head_gains: Vec<f32>,
    interp_mode: InterpMethod,
    mix_mode: MixMethod,
}

pub enum InterpMethod {
    Truncate,
    NearestNeighbor,
    Linear,
}

pub enum MixMethod {
    Sum,
    Sqrt,
    Average,
}

impl DelayLine {
    /// create a new delay line
    /// # Parameters
    /// - size: size in milliseconds
    /// - sr: sample rate in hertz
    /// - interp: interpolation method
    pub fn new(size: f32, sr: u32, interp: InterpMethod, mix: MixMethod) -> Self {
        let num = (size/1000.0 * sr as f32) as usize + 1;
        Self {
            vector: VecDeque::from(vec![0.0; num]),
            num: num,
            head_offsets: Vec::new(),
            head_gains: Vec::new(),
            interp_mode: interp,
            mix_mode: mix,
        }
    }

    /// add a read head
    /// # Parameters
    /// - offset: distance in milliseconds (smaller or equal to delay line size)
    /// - gain: gain at which the delay line is played back
    /// # Returns
    /// - index of the head
    pub fn add_head(&mut self, offset: f32, gain: f32) -> usize {
        self.head_offsets.push(offset/1000.0);
        self.head_gains.push(gain);
        self.head_offsets.len() - 1
    }

    /// remove a read head
    /// # Parameters
    /// - index: index of head to remove
    /// # Returns
    /// - boolean representing wether the head existed in the first place.
    pub fn remove_head(&mut self, index: usize) -> bool {
        if index < self.head_offsets.len() {
            self.head_offsets.remove(index);
            self.head_gains.remove(index);
            true
        } else {
            false
        }
    }

    /// changes the offset of one of the heads.
    /// # Parameters
    /// - index: index of the head to be changed
    /// - offset: new offset for the head
    /// # Returns
    /// - boolean representing wether the chosen head exists.
    /// # Side-effects
    /// The vector of heads is shifted, thus all indexes greater than the one
    /// removed are shifted with it.
    /// TODO: perhaps heads should be stored in a dictionary instead.
    pub fn set_offset(&mut self, index: usize, offset: f32) -> bool {
        if index < self.head_offsets.len() {
            self.head_offsets[index] = offset;
            true
        } else {
            false
        }
    }

    /// write a new value into the delay line and read from all active read heads
    /// # Parameters
    /// - write: input to write
    /// # Returns
    /// - mixed outputs from active heads
    pub fn read_write(&mut self, write: f32) -> f32{
        // Step 1: read previous values from read heads
        let accumulator = self.head_offsets.iter()
            .zip(self.head_gains.iter())
            .map(|(a, b)| match self.interp_mode {
                    InterpMethod::Truncate => 
                        self.vector[*a as usize]*b,
                    InterpMethod::NearestNeighbor => 
                        self.vector[(*a).round() as usize]*b,
                    InterpMethod::Linear => {
                        let i = (*a).floor() as usize;
                        let x = *a - i as f32;
                        math::x_fade(self.vector[i], x, self.vector[i + 1])
                    },
                })
            .sum::<f32>() / match self.mix_mode {
                MixMethod::Sum => 1.0,
                MixMethod::Sqrt => (self.head_offsets.len() as f32).sqrt(),
                MixMethod::Average => self.head_offsets.len() as f32,
            };

        // Step 2: write new value and shift deque
        self.vector.push_front(write);
        self.vector.pop_back();

        return accumulator;
    }
}