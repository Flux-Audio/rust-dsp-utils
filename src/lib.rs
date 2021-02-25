pub mod utils;
pub mod fft;
pub mod vst;

/**
 *  Unit tests
 */
#[cfg(test)]
mod tests {
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256Plus;

    use crate::utils::chaos;

    #[test]
    fn test_randf() {
        let mut rng = Xoshiro256Plus::seed_from_u64(42635680);
        for _i in 0..100000 {
            let num = chaos::randf(&mut rng);
            assert!(num >= 0.0 && num <= 1.0);
        }
    }
}