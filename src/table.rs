use std::{num::NonZeroU32, iter::repeat};

use smallvec::{SmallVec, smallvec};

// I can't see any differences no matter what I set this to tbh
const BLOCK: usize = 32;

// A simple Sparse set
#[derive(Debug, Clone, Default)]
pub struct KeyTable<T> {
    indexes: Vec<Option<Box<[Option<NonZeroU32>; BLOCK]>>>,
    values: Vec<SmallVec<[T; 4]>>
}
impl<T> KeyTable<T> {
    pub fn new(size: usize) -> Self {
        KeyTable {
            indexes: vec![None; size],
            values: Vec::with_capacity(size)
        }
    }
    pub fn insert(&mut self, loc: NonZeroU32, value: T) {
        let shifted = (loc.get() - 1) as usize;
        let array = shifted / BLOCK;
        let array_idx = shifted % BLOCK;
        let inserted_index = unsafe {NonZeroU32::new_unchecked(self.values.len() as u32 + 1)};
        //self.values.push(value);
        match self.indexes.get_mut(array) {
            Some(Some(ma)) => match &mut ma[array_idx] {
                Some(i) => {
                    let i = (i.get() - 1) as usize;
                    self.values[i].push(value);
                    return;
                },
                a @ None => {
                    *a = Some(inserted_index);
                },
            },
            Some(a @ None) => {
                let mut new_array = Box::new([None; BLOCK]);
                new_array[array_idx] = Some(inserted_index);
                *a = Some(new_array);
            }
            None => {
                let diff = array - self.indexes.len();
                let iter = repeat(None);
                self.indexes.extend(iter.take(diff));
                let mut new_array = Box::new([None; BLOCK]);
                new_array[array_idx] = Some(inserted_index);
                self.indexes.push(Some(new_array));
            }
        }
        self.values.push(smallvec![value]);
    }
}