use rfortune::config::{get_config_path, load_config};
use rfortune::utils::{
    load_last_cache, print_random_from_files, random_nonrepeating, save_last_cache,
};
use std::path::Path;
use std::{env, fs};

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
    // Creiamo un file fortune temporaneo
    let mut path = env::temp_dir();
    path.push("test_single.fort");

    fs::write(&path, "Hello world\n%") // formato fortune valido
        .expect("Failed to write temp fortune file");

    let paths: Vec<&Path> = vec![path.as_path()];

    // Verifica solo che non panichi
    let result = print_random_from_files(&paths);
    assert!(result.is_ok());
}

#[test]
fn test_cache_read_write() {
    let mut path = env::temp_dir();
    path.push("test_cache_source.fort");

    // Simuliamo una citazione
    let quote = "Hello Cache";

    // Scriviamo la cache
    save_last_cache(&path, quote).expect("failed to save cache");

    // Rileggiamo
    let loaded = load_last_cache(&path).expect("failed to load cache");

    assert_eq!(loaded, quote);
}

#[test]
fn test_config_auto_migration_to_fortune_files() {
    // Prepariamo una config vecchio stile (senza fortune_files)
    let cfg_path = get_config_path();
    let parent = cfg_path.parent().unwrap();
    fs::create_dir_all(parent).unwrap();

    let legacy_config = r#"default_file: "/tmp/rfortune_test.dat"
print_title: true
use_cache: true
"#;

    fs::write(&cfg_path, legacy_config).unwrap();

    // Carichiamo la config tramite la funzione ufficiale
    let cfg = load_config().expect("config must load");

    // ✅ fortune_files deve essere popolato automaticamente
    assert_eq!(
        cfg.fortune_files,
        vec!["/tmp/rfortune_test.dat".to_string()]
    );
}

#[test]
fn test_no_repeat_on_same_file() {
    // Creiamo un file fortune temporaneo
    let mut file_path = env::temp_dir();
    file_path.push("test_no_repeat.fort");

    let content = "Quote 1\n%\nQuote 2\n%\nQuote 3\n";
    fs::write(&file_path, content).unwrap();

    let paths: Vec<&Path> = vec![file_path.as_path()];

    // 1) Forziamo la cache: ultima citazione = "Quote 1"
    save_last_cache(&file_path, "Quote 1").expect("failed to save initial cache");

    // 2) Eseguiamo la scelta casuale
    print_random_from_files(&paths).expect("print_random_from_files failed");

    // 3) L’ultima citazione del file ora deve essere diversa da "Quote 1"
    let new_last = load_last_cache(&file_path).expect("cache missing after print");
    assert_ne!(
        new_last, "Quote 1",
        "The same quote should not repeat from the same file"
    );
}
