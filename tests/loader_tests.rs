use rfortune::config::set_app_dir_for_tests;
use rfortune::loader::FortuneFile;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn setup_test_env() -> PathBuf {
    let sandbox = std::env::temp_dir().join("rfortune_test_env_loader");
    println!("Setting up test sandbox at {:?}", sandbox);
    fs::create_dir_all(&sandbox).unwrap();
    set_app_dir_for_tests(sandbox.clone());
    sandbox
}

fn create_temp_file(sandbox: &Path, content: &str, filename: &str) -> PathBuf {
    let path = sandbox.join(filename);
    let mut file = File::create(&path).expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    path
}

#[test]
fn test_loader_with_title() {
    let sandbox = setup_test_env();

    let data = "# Murphy's Law\n%\nIf something can go wrong, it will.\n%\nLeft to themselves, things tend to go from bad to worse.";
    let path = create_temp_file(&sandbox, data, "murphy_test.dat");
    let fortune_file = FortuneFile::from_file(&path).expect("Failed to parse file");

    assert_eq!(fortune_file.title.unwrap(), "Murphy's Law");
    assert_eq!(fortune_file.quotes.len(), 2);
    assert!(fortune_file.quotes.iter().any(|q| q.contains("go wrong")));
}

#[test]
fn test_loader_without_title() {
    let sandbox = setup_test_env();

    let data = "%\nQuote one.\n%\nQuote two.";
    let path = create_temp_file(&sandbox, data, "notitle_test.dat");
    let fortune_file = FortuneFile::from_file(&path).expect("Failed to parse file");

    assert!(fortune_file.title.is_none());
    assert_eq!(fortune_file.quotes.len(), 2);
}

#[test]
fn test_loader_empty_file() {
    let sandbox = setup_test_env();

    let data = "";
    let path = create_temp_file(&sandbox, data, "empty_test.dat");
    let result = FortuneFile::from_file(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No quotes found in the file.");
}
