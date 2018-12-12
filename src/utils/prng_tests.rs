#[cfg(test)]
mod prng_tests {
    use crate::utils::prng::*;
    use rand::Rng;
    use spectral::prelude::*;

    #[test]
    fn create_rng() {
        let mut r = PseudoNormalRng::new(1.0);
        let x: f64 = r.next_f64();
        assert_that!(x >= 0.0 && x <= 1.0).is_true();
    }

    #[test]
    fn is_normal_distribution() {
        let v = vec![0.1, 0.25, 0.33, 0.5, 0.66, 0.75, 0.9];
        let mut diff_sum = 0.0;
        for i in 0..(v.len() - 1) {
            diff_sum += v[i + 1] - v[i];
        }

        let vn: Vec<f64> = v.iter().map(|&x| pseudo_normalize(x, 1.0)).collect();

        let mut diff_sum_n = 0.0;
        for i in 0..(vn.len() - 1) {
            diff_sum_n += vn[i + 1] - vn[i];
        }

        assert_that!(diff_sum > diff_sum_n).is_true();
    }

    fn variance(a: f64) -> f64 {
        const SIZE: usize = 10;
        const SAMPLES: usize = 10000;

        let mut r = PseudoNormalRng::new(a);
        let mut results = vec![0usize; SIZE];
        for _i in 0..SAMPLES {
            let index = r.gen_range(0usize, SIZE);
            results[index] += 1;
        }

        results
            .iter()
            .enumerate()
            .map(|(i, &x)| ((((SIZE as f64 / 2.0) as f64 - i as f64) * x as f64) as f64).powi(2))
            .sum()
    }

    #[test]
    fn diminishing_variances() {
        let v025 = variance(0.25);
        let v10 = variance(1.0);
        let v25 = variance(2.5);
        let v50 = variance(5.0);
        assert_that!(v025 > v10);
        assert_that!(v10 > v25);
        assert_that!(v25 > v50);
        println!("{:?}, {:?}, {:?}, {:?}", v025, v10, v25, v50);
    }

    #[test]
    fn foo() {
        const SAMPLES: usize = 10000;
        let mut r = PseudoNormalRng::new(1.0);
        let mut rsum = 0.0;
        let mut asum = 0.0;
        let mut min = 1.0;
        let mut max = 0.0;
        for _i in 0..SAMPLES {
            let res: f64 = r.gen();
            asum += res;
            if res < min {
                min = res
            };
            if res > max {
                max = res
            };
            rsum += (0.5 - res).powi(2);
        }

        let avg = asum / (SAMPLES as f64);
        let dev = (rsum / (SAMPLES as f64)).sqrt();
        println!(
            "min:{:?}, max: {:?}, avg: {:?}, dev: {:?}",
            min, max, avg, dev
        );
    }
}
