use rand::distr::Alphanumeric;
use rand::{rng, Rng};

pub struct OtpHelper;

impl OtpHelper {
    pub fn generate_otp(length: usize) -> String {
        let mut rng = rng();
        (0..length)
            .map(|_| rng.random_range(0..10).to_string())
            .collect()
    }

    pub fn generate_reset_token() -> String {
        let mut rng = rng();
        (0..32).map(|_| rng.sample(Alphanumeric) as char).collect()
    }
}