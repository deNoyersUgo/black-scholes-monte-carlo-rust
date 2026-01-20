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
        let dplus = ((self.spot_price / self.strike_price).ln()
            + (self.risk_free + 0.5 * self.volatility.powi(2)) * self.maturity)
            / sigma_sqrt_t;

        //d_- computation
        let dminus = dplus - sigma_sqrt_t;

        match self.option_type {
            OptionType::Call => {
                // Call = S * N(dplus) - K * e^(-rT) * N(dminus)
                self.spot_price * n.cdf(dplus) - self.strike_price * E.powf(-self.risk_free * self.maturity) * n.cdf(dminus)
            }
            OptionType::Put => {
                // Put = K * e^(-rT) * N(-dminus) - S * N(-dplus)
                self.strike_price * E.powf(-self.risk_free * self.maturity) * n.cdf(-dminus) - self.spot_price * n.cdf(-dplus)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
