use crate::math::{logbeta, loggamma};

struct CountVariant {
    events: u32,
    exposure: u32,
}

/// A test for count data.
pub struct CountTest {
    variants: Vec<CountVariant>,
}

impl CountTest {
    /// Creates a new test for count data.
    pub fn new() -> Self {
        Self {
            variants: Vec::with_capacity(3),
        }
    }

    /// Adds a new variant.
    pub fn add(&mut self, events: u32, exposure: u32) {
        assert!(self.variants.len() < 3);

        self.variants.push(CountVariant { events, exposure });
    }

    /// Returns the winning probability of each variant.
    pub fn probabilities(&self) -> Vec<f64> {
        match self.variants.len() {
            0 => vec![],
            1 => vec![1.0],
            2 => {
                let a = &self.variants[0];
                let b = &self.variants[1];

                let prob = prob_1_beats_2(
                    a.events,
                    a.exposure,
                    b.events,
                    b.exposure,
                );
                vec![prob, 1.0 - prob]
            }
            _ => {
                let mut probs = Vec::new();
                let mut total = 0.0;
                for i in 0..2 {
                    let a = &self.variants[i];
                    let b = &self.variants[(i + 1) % 3];
                    let c = &self.variants[(i + 2) % 3];

                    let prob = prob_1_beats_23(
                        a.events,
                        a.exposure,
                        b.events,
                        b.exposure,
                        c.events,
                        c.exposure,
                    );

                    probs.push(prob);
                    total += prob;
                }
                probs.push(1.0 - total);
                probs
            }
        }
    }
}

impl Default for CountTest {
    fn default() -> Self {
        Self::new()
    }
}

fn prob_1_beats_2(alpha_1: u32, beta_1: u32, alpha_2: u32, beta_2: u32) -> f64 {
    let mut total = 0.0;
    let log_b1 = (beta_1 as f64).ln();
    let a2_log_b2 = alpha_2 as f64 * (beta_2 as f64).ln();
    let log_b1_b2 = ((beta_1 + beta_2) as f64).ln();

    for k in 0..alpha_1 {
        total += (k as f64 * log_b1 +
            a2_log_b2 -
            (k + alpha_2) as f64 * log_b1_b2 -
            ((k + alpha_2) as f64).ln() -
            logbeta((k + 1) as f64, alpha_2 as f64)).exp();
    }

    total
}

fn prob_1_beats_23(alpha_1: u32, beta_1: u32, alpha_2: u32, beta_2: u32, alpha_3: u32, beta_3: u32) -> f64 {
    let mut total = 0.0;

    let log_b1_b2_b3 = ((beta_1 + beta_2 + beta_3) as f64).ln();
    let a1_log_b1 = alpha_1 as f64 * (beta_1 as f64).ln();
    let log_b2 = (beta_2 as f64).ln();
    let log_b3 = (beta_3 as f64).ln();
    let loggamma_a1 = loggamma(alpha_1 as f64);

    for k in 0..alpha_2 {
        let sum_k = a1_log_b1 + k as f64 * log_b2 - loggamma((k + 1) as f64);

        for l in 0..alpha_3 {
            total += (sum_k + l as f64 * log_b3
                - (k + l + alpha_1) as f64 * log_b1_b2_b3
                + loggamma((k + l + alpha_1) as f64) - loggamma((l + 1) as f64) - loggamma_a1).exp();
        }
    }

    1.0 - prob_1_beats_2(alpha_2, beta_2, alpha_1, beta_1)
        - prob_1_beats_2(alpha_3, beta_3, alpha_1, beta_1)
        + total
}

#[cfg(test)]
mod tests {
    use super::prob_1_beats_2;
    use super::prob_1_beats_23;
    use crate::CountTest;

    fn assert_approx(act: f64, exp: f64) {
        assert!((act - exp).abs() < 0.0000000001);
    }

    #[test]
    fn test_no_variants() {
        let test = CountTest::new();
        assert!(test.probabilities().is_empty());
    }

    #[test]
    fn test_one_variant() {
        let mut test = CountTest::new();
        test.add(2, 1);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 1);
        assert_eq!(probabilities, vec![1.0]);
    }

    #[test]
    fn test_two_variants() {
        let mut test = CountTest::new();
        test.add(55, 50);
        test.add(30, 30);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 2);
        assert_approx(probabilities[0], 0.6710529663661625);
        assert_approx(probabilities[1], 0.3289470336338596);
    }

    #[test]
    fn test_three_variants() {
        let mut test = CountTest::new();
        test.add(55, 50);
        test.add(30, 30);
        test.add(10, 10);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 3);
        assert_approx(probabilities[0], 0.4633365654508068);
        assert_approx(probabilities[1], 0.2306153779716283);
        assert_approx(probabilities[2], 0.3060480565775272);
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.variants.len() < 3")]
    fn test_four_variants() {
        let mut test = CountTest::new();
        for _ in 0..4 {
            test.add(2, 1);
        }
    }

    #[test]
    fn test_prob_1_beats_2() {
        assert_approx(prob_1_beats_2(1, 2, 3, 4), 0.29629629629629595);
        assert_approx(prob_1_beats_2(55, 50, 30, 30), 0.6710529663661625);
        assert_approx(prob_1_beats_2(50, 50, 35, 30), 0.24796547380927997);
    }

    #[test]
    fn test_prob_1_beats_23() {
        assert_approx(prob_1_beats_23(1, 2, 3, 4, 5, 6), 0.16901765046296247);
        assert_approx(prob_1_beats_23(1, 2, 3, 4, 5, 100), 0.2962330601144884);
        assert_approx(prob_1_beats_23(55, 50, 30, 30, 10, 10), 0.4633365654508068);
        assert_approx(prob_1_beats_23(50, 50, 35, 30, 13, 18), 0.23397153850438435);
    }
}
