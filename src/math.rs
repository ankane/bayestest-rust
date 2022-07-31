extern "C" {
    pub fn lgamma_r(x: f64, signp: *mut i32) -> f64;
}

pub fn loggamma(x: f64) -> f64 {
    let mut signp = 0;
    unsafe { lgamma_r(x, &mut signp) }
}

pub fn logbeta(a: f64, b: f64) -> f64 {
    loggamma(a) + loggamma(b) - loggamma(a + b)
}
