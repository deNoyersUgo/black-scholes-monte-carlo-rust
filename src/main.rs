use rand::prelude::*;
use rand_dist::StandardNormal;
use rayon::prelude::*;
use stars::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;
use sdt::time::Instant;

pub enum OptionType {
    Call,
    Put
}
pub struct EuropeanOption {
    spot_price: f64,
    strike_price: f64,
    risk_free: f64,
    volatility: f64,
    maturity: f64,
    option_type: OptionType,
}

impl EuropeanOption {
    fn new(spot_price: f64, strike_price: f64, risk_free: f64, volatility: f64, maturity: f64, option_type: OptionType) -> Self {
        EuropeanOption { spot_price, strike_price, risk_free, volatility, maturity, option_type }
    }

    pub fn price_bs(&self) -> f64 {
        let n = Normal::new(0.0, 1.0).unwrap();

        let sigma_sqrt_t = self.volatility * self.maturity.sqrt();

        //d_+ computation
        let d_plus = ((self.spot_price / self.strike_price).ln()
            + (self.risk_free + 0.5 * self.volatility.powi(2)) * self.maturity)
            / sigma_sqrt_t;

        //d_- computation
        let d_minus = d_plus - sigma_sqrt_t;

        match self.option_type {
            OptionType::Call => {
                // Call = S * N(d_plus) - K * e^(-rT) * N(d_minus)
                self.spot_price * n.cdf(d_plus) - self.strike_price * E.powf(-self.risk_free * self.maturity) * n.cdf(d_minus)
            }
            OptionType::Put => {
                // Put = K * e^(-rT) * N(-d_minus) - S * N(-d_plus)
                self.strike_price * E.powf(-self.risk_free * self.maturity) * n.cdf(-d_minus) - self.spot_price * n.cdf(-d_plus)
            }
        }
    }

    pub fn price_mc(&self, num_sims: usize) -> f64 {
        let drift = (self.risk_free - 0.5 * self.volatility.powi(2)) * self.maturity;
        let diffusion = self.volatility * self.maturity.sqrt();
        let discount_factor = E.powf(-self.risk_free * self.maturity);

        let sum_payoffs: f64 = (0..num_sims).into_par_iter().map(|_| {
            let mut rng = rand::thread_rng();

            let z: f64 = rng.sample(StandardNormal);

            let s_t = self.spot_price * (drift * diffusion * z).exp();

            match self.option_type {
                OptionType::Call => {s_t - self.strike_price}.max(0.0),
                OptionType::Put => {self.strike_price - s_t}.max(0.0),
            }
        }).sum();

        (sum_payoffs / num_sims as f64) * discount_factor
    }
}

fn main() {
    println!("Hello, world!");
}
