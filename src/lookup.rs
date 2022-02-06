use smallvec::{SmallVec, smallvec};
use std::{num::NonZeroU32, iter::repeat};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letters(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counts(u64);

fn digest(word: &str) -> (Letters, Counts) {
    let letters = word.chars()
        .map(|c| {
            1 << (c as u32 - 97)
        })
        .fold(0, |old, new| new | old);
    let counts = (0u8..26)
        .filter(|n| {
            let shifted = 1u32 << n;
            let overlapper = letters & shifted;
            overlapper != 0
        }).map(|n| {
            let letter = (97 + n) as char;
            let occurances = word
                .chars()
                .filter(|&c| c == letter)
                .count();
            occurances
        }).enumerate()
        .fold(0, |old, (i, occurances)| {
            let insert = (occurances as u64) << (i * 4);
            old | insert
        });
    (Letters(letters), Counts(counts))
}

const BLOCK: usize = 32;

#[derive(Debug, Clone, Default)]
struct SparseSet<T, B> {
    indexes: Vec<Option<Box<[Option<NonZeroU32>; BLOCK]>>>,
    values: Vec<SmallVec<[T; 2]>>,
    secondary: Vec<Vec<SmallVec<[B; 4]>>>
}
impl<T: Eq, B> SparseSet<T, B> {
    fn new(size: usize) -> Self {
        SparseSet {
            indexes: vec![None; size / BLOCK],
            secondary: Vec::with_capacity(size),
            values: Vec::with_capacity(size),
        }
    }
    // Does not currently use secondary
    fn insert(&mut self, letters: Letters, value: T, second_value: B) {
        let loc = match NonZeroU32::new(letters.0) {
            Some(n) => n,
            None => return
        };
        let shifted = (loc.get() - 1) as usize;
        let array = shifted / BLOCK;
        let array_idx = shifted % BLOCK;
        let inserted_index = unsafe {NonZeroU32::new_unchecked(self.values.len() as u32 + 1)};
        match self.indexes.get_mut(array) {
            Some(Some(ma)) => match &mut ma[array_idx] {
                Some(i) => {
                    let i = (i.get() - 1) as usize;
                    for (j, ev) in self.values[i].iter().enumerate() {
                        if *ev == value {
                            self.secondary[i][j].push(second_value);
                            return;
                        }
                    }
                    self.values[i].push(value);
                    self.secondary[i].push(smallvec![second_value]);
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
        self.secondary.push(vec![smallvec![second_value]]);
    }
}

pub struct Finder {
    dict: String,
    letters_to_pcounts: SparseSet<Counts, u32>,
    word_starts: Vec<u32>,
    word_lens: Vec<u8>
}
impl Finder {
    pub fn new(dict: String) -> Finder {
        let len = dict.split_ascii_whitespace().count();
        let mut me = Finder {
            dict: String::new(),
            letters_to_pcounts: SparseSet::new(len),
            word_starts: Vec::with_capacity(len),
            word_lens: Vec::with_capacity(len),
        };
        let mut word_start = 0;
        for (i, _) in dict.char_indices()
            .filter(|(_, c)| *c == '\n' ) {
            let diff = (i - word_start) as u8;
            let start = word_start as u32;
            word_start = i + 1;
            me.insert(&dict, start, diff);
        }
        me.dict = dict;
        me
    }
    fn insert(&mut self, ex_dict: &str, word_start: u32, word_len: u8) {
        let start_idx = word_start as usize;
        let word = &ex_dict[start_idx..start_idx + word_len as usize];
        let insertion = self.word_starts.len() as u32;
        self.word_starts.push(word_start);
        self.word_lens.push(word_len);
        let (letters, counts) = digest(word);
        self.letters_to_pcounts.insert(letters, counts, insertion);
    }
}
