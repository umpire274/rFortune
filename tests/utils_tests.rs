use std::env;
use rfortune::loader::FortuneFile;
use rfortune::utils::{random_quote, print_random, get_cache_path, read_last_cache};

#[test]
fn test_random_quote_selection() {
    let quotes = vec![
        String::from("Quote A"),
        String::from("Quote B"),
        String::from("Quote C"),
    ];

    let result = random_quote(&quotes);
    assert!(quotes.iter().any(|q| q == result));
}

#[test]
fn test_print_random_output() {
    let fortune_file = FortuneFile {
        title: Some("Test Title".to_string()),
        quotes: vec!["This is a test quote.".to_string()],
    };

    // Simula path verso un file .dat
    let mut path = env::temp_dir();
    path.push("test_quotes.dat");

    // Verifica solo che non panichi (output testato altrove)
    print_random(&fortune_file, &path);
}

#[test]
fn test_cache_read_write() {
    let quotes = vec!["Alpha".to_string(), "Beta".to_string()];
    let fortune_file = FortuneFile {
        title: None,
        quotes: quotes.clone(),
    };

    let mut temp_path = env::temp_dir();
    temp_path.push("test_cache.dat");

    // Prima esecuzione: salva una citazione
    print_random(&fortune_file, &temp_path);

    // Leggi da cache
    let cache_path = get_cache_path(&temp_path);
    let cached = read_last_cache(&cache_path);

    assert!(cached.is_some());
    assert!(quotes.iter().any(|q| Some(q) == cached.as_ref()));
}
