use arrayvec::ArrayString;
use smallvec::SmallVec;

fn main() {
    let all_words = include_str!("../words_alpha.txt");
    let finder = WordFinder::from_str(all_words);
}

#[derive(Debug, Default)]
struct WordFinder<'a> {
    string: &'a str,
    map: ahash::AHashMap<ArrayString<20>, SmallVec<[u32; 4]>>,
    word_starts: Vec<u32>,
    word_lens: Vec<u8>,
}
impl<'a> WordFinder<'a> {
    fn from_str(src: &'a str) -> Self {
        let len = src.split('\n').count();
        let mut me = WordFinder {
            string: src,
            map: ahash::AHashMap::with_capacity(len),
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
            me.insert_word(start as u32, end as u8);
            let word = me.get_word(start as u32, end as u8);
            println!("word: {}", word);
            start += end + 1;
        }
        me
    }
    fn insert_word(&mut self, word_start: u32, word_len: u8) {
        let mut sortable: Vec<char> = self.get_word(word_start, word_len).chars().collect();
        sortable.sort_unstable();
        sortable.dedup();
        for perm in Permutator::new(&sortable) {
            self.insert_strict(perm, word_start, word_len);
        }
    }
    fn insert_strict(&mut self, loc: ArrayString<20>, word_start: u32, word_len: u8) {
        let insertion = self.word_starts.len() as u32;
        self.word_starts.push(word_start);
        self.word_lens.push(word_len);
        match self.map.get_mut(&*loc) {
            Some(words) => {
                words.push(insertion);
            },
            None => {
                self.map.insert(loc,  smallvec::smallvec![insertion]);
            }
        }
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

struct Permutator<'a> {
    counter: u32,
    chars: &'a [char],
}
impl<'a> Permutator<'a> {
    fn new(chars: &'a [char]) -> Self {
        Permutator {
            counter: 0,
            chars
        }
    }
}
impl<'a> Iterator for Permutator<'a> {
    type Item = ArrayString<20>;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        let end = 2u32.pow(self.chars.len() as u32);
        if self.counter >= end {
            return None;
        }

        let ones = (0..self.chars.len())
            .map(|n| (self.counter >> n) % 2 != 0);
        let mut collected = ArrayString::new();
        let filtered = ones.zip(self.chars)
            .filter_map(|(b, c)| b.then(|| *c));
        for c in filtered {
            collected.push(c);
        }
        Some(collected)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (2u32.pow(self.chars.len() as u32) - self.counter) as usize - 1;
        (remaining, Some(remaining))
    }
}
