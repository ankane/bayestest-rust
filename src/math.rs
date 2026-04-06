#[cfg(not(feature = "no_std"))]
pub fn exp(x: f64) -> f64 {
    x.exp()
}

#[cfg(feature = "no_std")]
pub use libm::exp;

#[cfg(not(feature = "no_std"))]
pub fn ln(x: f64) -> f64 {
    x.ln()
}

#[cfg(feature = "no_std")]
pub use libm::log as ln;

#[cfg(feature = "nightly")]
pub fn loggamma(x: f64) -> f64 {
    x.ln_gamma().0
}

#[cfg(not(feature = "nightly"))]
extern "C" {
    // use lgamma_r instead of lgamma for thread-safety
    pub fn lgamma_r(x: f64, signp: *mut i32) -> f64;
}

#[cfg(not(feature = "nightly"))]
pub fn loggamma(x: f64) -> f64 {
    let mut signp = 0;
    unsafe { lgamma_r(x, &mut signp) }
}

pub fn logbeta(a: f64, b: f64) -> f64 {
    loggamma(a) + loggamma(b) - loggamma(a + b)
}
