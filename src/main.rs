pub struct BlackScholesModel {
    risk_free: f64 // risk-free rate
}

impl BlackScholesModel {
    pub fn new(risk_free: f64) -> Self{
        BlackScholesModel {risk_free}
    }
}

pub struct EuropeanOption {
    strike_price: f64,
    maturity: f64,
    is_call: bool,
}

impl EuropeanOption {
    fn new(strike_price: f64, maturity: f64, is_call: bool) -> Self {
        EuropeanOption {strike_price, maturity, is_call}
    }
}

pub struct MonteCarloEngine {
    num_paths: usize,
    num_steps: usize,
}

impl MonteCarloEngine {
    pub fn new(num_paths: usize, num_steps: usize) -> Self{
        MonteCarloEngine { num_paths, num_steps }
    }

    pub fn price_option(&self, model: &BlackScholesModel, option: &EuropeanOption, s0: f64) -> f64 {

    }
}


fn main() {
    println!("Hello, world!");
}
