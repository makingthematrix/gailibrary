#[cfg(test)]
mod uset_tests {
    use utils::uset::*;

    use std::collections::HashSet;

    use quickcheck::TestResult;
    use spectral::prelude::*;

    #[test]
    fn set_from_and_to_vec() {
        let v = vec![0, 3, 8, 10];
        let s: USet = USet::from(&v);

        assert_that(&(s.len()))
            .named(&"USet length")
            .is_equal_to(&4);
        assert_that(&(s.capacity()))
            .named(&"USet capacity")
            .is_equal_to(&11);

        assert_that(&(s.contains(0))).is_true();
        assert_that(&(s.contains(3))).is_true();
        assert_that(&(s.contains(8))).is_true();
        assert_that(&(s.contains(10))).is_true();
        assert_that(&(s.contains(9))).is_false();

        let v2: Vec<usize> = s.into();

        assert_that!(&v2).is_equal_to(&v);
    }

    fn to_unique_sorted_vec(v: &Vec<usize>) -> Vec<usize> {
        let mut hs = HashSet::new();
        for x in v {
            hs.insert(*x);
        }

        let mut v2: Vec<usize> = hs.into_iter().collect();
        v2.sort();
        v2
    }

    fn vec_compare(va: &[usize], vb: &[usize]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
            va.iter()
                .zip(vb)
                .all(|(&a, &b)| a == b)
    }

    quickcheck! {
        fn from_and_to_vec(v: Vec<usize>) -> TestResult {
            let unique_v = to_unique_sorted_vec(&v);

            if v.len() != unique_v.len() {
                return TestResult::discard()
            }

            let result: Vec<usize> = USet::from(&unique_v).iter().collect();
            TestResult::from_bool(vec_compare(&unique_v, &result))
        }
    }

    #[test]
    fn should_substract() {
        let s1 = uset![0, 3, 8, 10];
        let s2 = uset![3, 8];
        let s5 = USet::new();

        let s3 = &s1 - &s2;

        assert_that(&(s3.len())).is_equal_to(&2);
        assert_that(&(s3.contains(0))).is_true();
        assert_that(&(s3.contains(10))).is_true();

        let s4 = &s1 - &s2;

        assert_that(&(s4.len())).is_equal_to(&2);
        assert_that(&(s4.contains(0))).is_true();
        assert_that(&(s4.contains(10))).is_true();

        assert_that!((&s1 - &s5)).is_equal_to(s1.clone());
        assert_that!((&s5 - &s5)).is_equal_to(USet::new());
    }

    #[test]
    fn should_compile() {
        let s4 = vec![0usize, 3, 8, 10];
        for _i in 1..10 {
            let _s5: USet = USet::from(&s4);
        }
    }

    #[test]
    fn should_be_equal() {
        let s1 = uset![0, 3, 8, 10];
        let s2 = uset![0, 3, 8, 10];
        assert_that(&s1).is_equal_to(&s2);
        assert_that(&(s1 == s2)).is_true();
    }

    #[test]
    fn should_find_min() {
        let s1 = uset![0, 3, 8, 10];
        assert_that(&(s1.iter().next())).is_equal_to(&Some(0));
        let s2 = uset![3, 8, 10];
        assert_that(&(s2.iter().next())).is_equal_to(&Some(3));
        let s3 = USet::new();
        let mut s3iter = s3.iter();
        assert_that(&(s3iter.next())).is_equal_to(&None);
        assert_that(&(s3iter.next())).is_equal_to(&None);

        let mut s2iter = s2.iter();
        assert_that!(s2iter.next()).is_equal_to(Some(3));
        assert_that!(s2iter.next()).is_equal_to(Some(8));
        assert_that!(s2iter.next()).is_equal_to(Some(10));
        assert_that!(s2iter.next()).is_equal_to(None);
        assert_that!(s2iter.next()).is_equal_to(None);

        let s4 = uset![0];
        let mut s4iter = s4.iter();
        assert_that!(s4iter.next()).is_equal_to(Some(0));
        assert_that!(s4iter.next()).is_equal_to(None);
        assert_that!(s4iter.next()).is_equal_to(None);

        // TODO: find min after adding a new element, smaller than the previous min

        // TODO: find min after removing the previous min
    }

    #[test]
    fn should_find_max() {
        let s1 = uset![0, 3, 8, 10];
        assert_that!(s1.iter().rev().next()).is_equal_to(Some(10));
        let s2 = uset![0];
        assert_that!(s2.iter().rev().next()).is_equal_to(Some(0));
        let s3 = USet::new();
        let mut s3iter = s3.iter().rev();
        assert_that!(s3iter.next()).is_equal_to(None);
        assert_that!(s3iter.next()).is_equal_to(None);

        let mut s2iter = s2.iter().rev();
        assert_that!(s2iter.next()).is_equal_to(Some(0));
        assert_that!(s2iter.next()).is_equal_to(None);
        assert_that!(s2iter.next()).is_equal_to(None);

        // TODO: find max after adding a new element, bigger than the previous max

        // TODO: find max after removing the previous max
    }

    #[test]
    fn should_add() {
        let s1 = uset![0, 3, 8, 10];
        let s2 = uset![1, 4];
        let s3 = uset![3, 5];
        let s4 = USet::new();

        assert_that!((&s1 + &s2)).is_equal_to(uset![0, 1, 3, 4, 8, 10]);
        assert_that!((&s1 + &s3)).is_equal_to(uset![0, 3, 5, 8, 10]);
        assert_that!((&s1 + &s4)).is_equal_to(s1.clone());
        assert_that!((&s1 + &s1)).is_equal_to(s1.clone());
        assert_that!((&s4 + &s4)).is_equal_to(s4.clone());
    }

    #[test]
    fn should_mul() {
        let s1 = uset![0, 3, 8, 10];
        let s2 = uset![3, 8];
        assert_that!((&s1 * &s2)).is_equal_to(uset![3, 8]);

        let s3 = uset![1, 2, 3];
        assert_that!((&s1 * &s3)).is_equal_to(uset![3]);

        let s4 = USet::new();
        assert_that!((&s1 * &s4)).is_equal_to(USet::new());

        assert_that!((&s1 * &s1)).is_equal_to(s1.clone());

        let s5 = uset![2, 4, 6];
        assert_that!((&s1 * &s5)).is_equal_to(USet::new());

        let s6 = uset![10];
        assert_that!((&s1 * &s6)).is_equal_to(s6.clone());
    }

    #[test]
    fn should_xor() {
        let s1 = uset![0, 3, 8, 10];

        let s2 = uset![3, 8];
        assert_that!((&s1 ^ &s2)).is_equal_to(uset![0, 10]);

        let s3 = uset![1, 2, 3];
        assert_that!((&s1 ^ &s3)).is_equal_to(uset![0, 1, 2, 8, 10]);

        let s4 = USet::new();
        assert_that!((&s1 ^ &s4)).is_equal_to(s1.clone());

        assert_that!((&s1 ^ &s1)).is_equal_to(USet::new());

        let s5 = uset![2, 4, 6];
        assert_that!((&s1 ^ &s5)).is_equal_to(uset![0, 2, 3, 4, 6, 8, 10]);

        let s6 = uset![10];
        assert_that!((&s1 ^ &s6)).is_equal_to(uset![0, 3, 8]);
    }
}
