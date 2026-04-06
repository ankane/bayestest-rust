#[cfg(feature = "no_std")]
pub use libm::{exp, lgamma as loggamma, log as ln};

#[cfg(not(feature = "no_std"))]
pub fn exp(x: f64) -> f64 {
    x.exp()
}

#[cfg(not(feature = "no_std"))]
pub fn ln(x: f64) -> f64 {
    x.ln()
}

#[cfg(all(feature = "nightly", not(feature = "no_std")))]
pub fn loggamma(x: f64) -> f64 {
    x.ln_gamma().0
}

#[cfg(all(not(feature = "nightly"), not(feature = "no_std")))]
extern "C" {
    // use lgamma_r instead of lgamma for thread-safety
    pub fn lgamma_r(x: f64, signp: *mut i32) -> f64;
}

#[cfg(all(not(feature = "nightly"), not(feature = "no_std")))]
pub fn loggamma(x: f64) -> f64 {
    let mut signp = 0;
    unsafe { lgamma_r(x, &mut signp) }
}

pub fn logbeta(a: f64, b: f64) -> f64 {
    loggamma(a) + loggamma(b) - loggamma(a + b)
}
