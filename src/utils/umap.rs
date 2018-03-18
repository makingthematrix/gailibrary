#![macro_use]

use std::clone::Clone;
use utils::uset::USet;

use std::cmp::{max, min};

use std::ops::{Add, BitXor, Mul, Sub};

// TODO: https://doc.rust-lang.org/src/alloc/vec_deque.rs.html#1909-1913
// rewrite in a similar fashion

#[derive(Debug, Default, Clone)]
pub struct UMap<T> {
    vec: Vec<Option<T>>,
    len: usize,
}

#[derive(Debug, Clone)]
pub struct UMapIter<'a, T: 'a> {
    handle: &'a UMap<T>,
    index: usize,
    rindex: usize,
}

impl<'a, T> Iterator for UMapIter<'a, T>
where
    T: Clone + PartialEq,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.handle.vec.len() - self.rindex;
        while self.index < max {
            let index = self.index;
            self.index += 1;
            if let Some(&Some(ref value)) = self.handle.vec.get(index) {
                return Some((index, value.clone()));
            }
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for UMapIter<'a, T>
where
    T: Clone + PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.handle.vec.len();
        while self.rindex < len - self.index {
            let index = len - self.rindex - 1;
            self.rindex += 1;
            if let Some(&Some(ref value)) = self.handle.vec.get(index) {
                return Some((index, value.clone()));
            }
        }
        None
    }
}

impl<T> UMap<T>
where
    T: Clone + PartialEq,
{
    #[inline]
    pub fn new() -> Self {
        UMap::with_capacity(0)
    }

    pub fn with_capacity(size: usize) -> Self {
        UMap {
            vec: vec![None; size],
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

    pub fn put(&mut self, key: usize, value: T) {
        if key >= self.vec.len() {
            self.vec.resize(key + 1, None);
        }
        if self.vec[key].is_none() {
            self.len += 1;
        }
        self.vec[key] = Some(value);
    }

    #[inline]
    pub fn contains(&self, key: usize) -> bool {
        key < self.vec.len() && self.vec[key].is_some()
    }

    #[inline]
    pub fn get(&self, key: usize) -> Option<T> {
        if key < self.vec.len() {
            unsafe { self.vec.get_unchecked(key).clone() }
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key < self.vec.len() && self.vec[key].is_some() {
            let res = self.get(key);
            self.vec[key] = None;
            self.len -= 1;
            res
        } else {
            None
        }
    }

    #[inline]
    pub fn to_set(&self) -> USet {
        let set: Vec<bool> = self.vec.iter().map(|value| value.is_some()).collect();
        USet::from_fields(set, self.len)
    }

    #[inline]
    pub fn iter(&self) -> UMapIter<T> {
        UMapIter {
            handle: self,
            index: 0,
            rindex: 0,
        }
    }

    #[inline]
    pub fn min(&self) -> Option<usize> {
        self.iter().next().map(|(i, ..)| i)
    }

    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.iter().rev().next().map(|(i, ..)| i)
    }

    pub fn from_vec(org_vec: &[(usize, T)]) -> Self {
        let &mx = org_vec.iter().map(|(i, ..)| i).max().unwrap_or(&0);
        let mut vec = vec![None; mx + 1];
        org_vec
            .iter()
            .for_each(|&(i, ref v)| vec[i] = Some(v.clone()));
        UMap {
            vec,
            len: org_vec.len(),
        }
    }

    #[inline]
    fn debug_compare(self: &Self, other: &UMap<T>) {
        // don't perform operation on maps if they have different elements at the same places - clearly something's messed up
        debug_assert!(
            self.iter()
                .zip(other.iter())
                .find(|&((i1, ref v1), (i2, ref v2))| i1 == i2 && v1 != v2)
                .is_none()
        );
    }

    // TODO: think about the naming: verbs or nouns? `substract` is not symmetric, is that important?
    pub fn union(&self, other: &Self) -> Self {
        self.debug_compare(other);
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let min: usize = min(self.min().unwrap(), other.min().unwrap());
            let max: usize = max(self.max().unwrap(), other.max().unwrap());
            let mut vec = vec![None; max + 1];
            let mut len = 0usize;

            vec.iter_mut()
                .enumerate()
                .skip(min)
                .take(max - min + 1)
                .for_each(|(key, value)| {
                    if let Some(&Some(ref v)) = self.vec.get(key) {
                        *value = Some(v.clone());
                        len += 1;
                    } else if let Some(&Some(ref v)) = other.vec.get(key) {
                        *value = Some(v.clone());
                        len += 1;
                    }
                });

            if len == 0 {
                UMap::new()
            } else {
                UMap { vec, len }
            }
        }
    }

    pub fn subset(&self, uset: &USet) -> Self {
        if uset.is_empty() {
            self.clone()
        } else if self.is_empty() {
            UMap::new()
        } else {
            let capacity = min(self.capacity(), uset.capacity());
            let mx = min(self.max().unwrap_or(0), uset.max().unwrap_or(0));
            let mut vec = vec![None; capacity];
            let mut len = 0usize;
            uset.iter().take_while(|&key| key <= mx).for_each(|key| {
                if let Some(value) = self.get(key) {
                    vec[key] = Some(value.clone());
                    len += 1;
                }
            });
            UMap { vec, len }
        }
    }

    pub fn substract(&self, uset: &USet) -> Self {
        if uset.is_empty() {
            self.clone()
        } else if self.is_empty() {
            UMap::new()
        } else {
            let mut vec = vec![None; self.capacity()];
            let mut len = 0usize;
            for (key, value) in self.iter() {
                if !uset.contains(key) {
                    vec[key] = Some(value.clone());
                    len += 1;
                }
            }
            UMap { vec, len }
        }
    }

    // TODO: rewrite it!
    pub fn xor(&self, other: &Self) -> Self {
        self.debug_compare(other);
        &(self + other) - &(self * other)
    }
}

impl<T> PartialEq for UMap<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
            && self.iter()
                .zip(other.iter())
                .find(|&((key1, ref value1), (key2, ref value2))| key1 != key2 || value1 != value2)
                .is_none()
    }
}

impl<T> Eq for UMap<T>
where
    T: Clone + PartialEq,
{
}

impl<'a, T> Add for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn add(self, other: &UMap<T>) -> UMap<T> {
        self.union(other)
    }
}

impl<'a, T> Sub for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn sub(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.substract(&other.to_set())
    }
}

impl<'a, T> Mul for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn mul(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.subset(&other.to_set())
    }
}

impl<'a, T> BitXor for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn bitxor(self, other: &UMap<T>) -> UMap<T> {
        self.xor(other)
    }
}

impl<T> From<Vec<(usize, T)>> for UMap<T>
where
    T: Clone + PartialEq,
{
    fn from(vec: Vec<(usize, T)>) -> Self {
        UMap::from_vec(&vec)
    }
}
