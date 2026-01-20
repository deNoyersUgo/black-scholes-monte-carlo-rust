use rand::prelude::*;
use rand_distr::StandardNormal;
use rayon::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};
use std::time::Instant;

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
                self.spot_price * n.cdf(d_plus) - self.strike_price * (-self.risk_free * self.maturity).exp() * n.cdf(d_minus)
            }
            OptionType::Put => {
                // Put = K * e^(-rT) * N(-d_minus) - S * N(-d_plus)
                self.strike_price * (-self.risk_free * self.maturity).exp() * n.cdf(-d_minus) - self.spot_price * n.cdf(-d_plus)
            }
        }
    }

    pub fn price_mc(&self, num_sims: usize) -> f64 {
        let drift = (self.risk_free - 0.5 * self.volatility.powi(2)) * self.maturity;
        let diffusion = self.volatility * self.maturity.sqrt();
        let discount_factor = (-self.risk_free * self.maturity).exp();

        let sum_payoffs: f64 = (0..num_sims).into_par_iter().map(|_| {
            let mut rng = rand::rng();

            let z: f64 = rng.sample(StandardNormal);

            let s_t = self.spot_price * (drift + diffusion * z).exp();

            match self.option_type {
                OptionType::Call => {s_t - self.strike_price}.max(0.0),
                OptionType::Put => {self.strike_price - s_t}.max(0.0),
            }
        }).sum();

        (sum_payoffs / num_sims as f64) * discount_factor
    }
}

fn main() {
    // 1. Initialize Market Parameters
    let spot_price = 100.0;
    let strike_price = 100.0;
    let risk_free_rate = 0.05; // 5%
    let volatility = 0.2;      // 20%
    let time_to_maturity = 1.0; // 1 year
    let num_sims = 100_000_000;

    let call_option = EuropeanOption::new(
        spot_price,
        strike_price,
        risk_free_rate,
        volatility,
        time_to_maturity,
        OptionType::Call,
    );

    println!("--- Pricing Engine Initialized ---");
    println!("Parameters: S={}, K={}, r={}, σ={}, T={}",
             spot_price, strike_price, risk_free_rate, volatility, time_to_maturity);
    println!("Simulations: {}", num_sims);
    println!("----------------------------------");

    // 2. Run Black-Scholes
    let start_bs = Instant::now();
    let bs_price = call_option.price_bs();
    let duration_bs = start_bs.elapsed();

    println!("Black-Scholes Price: {:.6} (Time: {:?})", bs_price, duration_bs);

    // 3. Run Monte Carlo (Parallel)
    // Note: On your Arch Linux laptop, run `htop` while this executes to see cores spike.
    let start_mc = Instant::now();
    let mc_price = call_option.price_mc(num_sims);
    let duration_mc = start_mc.elapsed();

    println!("Monte Carlo Price:   {:.6} (Time: {:?})", mc_price, duration_mc);

    // 4. Analysis
    let diff = (bs_price - mc_price).abs();
    println!("----------------------------------");
    println!("Difference: {:.6}", diff);

    // Convergence check (Standard Error ~ 1/sqrt(N))
    if diff < 0.01 {
        println!("Result: CONVERGED within acceptable tolerance.");
    } else {
        println!("Result: DIVERGENCE detected. Increase N.");
    }
}
