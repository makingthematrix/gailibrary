#[cfg(test)]
mod umap_tests {
    use crate::utils::umap::*;

    // use quickcheck::TestResult;
    use spectral::prelude::*;

    #[test]
    fn basic_operations() {
        let mut map = UMap::new() as UMap<bool>;
        assert_that!(map.is_empty()).is_true();
        map.put(5, true);
        assert_that!(map.is_empty()).is_false();
        assert_that!(map.len()).is_equal_to(1);
        assert_that!(map.contains(5)).is_true();
        assert_that!(map.contains(4)).is_false();
        map.put(2, false);
        assert_that!(map.len()).is_equal_to(2);
        assert_that!(map.get(2)).is_equal_to(Some(false));
        let re1 = map.remove(5);
        assert_that!(re1).is_equal_to(Some(true));
        assert_that!(map.len()).is_equal_to(1);
        let re2 = map.remove(1);
        assert_that!(re2).is_equal_to(None);
        map.remove(2);
        assert_that!(map.is_empty()).is_true();

        assert_that!(map.get(12)).is_equal_to(None);
    }

    #[test]
    fn basic_iterator() {
        let vec = vec![None, None, Some(2), None, Some(4), Some(5)];
        let mut iter = vec.iter().enumerate().filter_map(|(i, &v)| {
            if v.is_some() {
                Some((i, v.unwrap()))
            } else {
                None
            }
        });
        assert_that!(iter.next()).is_equal_to(Some((2, 2)));
        assert_that!(iter.next()).is_equal_to(Some((4, 4)));
        assert_that!(iter.next()).is_equal_to(Some((5, 5)));
        assert_that!(iter.next()).is_equal_to(None);
    }

    #[test]
    fn better_iterator() {
        let mut map = UMap::new();
        map.put(2, 2);
        map.put(4, 4);
        map.put(5, 5);
        /* TODO: I need an awesome macro that would let me write it like:
            ```
            let map = umap!(2 -> 2, 4 -> 4, 5 -> 5)
            ```
        */

        let mut iter = map.iter();

        assert_that!(iter.next()).is_equal_to(Some((2, &2)));
        assert_that!(iter.next()).is_equal_to(Some((4, &4)));
        assert_that!(iter.next()).is_equal_to(Some((5, &5)));
        assert_that!(iter.next()).is_equal_to(None);
    }

    #[test]
    fn min_max() {
        let map: UMap<&str> = vec![(2, "a"), (4, "b"), (5, "c")].into();

        assert_that!(map.min()).is_equal_to(Some(2));
        assert_that!(map.max()).is_equal_to(Some(5));
    }

    #[test]
    fn min_max_when_empty() {
        let map: UMap<&str> = UMap::new();

        assert_that!(map.min()).is_equal_to(None);
        assert_that!(map.max()).is_equal_to(None);
    }

    #[test]
    fn join_maps() {
        let map1: UMap<i32> = vec![(2, 2), (4, 4), (5, 5)].into();
        let mut iter1 = map1.iter();
        assert_that!(iter1.next()).is_equal_to(Some((2, &2)));
        assert_that!(iter1.next()).is_equal_to(Some((4, &4)));
        assert_that!(iter1.next()).is_equal_to(Some((5, &5)));
        assert_that!(iter1.next()).is_equal_to(None);

        let map2: UMap<i32> = vec![(1, 1), (3, 3), (5, 5), (8, 8)].into();
        let mut iter2 = map2.iter();
        assert_that!(iter2.next()).is_equal_to(Some((1, &1)));
        assert_that!(iter2.next()).is_equal_to(Some((3, &3)));
        assert_that!(iter2.next()).is_equal_to(Some((5, &5)));
        assert_that!(iter2.next()).is_equal_to(Some((8, &8)));
        assert_that!(iter2.next()).is_equal_to(None);

        let map3 = &map1 + &map2;
        assert_that!(map3.len()).is_equal_to(6);
        let mut iter3 = map3.iter();
        assert_that!(iter3.next()).is_equal_to(Some((1, &1)));
        assert_that!(iter3.next()).is_equal_to(Some((2, &2)));
        assert_that!(iter3.next()).is_equal_to(Some((3, &3)));
        assert_that!(iter3.next()).is_equal_to(Some((4, &4)));
        assert_that!(iter3.next()).is_equal_to(Some((5, &5)));
        assert_that!(iter3.next()).is_equal_to(Some((8, &8)));
        assert_that!(iter3.next()).is_equal_to(None);
    }

    #[test]
    fn common_part_of_maps() {
        let map1: UMap<i32> = vec![(2, 2), (4, 4), (5, 5)].into();
        let map2: UMap<i32> = vec![(1, 1), (3, 3), (5, 5), (8, 8)].into();

        let map3 = &map1 * &map2;
        assert_that!(map3.len()).is_equal_to(1);
        let mut iter3 = map3.iter();
        assert_that!(iter3.next()).is_equal_to(Some((5, &5)));
        assert_that!(iter3.next()).is_equal_to(None);
    }

    #[test]
    fn substract_maps() {
        let map1: UMap<i32> = vec![(2, 2), (4, 4), (5, 5)].into();
        let map2: UMap<i32> = vec![(1, 1), (3, 3), (5, 5), (8, 8)].into();

        let map3 = &map1 - &map2;
        assert_that!(map3.len()).is_equal_to(2);
        let mut iter3 = map3.iter();
        assert_that!(iter3.next()).is_equal_to(Some((2, &2)));
        assert_that!(iter3.next()).is_equal_to(Some((4, &4)));
        assert_that!(iter3.next()).is_equal_to(None);
    }

    #[test]
    fn xor_maps() {
        let map1: UMap<i32> = vec![(2, 2), (4, 4), (5, 5)].into();
        let map2: UMap<i32> = vec![(1, 1), (3, 3), (5, 5), (8, 8)].into();

        let map3 = &map1 ^ &map2;
        assert_that!(map3.len()).is_equal_to(5);
        let mut iter3 = map3.iter();
        assert_that!(iter3.next()).is_equal_to(Some((1, &1)));
        assert_that!(iter3.next()).is_equal_to(Some((2, &2)));
        assert_that!(iter3.next()).is_equal_to(Some((3, &3)));
        assert_that!(iter3.next()).is_equal_to(Some((4, &4)));
        assert_that!(iter3.next()).is_equal_to(Some((8, &8)));
        assert_that!(iter3.next()).is_equal_to(None);
        assert_that!(iter3.next()).is_equal_to(None);
    }
}
