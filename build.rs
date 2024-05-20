#[cfg(not(feature = "axum-server"))]
const COMMANDS: &[&str] = &[
    "init",
    "execute",
    "parse_node",
    "unparse_node",
    "get_node_info",
    "get_childs_info",
];
#[cfg(feature = "axum-server")]
const COMMANDS: &[&str] = &[
    "init",
    "execute",
    "get_server_url",
    "parse_node",
    "unparse_node",
    "get_childs_info",
    "get_node_info",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
