#[cfg(not(feature = "axum-server"))]
const COMMANDS: &[&str] = &["init", "execute"];
#[cfg(feature = "axum-server")]
const COMMANDS: &[&str] = &["init", "execute", "get_server_url"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
