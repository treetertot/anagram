use lookup::Finder;

fn main() {
    let all_words = include_str!("../words_alpha");
    let finder = Finder::new(all_words.into());
}

mod lookup;
