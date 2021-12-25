use std::num::NonZeroU32;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct Permutations {
    counter: u32,
    filters: SmallVec<[u32; 4]>,
}
impl Permutations {
    pub fn new(word: &str) -> Self {
        let mut filters: SmallVec<_> = word.chars()
            .map(|c| {
                1 << (c as u32 - 97)
            })
            .collect();
        filters.sort_unstable();
        filters.dedup();
        Permutations {
            counter: 0,
            filters
        }
    }
}
impl Iterator for Permutations {
    type Item = NonZeroU32;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        let end = 2u32.pow(self.filters.len() as u32);
        if self.counter >= end {
            return None;
        }

        let ones = (0..self.filters.len())
            .map(|n| (self.counter >> n) % 2 != 0);
        let combined = ones.zip(&self.filters)
            .filter_map(|(b, &n)| b.then(|| n))
            .fold(0, |old, new| {
                old | new
            });
        NonZeroU32::new(combined)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = ((1 << (self.filters.len() as u32)) - self.counter) as usize - 1;
        (remaining, Some(remaining))
    }
}

#[test]
fn hint_test() {
    let iter = Permutations::new("beans");
    let pred = iter.size_hint().0;
    let real = iter.count();
    assert_eq!(pred, real)
}
