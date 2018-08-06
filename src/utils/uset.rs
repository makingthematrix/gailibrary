#![macro_use]

use std::cmp::{max, min};
use std::ops::Range;

use std::ops::{Add, BitXor, Mul, Sub};

// TODO: https://doc.rust-lang.org/src/alloc/vec_deque.rs.html#1909-1913
// rewrite in a similar fashion

#[allow(unused_macros)]
macro_rules! uset {
    ($($x:expr),*) => (USet::from_vec(&vec![$($x),*]))
}

#[derive(Debug, Default, Clone)]
pub struct USet {
    vec: Vec<bool>,
    len: usize,
}

pub struct USetIter<'a> {
    handle: &'a USet,
    index: usize,
    rindex: usize,
}

impl<'a> Iterator for USetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.handle.vec.len() - self.rindex;
        while self.index < max {
            let index = self.index;
            self.index += 1;
            if self.handle.vec[index] {
                return Some(index);
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for USetIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.handle.vec.len();
        while self.rindex < len - self.index {
            let index = len - self.rindex - 1;
            self.rindex += 1;
            if self.handle.vec[index] {
                return Some(index);
            }
        }
        None
    }
}

impl USet {
    #[inline]
    pub fn new() -> Self {
        USet::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        USet {
            vec: vec![false; size],
            len: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.len()
    }

    pub fn add(&mut self, el: usize) {
        if el >= self.vec.len() {
            self.vec.resize(el + 1, false);
        }
        if !self.vec[el] {
            self.vec[el] = true;
            self.len += 1;
        }
    }

    pub fn remove(&mut self, id: usize) {
        if id < self.vec.len() && self.vec[id] {
            self.vec[id] = false;
            self.len -= 1
        }
    }

    pub fn pop(&mut self, index: usize) -> Option<usize> {
        let d = self.find_by_index(index);
        if d.is_some() {
            self.remove(d.unwrap());
        }
        d
    }

    #[inline]
    pub fn iter(&self) -> USetIter {
        USetIter {
            handle: self,
            index: 0,
            rindex: 0,
        }
    }

    #[inline]
    pub fn contains(&self, id: usize) -> bool {
        id < self.vec.len() && self.vec[id]
    }

    fn find_by_index(&self, index: usize) -> Option<usize> {
        let mut it = self.iter();
        for _i in 0..index {
            it.next();
        }
        it.next()
    }

    #[inline]
    pub fn min(&self) -> Option<usize> {
        self.iter().next()
    }

    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.iter().rev().next()
    }

    pub fn from_vec(vec: &[usize]) -> Self {
        let &mx = vec.iter().max().unwrap_or(&0);
        let mut set = vec![false; mx + 1];
        vec.iter().for_each(|&i| set[i] = true);
        USet {
            vec: set,
            len: vec.len(),
        }
    }

    pub fn from_range(r: Range<usize>) -> Self {
        let mut set = vec![false; r.end];
        let len = r.len();
        for i in r {
            set[i] = true;
        }
        USet { vec: set, len }
    }

    #[inline]
    pub fn from_fields(set: Vec<bool>, len: usize) -> Self {
        debug_assert_eq!(len, set.iter().filter(|&b| *b).count());
        USet { vec: set, len }
    }

    // TODO: think about the naming: verbs or nouns? `substract` is not symmetric, is that important?
    fn union(&self, other: &Self) -> Self {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let min: usize = min(self.min().unwrap(), other.min().unwrap());
            let max: usize = max(self.max().unwrap(), other.max().unwrap());

            let mut vec = vec![false; max + 1];
            let mut len = 0usize;

            vec.iter_mut()
                .enumerate()
                .skip(min)
                .take(max - min + 1)
                .for_each(|(i, value)| {
                    if self.contains(i) || other.contains(i) {
                        *value = true;
                        len += 1;
                    }
                });

            if len == 0 {
                USet::new()
            } else {
                USet { vec, len }
            }
        }
    }

    fn substract(&self, other: &USet) -> Self {
        let mut vec = self.vec.clone();
        let mut len = self.len();

        other
            .vec
            .iter()
            .take(vec.len())
            .enumerate()
            .for_each(|(i, &v)| {
                if v && vec[i] {
                    vec[i] = false;
                    len -= 1;
                }
            });

        if len == 0 {
            USet::new()
        } else {
            USet { vec, len }
        }
    }

    fn common_part(&self, other: &USet) -> Self {
        let total_len: usize = min(self.capacity(), other.capacity());
        let mn = (0..total_len).find(|&i| self.contains(i) && other.contains(i));
        if mn.is_none() {
            USet::new()
        } else {
            let min = mn.unwrap();
            let mx = (0..(total_len - min + 1))
                .find(|&i| self.contains(total_len - i) && other.contains(total_len - i));
            let max = total_len - mx.unwrap();
            debug_assert!(max >= min);

            let mut set = vec![false; max + 1];
            let mut len = 0usize;

            set.iter_mut()
                .enumerate()
                .skip(min)
                .take(max - min + 1)
                .for_each(|(i, value)| {
                    if self.contains(i) && other.contains(i) {
                        *value = true;
                        len += 1;
                    }
                });

            if len == 0 {
                USet::new()
            } else {
                USet { vec: set, len }
            }
        }
    }

    // TODO: rewrite it!
    fn xor_set(&self, other: &USet) -> Self {
        &(self + other) - &(self * other)
    }
}

impl PartialEq for USet {
    fn eq(&self, other: &USet) -> bool {
        self.len == other.len && self
            .vec
            .iter()
            .zip(other.vec.iter())
            .find(|&(&a, &b)| a != b)
            .is_none()
    }
}

impl Eq for USet {}

impl<'a> Add for &'a USet {
    type Output = USet;
    fn add(self, other: &USet) -> USet {
        self.union(other)
    }
}

impl<'a> Sub for &'a USet {
    type Output = USet;
    fn sub(self, other: &USet) -> USet {
        self.substract(other)
    }
}

impl<'a> Mul for &'a USet {
    type Output = USet;
    fn mul(self, other: &USet) -> USet {
        self.common_part(other)
    }
}

impl<'a> BitXor for &'a USet {
    type Output = USet;
    fn bitxor(self, other: &USet) -> USet {
        self.xor_set(other)
    }
}

impl<'a> From<&'a [usize]> for USet {
    fn from(slice: &'a [usize]) -> Self {
        USet::from_vec(slice)
    }
}

impl From<Vec<usize>> for USet {
    fn from(vec: Vec<usize>) -> Self {
        USet::from_vec(&vec)
    }
}

impl Into<Vec<usize>> for USet {
    fn into(self) -> Vec<usize> {
        self.iter().collect()
    }
}

use utils::umap::UMap;

impl<T> From<UMap<T>> for USet
where
    T: Clone + PartialEq,
{
    fn from(map: UMap<T>) -> Self {
        map.to_set()
    }
}

impl<'a> From<&'a Vec<usize>> for USet {
    fn from(vec: &'a Vec<usize>) -> Self {
        USet::from_vec(vec)
    }
}

impl From<Range<usize>> for USet {
    fn from(r: Range<usize>) -> Self {
        USet::from_range(r)
    }
}
