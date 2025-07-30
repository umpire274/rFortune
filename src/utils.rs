use crate::loader::FortuneFile;
use rand::seq::IndexedRandom;

pub fn random_quote(quotes: &[String]) -> &str {
    let mut rng = rand::rng();
    quotes.choose(&mut rng).map(|s| s.as_str()).unwrap()
}

pub fn print_random(fortune_file: &FortuneFile) {
    if let Some(title) = &fortune_file.title {
        println!("{}\n", title);
    }
    println!("{}", random_quote(&fortune_file.quotes));
}
