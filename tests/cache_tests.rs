use std::path::PathBuf;

use rfortune::{config, utils};

#[test]
fn cache_write_read_roundtrip() {
    // Crea una sandbox temporanea per la directory dell'app
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut tmp = std::env::temp_dir();
    tmp.push(format!("rfortune_test_{}_{}", std::process::id(), nanos));

    // Pulizia iniziale e creazione
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("create tmp dir");

    // Imposta la directory di app per i test
    config::set_app_dir_for_tests(tmp.clone());

    // Crea un file sorgente che verrà usato come "fortune file"
    let source = tmp.join("source.dat");
    std::fs::write(&source, "Sample quote\n%").expect("write source");

    // Salva una citazione nella cache e poi la rilegge
    let quote = "This is a test quote";
    utils::save_last_cache(&source, quote).expect("save_last_cache failed");

    let loaded = utils::load_last_cache(&source).expect("load_last_cache failed");
    assert_eq!(loaded, quote);

    // Pulizia finale
    let _ = std::fs::remove_dir_all(&tmp);
}
