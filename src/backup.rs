use arrayvec::ArrayString;
use smallvec::SmallVec;

fn main() {
    let all_words = include_str!("../words_alpha.txt");
    let finder = WordFinder::from_str(all_words);
}

#[derive(Debug, Default)]
struct WordFinder<'a> {
    map: ahash::AHashMap<ArrayString<20>, SmallVec<[u32; 4]>>,
    words: Vec<&'a str>
}
impl<'a> WordFinder<'a> {
    fn from_str(src: &'a str) -> Self {
        let iter = src.split('\n');
        let len = iter.clone().count();
        let mut me = WordFinder {
            map: ahash::AHashMap::with_capacity(len),
            words: Vec::with_capacity(len),
        };
        for word in iter {
            println!("word: {}", word);
            me.insert_word(word);
        }
        me
    }
    fn insert_word(&mut self, word: &'a str) {
        let mut sortable: Vec<char> = word.chars().collect();
        sortable.sort_unstable();
        sortable.dedup();
        for perm in Permuter::new(&sortable) {
            self.insert_strict(perm, word);
        }
    }
    fn insert_strict(&mut self, loc: ArrayString<20>, word: &'a str) {
        let insertion = self.words.len() as u32;
        self.words.push(word);
        match self.map.get_mut(&*loc) {
            Some(words) => {
                words.push(insertion);
            },
            None => {
                self.map.insert(loc,  smallvec::smallvec![insertion]);
            }
        }
    }
}

struct Lookup<'a> {
    numbered: Vec<(char, usize)>,
    slice: &'a str
}
impl<'a> Lookup<'a> {

}

struct Permuter<'a> {
    counter: u32,
    chars: &'a [char],
}
impl<'a> Permuter<'a> {
    fn new(chars: &'a [char]) -> Self {
        Permuter {
            counter: 0,
            chars
        }
    }
}
impl<'a> Iterator for Permuter<'a> {
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
