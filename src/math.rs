#[cfg(not(feature = "libm"))]
extern "C" {
    // use lgamma_r instead of lgamma for thread-safety
    pub fn lgamma_r(x: f64, signp: *mut i32) -> f64;
}

#[cfg(not(feature = "libm"))]
pub fn loggamma(x: f64) -> f64 {
    let mut signp = 0;
    unsafe { lgamma_r(x, &mut signp) }
}

#[cfg(feature = "libm")]
pub use libm::lgamma as loggamma;

pub fn logbeta(a: f64, b: f64) -> f64 {
    loggamma(a) + loggamma(b) - loggamma(a + b)
}
