use rfortune::loader::FortuneFile;
use rfortune::utils::{random_quote, print_random};

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

    // Just ensure it doesn't panic; capture output requires more setup
    print_random(&fortune_file);
}
