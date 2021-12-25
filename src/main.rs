use std::num::NonZeroU32;

use permutation::Permutations;
use table::KeyTable;

fn main() {
    let all_words = include_str!("../words_alpha");
    let finder = WordFinder::from_str(all_words);
}

mod permutation;
mod table;

#[derive(Debug, Default)]
struct WordFinder<'a> {
    string: &'a str,
    table: KeyTable<u32>,
    word_starts: Vec<u32>,
    word_lens: Vec<u8>,
}
impl<'a> WordFinder<'a> {
    fn from_str(src: &'a str) -> Self {
        let len = src.split('\n').count();
        let mut me = WordFinder {
            string: src,
            table: KeyTable::new(len),
            word_starts: Vec::with_capacity(len),
            word_lens: Vec::with_capacity(len)
        };
        let mut start = 0;
        while start < src.len() {
            let end = src[start..].char_indices()
                .filter(|(_i, c)| *c == '\n')
                .map(|(i, _c)| i)
                .next()
                .unwrap_or(src.len());
            let word = me.get_word(start as u32, end as u8);
            println!("word: {}", word);
            me.insert_word(word, start as u32, end as u8);
            start += end + 1;
        }
        me
    }
    fn insert_word(&mut self, word: &str, word_start: u32, word_len: u8) {
        for perm in Permutations::new(word) {
            self.insert_strict(perm, word_start, word_len);
        }
    }
    fn insert_strict(&mut self, loc: NonZeroU32, word_start: u32, word_len: u8) {
        let word_start_idx = self.word_starts.len() as u32;
        self.word_starts.push(word_start);
        self.word_lens.push(word_len);
        self.table.insert(loc, word_start_idx);
    }
    fn get_word(&self, word_start: u32, word_len: u8) -> &'a str {
        let s = word_start as usize;
        let e = word_len as usize + s;
        if e > self.string.len() {
            panic!("S: {:?}, E: {:?}, M: {:?}", s, e, self.string.len());
        }
        &self.string[s..e]
    }
}
