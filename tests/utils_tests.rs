use rfortune::config::{get_config_path, load_config, set_app_dir_for_tests};
use rfortune::utils::{
    load_last_cache, print_random_from_files, random_nonrepeating, save_last_cache,
};
use std::fs;
use std::path::{Path, PathBuf};

fn setup_test_env() -> PathBuf {
    let sandbox = std::env::temp_dir().join("rfortune_test_env_utils");
    fs::create_dir_all(&sandbox).unwrap();
    set_app_dir_for_tests(sandbox.clone());
    sandbox
}

#[test]
fn test_random_quote_selection() {
    let quotes = vec![
        String::from("Quote A"),
        String::from("Quote B"),
        String::from("Quote C"),
    ];

    let result = random_nonrepeating(&quotes, None);
    assert!(quotes.contains(&result.to_string()));
}

#[test]
fn test_print_random_output() {
    let sandbox = setup_test_env();

    let file_path = sandbox.join("test_single.fort");
    fs::write(&file_path, "Hello world\n%").expect("Failed to write temp fortune file");

    let paths: Vec<&Path> = vec![file_path.as_path()];

    let result = print_random_from_files(&paths);
    assert!(result.is_ok());
}

#[test]
fn test_cache_read_write() {
    let sandbox = setup_test_env();

    let file_path = sandbox.join("test_cache_source.fort");
    let quote = "Hello Cache";

    save_last_cache(&file_path, quote).expect("failed to save cache");
    let loaded = load_last_cache(&file_path).expect("failed to load cache");

    assert_eq!(loaded, quote);
}

#[test]
fn test_config_auto_migration_to_fortune_files() {
    let _sandbox = setup_test_env(); // ✅ forza app_dir() → SANDBOX

    let cfg_path = get_config_path();
    let parent = cfg_path.parent().unwrap();
    fs::create_dir_all(parent).unwrap();

    let legacy_config = r#"default_file: "test.fort"
print_title: true
use_cache: true
"#;
    fs::write(&cfg_path, legacy_config).unwrap();

    let cfg = load_config().expect("config must load");

    assert_eq!(cfg.fortune_files, vec!["test.fort".to_string()]);
}

#[test]
fn test_no_repeat_on_same_file() {
    let sandbox = setup_test_env();

    let file_path = sandbox.join("test_no_repeat.fort");
    let content = "Quote 1\n%\nQuote 2\n%\nQuote 3\n";
    fs::write(&file_path, content).unwrap();

    let paths: Vec<&Path> = vec![file_path.as_path()];

    save_last_cache(&file_path, "Quote 1").expect("failed to save initial cache");

    print_random_from_files(&paths).expect("print_random_from_files failed");

    let new_last = load_last_cache(&file_path).expect("cache missing after print");
    assert_ne!(new_last, "Quote 1");
}
