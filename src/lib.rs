pub mod utils;
pub mod fft;
pub mod vst_utils;
pub mod effects;

/**
 *  Unit tests
 */
#[cfg(test)]
mod tests {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256Plus;

    use crate::utils::chaos;
    use crate::effects::delay;

    #[test]
    fn test_randf() {
        let mut rng = Xoshiro256Plus::seed_from_u64(42635680);
        for _i in 0..100000 {
            let num = chaos::randf(&mut rng);
            assert!(num >= 0.0 && num <= 1.0);
        }
    }

    #[test]
    fn test_delay_line_trunc_sum() {
        let mut delay_line = delay::DelayLine::new(10000.0, 2, delay::InterpMethod::Truncate, delay::MixMethod::Sum);
        assert!(delay_line.add_head(0.0, 1.0) == 0);
        assert!(delay_line.add_head(1000.0, 1.0) == 1);
        assert!(delay_line.read_write(1.0) == 0.0);
        assert!(delay_line.read_write(1.0) == 1.0);
        assert!(delay_line.read_write(0.0) == 1.0);
        assert!(delay_line.read_write(0.0) == 1.0);
    }

    #[test]
    fn test_delay_line_trunc_sqrt() {
        // TODO:
    }

    #[test]
    fn test_delay_line_round_sum() {
        // TODO:
    }

    #[test]
    fn test_delay_line_linear_sum() {
        // TODO:
    }
}