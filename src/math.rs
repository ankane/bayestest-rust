use libm::lgamma_r;

pub fn loggamma(x: f64) -> f64 {
    lgamma_r(x).0
}

pub fn logbeta(a: f64, b: f64) -> f64 {
    loggamma(a) + loggamma(b) - loggamma(a + b)
}
