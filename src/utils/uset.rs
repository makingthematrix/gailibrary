#![macro_use]

use std::cmp;
use std::ops::Range;

use std::ops::{Add, BitXor, Mul, Sub};

// TODO: https://doc.rust-lang.org/src/alloc/vec_deque.rs.html#1909-1913
// rewrite in a similar fashion

#[allow(unused_macros)]
macro_rules! uset {
    ($($x:expr),*) => (USet::from_slice(&vec![$($x),*]))
}

#[derive(Debug, Default, Clone)]
pub struct USet {
    vec: Vec<bool>,
    len: usize,
    offset: usize,
    min: usize,
    max: usize,
}

pub struct USetIter<'a> {
    handle: &'a USet,
    index: usize,
    rindex: usize,
}

impl<'a> Iterator for USetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.handle.vec.len() - self.rindex {
            let index = self.index;
            self.index += 1;
            if self.handle.vec[index] {
                return Some(index + self.handle.offset);
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
                return Some(index + self.handle.offset);
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a USet {
    type Item = usize;
    type IntoIter = USetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub const INITIAL_CAPACITY: usize = 7;

lazy_static! {
    pub static ref EMPTY_SET: USet = USet::with_capacity(0);
}

impl USet {
    pub fn new() -> Self {
        USet::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> Self {
        USet {
            vec: vec![false; size],
            len: 0,
            offset: 0,
            min: 0,
            max: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.vec.len()
    }

    pub fn prune(&mut self) {
        if !self.is_empty() {
            let mut vec = vec![false; self.max - self.min + 1];
            for i in self.min..=self.max {
                vec[i - self.min] = self.contains(i);
            }
            self.vec = vec;
            self.offset = self.min;
        }
    }

    pub fn add(&mut self, id: usize) {
        match id {
            _ if self.capacity() == 0 => {
                // essentially EMPTY_SET
                self.vec = vec![false; INITIAL_CAPACITY];
                self.vec[0] = true;
                self.min = id;
                self.len += 1;
                self.max = id;
                self.offset = id;
            }
            _ if self.is_empty() => {
                self.vec[0] = true;
                self.min = id;
                self.len = 1;
                self.max = id;
                self.offset = id;
            }
            _ if id < self.offset => {
                let mut vec = vec![false; self.max - id];
                vec[0] = true;
                for i in self.min..=self.max {
                    vec[i - id] = self.contains(i);
                }
                self.vec = vec;
                self.len += 1;
                self.min = id;
                self.offset = id;
            }
            _ if id > self.offset + self.capacity() => {
                self.vec.resize(id + 1 - self.offset, false);
                self.vec[id - self.offset] = true;
                self.len += 1;
                self.max = id;
            }
            _ if !self.vec[id - self.offset] => {
                self.vec[id - self.offset] = true;
                self.len += 1;
                if id < self.min {
                    self.min = id
                } else if id > self.max {
                    self.max = id
                }
            }
            _ => {}
        }
    }

    pub fn remove(&mut self, id: usize) {
        match id {
            _ if id < self.min || id > self.max => {}
            _ if !self.contains(id) => {}
            _ if self.len == 1 => {
                self.vec[id - self.offset] = false;
                self.max = 0;
                self.min = 0;
                self.len = 0;
                self.offset = 0;
            }
            _ if id > self.min && id < self.max => {
                self.vec[id - self.offset] = false;
                self.len -= 1;
            }
            _ if id == self.min => {
                self.vec[id - self.offset] = false;
                self.len -= 1;
                self.min = (self.min..self.max)
                    .find(|&i| self.vec[i - self.offset])
                    .unwrap_or(self.max);
            }
            _ if id == self.max => {
                self.vec[id - self.offset] = false;
                self.len -= 1;
                self.max = (self.min..self.max)
                    .rev()
                    .find(|&i| self.vec[i - self.offset])
                    .unwrap_or(self.min);
            }
            _ => {}
        }
    }

    pub fn pop(&mut self, index: usize) -> Option<usize> {
        let d = self.find_by_index(index);
        if d.is_some() {
            self.remove(d.unwrap());
        }
        d
    }

    pub fn iter(&self) -> USetIter {
        USetIter {
            handle: self,
            index: 0,
            rindex: 0,
        }
    }

    pub fn contains(&self, id: usize) -> bool {
        id >= self.min && id <= self.max && self.vec[id - self.offset]
    }

    fn find_by_index(&self, index: usize) -> Option<usize> {
        if index >= self.len {
            None
        } else {
            let mut it = self.iter();
            for _i in 0..index {
                it.next();
            }
            it.next()
        }
    }

    pub fn min(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.min)
        }
    }

    pub fn max(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.max)
        }
    }

    fn make_from_slice(slice: &[usize]) -> (usize, usize, usize, Vec<bool>) {
        match slice.iter().minmax() {
            MinMaxResult::NoElements => (0, 0, 0, Vec::<bool>::new()),
            MinMaxResult::OneElement(&min) => (min, min, 1, vec![true]),
            MinMaxResult::MinMax(&min, &max) => {
                let len = slice.len();
                let capacity = cmp::max(INITIAL_CAPACITY, max + 1 - min);
                let mut vec = vec![false; capacity];
                slice.iter().for_each(|&i| vec[i - min] = true);
                (min, max, len, vec)
            }
        }
    }

    pub fn from_slice(slice: &[usize]) -> Self {
        if slice.is_empty() {
            EMPTY_SET.clone()
        } else {
            let (min, max, len, new_vec) = USet::make_from_slice(slice);
            USet {
                vec: new_vec,
                len,
                offset: min,
                min,
                max,
            }
        }
    }

    pub fn from_range(r: Range<usize>) -> Self {
        if r.len() == 0 {
            EMPTY_SET.clone()
        } else {
            let offset = r.start;
            let max = r.end;
            let len = r.len();
            let capacity = cmp::max(INITIAL_CAPACITY, r.len());
            let mut vec = vec![false; capacity];
            r.for_each(|i| vec[i - offset] = true);
            USet {
                vec,
                len,
                offset,
                min: offset,
                max,
            }
        }
    }

    pub fn from_fields(vec: Vec<bool>, offset: usize) -> Self {
        if vec.is_empty() {
            EMPTY_SET.clone()
        } else {
            let len = vec.iter().filter(|&b| *b).count();
            let min = vec
                .iter()
                .enumerate()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
                .unwrap()
                + offset;
            let max = vec
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
                .unwrap()
                + offset;
            USet {
                vec,
                len,
                offset,
                min,
                max,
            }
        }
    }

    pub fn add_all(&mut self, slice: &[usize]) {
        if !slice.is_empty() {
            if self.is_empty() {
                let (min, max, len, new_vec) = USet::make_from_slice(slice);
                self.min = min;
                self.max = max;
                self.offset = min;
                self.len = len;
                self.vec = new_vec;
            } else {
                let (min, max) = match slice.iter().minmax() {
                    MinMaxResult::NoElements => (0, 0), // should not happen
                    MinMaxResult::OneElement(&min) => (min, min),
                    MinMaxResult::MinMax(&min, &max) => (min, max),
                };

                if min >= self.min && max <= self.max {
                    slice.iter().for_each(|&id| {
                        if !self.vec[id - self.offset] {
                            self.vec[id - self.offset] = true;
                            self.len += 1;
                        }
                    })
                } else {
                    let new_min = cmp::min(self.min, min);
                    let new_max = cmp::max(self.max, max);
                    let mut new_vec = vec![false; new_max - new_min + 1];
                    self.iter().skip(self.min - self.offset).take(self.max - self.min + 1).for_each(|id| new_vec[id - new_min] = true);
                    slice.iter().for_each(|&id| {
                        if !new_vec[id - new_min] {
                            new_vec[id - new_min] = true;
                            self.len += 1;
                        }
                    });
                    self.min = new_min;
                    self.offset = new_min;
                    self.max = new_max;
                    self.vec = new_vec;
                }
            }
        }
    }

    fn union(&self, other: &Self) -> Self {
        if self.is_empty() {
            if other.is_empty() {
                EMPTY_SET.clone()
            } else {
                other.clone()
            }
        } else if other.is_empty() {
            if self.is_empty() {
                EMPTY_SET.clone()
            } else {
                self.clone()
            }
        } else {
            let min: usize = cmp::min(self.min, other.min);
            let max: usize = cmp::max(self.max, other.max);

            let mut vec = vec![false; max + 1 - min];
            let mut len = 0usize;

            vec.iter_mut()
                .enumerate()
                .for_each(|(i, value)| {
                    if self.contains(i + self.offset) || other.contains(i + self.offset) {
                        *value = true;
                        len += 1;
                    }
                });

            USet {
                vec,
                len,
                offset: min,
                min,
                max,
            }
        }
    }

    fn difference(&self, other: &USet) -> Self {
        let mut vec = self.vec.clone();
        let mut len = self.len;
        let offset = self.offset;

        other.iter().for_each(|i| {
            vec[i - offset] = false;
            len -= 1;
        });

        if len == 0 {
            EMPTY_SET.clone()
        } else {
            let min = vec
                .iter()
                .enumerate()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
                .unwrap()
                + offset;
            let max = vec
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
                .unwrap()
                + offset;
            USet {
                vec,
                len,
                offset,
                min,
                max,
            }
        }
    }

    fn common_part(&self, other: &USet) -> Self {
        if self.is_empty() || other.is_empty() {
            EMPTY_SET.clone()
        } else {
            let rough_range = cmp::max(self.min, other.min)..=cmp::min(self.max, other.max);
            let mn = rough_range
                .clone()
                .find(|&i| self.contains(i) && other.contains(i));
            let mx = rough_range
                .clone()
                .rev()
                .find(|&i| self.contains(i) && other.contains(i));
            if let Some(min) = mn {
                if let Some(max) = mx {
                    let mut vec = vec![false; max + 1 - min];
                    let mut len = 0usize;
                    for i in min..=max {
                        if self.contains(i) && other.contains(i) {
                            vec[i - min] = true;
                            len += 1;
                        }
                    }
                    USet {
                        vec,
                        len,
                        offset: min,
                        min,
                        max,
                    }
                } else {
                    EMPTY_SET.clone()
                }
            } else {
                EMPTY_SET.clone()
            }
        }
    }

    fn xor_set(&self, other: &USet) -> Self {
        if self.is_empty() && other.is_empty() {
            EMPTY_SET.clone()
        } else if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let rough_range = cmp::min(self.min, other.min)..=cmp::max(self.max, other.max);
            let mn = rough_range.clone().find(|&i| {
                (self.contains(i) && !other.contains(i)) || (!self.contains(i) && other.contains(i))
            });
            let mx = rough_range.clone().rev().find(|&i| {
                (self.contains(i) && !other.contains(i)) || (!self.contains(i) && other.contains(i))
            });
            if let Some(min) = mn {
                if let Some(max) = mx {
                    let mut vec = vec![false; max + 1 - min];
                    let mut len = 0usize;
                    for i in min..=max {
                        if (self.contains(i) && !other.contains(i))
                            || (!self.contains(i) && other.contains(i))
                        {
                            vec[i - min] = true;
                            len += 1;
                        }
                    }
                    USet {
                        vec,
                        len,
                        offset: min,
                        min,
                        max,
                    }
                } else {
                    EMPTY_SET.clone()
                }
            } else {
                EMPTY_SET.clone()
            }
        }
    }
}

impl PartialEq for USet {
    fn eq(&self, other: &USet) -> bool {
        self.len == other.len
            && self.min == other.min
            && self.max == other.max
            && self
                .vec
                .iter()
                .skip(self.min - self.offset)
                .take(self.max + 1 - self.min)
                .zip(
                    other
                        .vec
                        .iter()
                        .skip(other.min - other.offset)
                        .take(other.max + 1 - other.min),
                )
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
        self.difference(other)
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
        USet::from_slice(slice)
    }
}

impl From<Vec<usize>> for USet {
    fn from(vec: Vec<usize>) -> Self {
        USet::from_slice(&vec)
    }
}

impl Into<Vec<usize>> for USet {
    fn into(self) -> Vec<usize> {
        self.iter().collect()
    }
}

use crate::utils::umap::UMap;
use itertools::{Itertools, MinMaxResult};

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
        USet::from_slice(vec)
    }
}

impl From<Range<usize>> for USet {
    fn from(r: Range<usize>) -> Self {
        USet::from_range(r)
    }
}
