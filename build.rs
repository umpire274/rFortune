#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Assicurati che res/rfortune.ico esista
    let mut res = WindowsResource::new();
    res.set_icon("res/rfortune.ico")
        .set("FileDescription", "rFortune CLI")
        .set("ProductName", "rFortune")
        .set("OriginalFilename", "rfortune.exe")
        .set("FileVersion", env!("CARGO_PKG_VERSION"))
        .set("ProductVersion", env!("CARGO_PKG_VERSION"))
        .compile()
        .expect("Failed to embed icon resource");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
