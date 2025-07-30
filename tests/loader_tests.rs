use rfortune::loader::FortuneFile;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn create_temp_file(content: &str, filename: &str) -> String {
    let mut path: PathBuf = env::temp_dir();
    path.push(filename);

    let mut file = File::create(&path).expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    path.to_string_lossy().to_string()
}

#[test]
fn test_loader_with_title() {
    let data = "# Murphy's Law\n%\nIf something can go wrong, it will.\n%\nLeft to themselves, things tend to go from bad to worse.";
    let path = create_temp_file(data, "murphy_test.dat");
    let fortune_file = FortuneFile::from_file(&path).expect("Failed to parse file");

    assert_eq!(fortune_file.title.unwrap(), "Murphy's Law");
    assert_eq!(fortune_file.quotes.len(), 2);
    assert!(fortune_file.quotes.iter().any(|q| q.contains("go wrong")));
}

#[test]
fn test_loader_without_title() {
    let data = "%\nQuote one.\n%\nQuote two.";
    let path = create_temp_file(data, "notitle_test.dat");
    let fortune_file = FortuneFile::from_file(&path).expect("Failed to parse file");

    assert!(fortune_file.title.is_none());
    assert_eq!(fortune_file.quotes.len(), 2);
}

#[test]
fn test_loader_empty_file() {
    let data = "";
    let path = create_temp_file(data, "empty_test.dat");
    let result = FortuneFile::from_file(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No quotes found in the file.");
}
