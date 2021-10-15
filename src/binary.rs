use statrs::function::beta::ln_beta as logbeta;

struct BinaryVariant {
    participants: u32,
    conversions: u32
}

pub struct BinaryTest {
    variants: Vec<BinaryVariant>
}

impl BinaryTest {
    pub fn new() -> Self {
        Self {
            variants: Vec::with_capacity(4)
        }
    }

    pub fn add(&mut self, participants: u32, conversions: u32) {
        assert!(conversions <= participants);
        assert!(self.variants.len() < 4);

        self.variants.push(BinaryVariant { participants, conversions });
    }

    pub fn probabilities(&self) -> Vec<f64> {
        match self.variants.len() {
            0 => vec![],
            1 => vec![1.0],
            2 => {
                let b = &self.variants[0];
                let a = &self.variants[1];

                let prob = prob_b_beats_a(
                    1 + a.conversions,
                    1 + a.participants - a.conversions,
                    1 + b.conversions,
                    1 + b.participants - b.conversions
                );
                vec![prob, 1.0 - prob]
            },
            3 => {
                let mut probs = Vec::with_capacity(3);
                let mut total = 0.0;
                for i in 0..2 {
                    let c = &self.variants[i];
                    let b = &self.variants[(i + 1) % 3];
                    let a = &self.variants[(i + 2) % 3];

                    let prob = prob_c_beats_ab(
                        1 + a.conversions,
                        1 + a.participants - a.conversions,
                        1 + b.conversions,
                        1 + b.participants - b.conversions,
                        1 + c.conversions,
                        1 + c.participants - c.conversions
                    );

                    probs.push(prob);
                    total += prob;
                }
                probs.push(1.0 - total);
                probs
            },
            _ => {
                let mut probs = Vec::with_capacity(4);
                let mut total = 0.0;
                for i in 0..3 {
                    let d = &self.variants[i];
                    let c = &self.variants[(i + 1) % 4];
                    let b = &self.variants[(i + 2) % 4];
                    let a = &self.variants[(i + 3) % 4];

                    let prob = prob_d_beats_abc(
                        1 + a.conversions,
                        1 + a.participants - a.conversions,
                        1 + b.conversions,
                        1 + b.participants - b.conversions,
                        1 + c.conversions,
                        1 + c.participants - c.conversions,
                        1 + d.conversions,
                        1 + d.participants - d.conversions
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

fn prob_b_beats_a(alpha_a: u32, beta_a: u32, alpha_b: u32, beta_b: u32) -> f64 {
    let mut total = 0.0;
    let logbeta_aa_ba = logbeta(alpha_a as f64, beta_a as f64);
    let beta_ba = (beta_b + beta_a) as f64;

    for i in 0..alpha_b {
        total += (logbeta((alpha_a + i) as f64, beta_ba) - ((beta_b + i) as f64).ln() - logbeta((1 + i) as f64, beta_b as f64) - logbeta_aa_ba).exp();
    }

    return total;
}

fn prob_c_beats_ab(alpha_a: u32, beta_a: u32, alpha_b: u32, beta_b: u32, alpha_c: u32, beta_c: u32) -> f64 {
    let mut total = 0.0;

    let logbeta_ac_bc = logbeta(alpha_c as f64, beta_c as f64);

    let mut log_bb_j_logbeta_j_bb = Vec::with_capacity(alpha_b as usize);

    for j in 0..alpha_b {
        log_bb_j_logbeta_j_bb.push(((beta_b + j) as f64).ln() + logbeta((1 + j) as f64, beta_b as f64));
    }

    let abc = (beta_a + beta_b + beta_c) as f64;
    let mut logbeta_ac_i_j = Vec::with_capacity((alpha_a + alpha_b) as usize);

    for i in 0..alpha_a + alpha_b {
        logbeta_ac_i_j.push(logbeta((alpha_c + i) as f64, abc));
    }

    for i in 0..alpha_a {
        let sum_i = -((beta_a + i) as f64).ln() - logbeta((1 + i) as f64, beta_a as f64) - logbeta_ac_bc;

        for j in 0..alpha_b {
            total += (sum_i + logbeta_ac_i_j[(i + j) as usize] - log_bb_j_logbeta_j_bb[j as usize]).exp();
        }
    }

    return 1.0 - prob_b_beats_a(alpha_c, beta_c, alpha_a, beta_a) -
        prob_b_beats_a(alpha_c, beta_c, alpha_b, beta_b) + total;
}

fn prob_d_beats_abc(alpha_a: u32, beta_a: u32, alpha_b: u32, beta_b: u32, alpha_c: u32, beta_c: u32, alpha_d: u32, beta_d: u32) -> f64 {
    let mut total = 0.0;

    let logbeta_ad_bd = logbeta(alpha_d as f64, beta_d as f64);

    let mut log_bb_j_logbeta_j_bb = Vec::with_capacity(alpha_b as usize);
    for j in 0..alpha_b {
        log_bb_j_logbeta_j_bb.push(((beta_b + j) as f64).ln() + logbeta((1 + j) as f64, beta_b as f64));
    }

    let mut log_bc_k_logbeta_k_bc = Vec::with_capacity(alpha_c as usize);
    for k in 0..alpha_c {
        log_bc_k_logbeta_k_bc.push(((beta_c + k) as f64).ln() + logbeta((1 + k) as f64, beta_c as f64));
    }

    let abcd = (beta_a + beta_b + beta_c + beta_d) as f64;
    let mut logbeta_bd_i_j_k = Vec::with_capacity((alpha_a + alpha_b + alpha_c) as usize);

    for i in 0..alpha_a + alpha_b + alpha_c {
        logbeta_bd_i_j_k.push(logbeta((alpha_d + i) as f64, abcd));
    }

    for i in 0..alpha_a {
        let sum_i = -((beta_a + i) as f64).ln() - logbeta((1 + i) as f64, beta_a as f64) - logbeta_ad_bd;

        for j in 0..alpha_b {
            let sum_j = sum_i - log_bb_j_logbeta_j_bb[j as usize];

            for k in 0..alpha_c {
                total += (sum_j + logbeta_bd_i_j_k[(i + j + k) as usize] - log_bc_k_logbeta_k_bc[k as usize]).exp();
            }
        }
    }

    return 1.0 - prob_b_beats_a(alpha_a, beta_a, alpha_d, beta_d) -
        prob_b_beats_a(alpha_b, beta_b, alpha_d, beta_d) -
        prob_b_beats_a(alpha_c, beta_c, alpha_d, beta_d) +
        prob_c_beats_ab(alpha_a, beta_a, alpha_b, beta_b, alpha_d, beta_d) +
        prob_c_beats_ab(alpha_a, beta_a, alpha_c, beta_c, alpha_d, beta_d) +
        prob_c_beats_ab(alpha_b, beta_b, alpha_c, beta_c, alpha_d, beta_d) - total;
}

#[cfg(test)]
mod tests {
    use crate::BinaryTest;
    use super::prob_b_beats_a;
    use super::prob_c_beats_ab;
    use super::prob_d_beats_abc;

    fn assert_approx(act: f64, exp: f64) {
        assert!((act - exp).abs() < 0.0000000001);
    }

    #[test]
    fn test_no_variants() {
        let test = BinaryTest::new();
        assert!(test.probabilities().is_empty());
    }

    #[test]
    fn test_one_variant() {
        let mut test = BinaryTest::new();
        test.add(2, 1);
        assert_eq!(test.probabilities(), vec![1.0]);
    }

    #[test]
    fn test_two_variants() {
        let mut test = BinaryTest::new();
        test.add(200, 100);
        test.add(400, 250);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 2);
        assert_approx(probabilities[0], 0.001756431311879969);
        assert_approx(probabilities[1], 0.99824356868812);
    }

    #[test]
    fn test_three_variants() {
        let mut test = BinaryTest::new();
        test.add(61, 15);
        test.add(54, 13);
        test.add(72, 19);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 3);
        assert_approx(probabilities[0], 0.29632930651329037);
        assert_approx(probabilities[1], 0.277257277195332);
        assert_approx(probabilities[2], 0.42641341629137763);
    }

    #[test]
    fn test_four_variants() {
        let mut test = BinaryTest::new();
        test.add(55, 50);
        test.add(30, 30);
        test.add(10, 10);
        test.add(50, 45);
        let probabilities = test.probabilities();
        assert_eq!(probabilities.len(), 4);
        assert_approx(probabilities[0], 0.02692341639320739);
        assert_approx(probabilities[1], 0.7040521621641954);
        assert_approx(probabilities[2], 0.249824960767943);
        assert_approx(probabilities[3], 0.019199460674668434);
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.variants.len() < 4")]
    fn test_five_variants() {
        let mut test = BinaryTest::new();
        for _ in 0..5 {
            test.add(2, 1);
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: conversions <= participants")]
    fn test_too_many_conversions() {
        let mut test = BinaryTest::new();
        test.add(1, 2);
    }

    #[test]
    fn test_prob_b_beats_a() {
        assert_approx(prob_b_beats_a(1, 2, 3, 4), 0.6428571428571429);
        assert_approx(prob_b_beats_a(55, 50, 30, 30), 0.38386463776317903);
        assert_approx(prob_b_beats_a(50, 50, 35, 30), 0.6867997222295887);
    }

    #[test]
    fn test_prob_c_beats_ab() {
        assert_approx(prob_c_beats_ab(1, 2, 3, 4, 5, 6), 0.4128959276018096);
        assert_approx(prob_c_beats_ab(1, 2, 3, 4, 5, 100), 0.0004147782900546515);
        assert_approx(prob_c_beats_ab(55, 50, 30, 30, 10, 10), 0.35421204276865736);
        assert_approx(prob_c_beats_ab(50, 50, 35, 30, 13, 18), 0.09139305107602524);
    }

    #[test]
    fn test_prob_d_beats_abc() {
        assert_approx(prob_d_beats_abc(1, 2, 3, 4, 5, 6, 7, 8), 0.2853316096371013);
        assert_approx(prob_d_beats_abc(55, 50, 30, 30, 10, 10, 25, 30), 0.08421499131901738);
    }
}
