use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use wz_reader::{property::WzValue, WzNode, WzNodeName, WzObjectType};

pub use models::*;

mod desktop;
#[cfg(feature = "axum-server")]
mod server;

mod commands;
mod error;
mod models;

pub mod handlers;
pub mod utils;

pub use error::{Error, Result};

use desktop::WzReader;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the wz-reader APIs.
pub trait WzReaderExt<R: Runtime> {
    fn wz_reader(&self) -> &WzReader<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WzReaderExt<R> for T {
    fn wz_reader(&self) -> &WzReader<R> {
        self.state::<WzReader<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[cfg(feature = "axum-server")]
    let port = portpicker::pick_unused_port().expect("No port is free");

    let root_node = WzNode::new(
        &WzNodeName::from(""),
        WzObjectType::Value(WzValue::Null),
        None,
    )
    .into_lock();
    let tauri_node = root_node.clone();

    tauri::async_runtime::spawn(server::app(root_node, port));

    let app = Builder::new("wz-reader");

    #[cfg(not(feature = "axum-server"))]
    let app = app.invoke_handler(tauri::generate_handler![commands::execute, commands::init]);
    #[cfg(feature = "axum-server")]
    let app = app.invoke_handler(tauri::generate_handler![
        commands::execute,
        commands::init,
        commands::get_server_url
    ]);

    app.setup(move |app, api| {
        #[cfg(not(feature = "axum-server"))]
        let wz_reader = desktop::init(app, api, tauri_node)?;
        #[cfg(feature = "axum-server")]
        let wz_reader = desktop::init(app, api, tauri_node, port)?;

        app.manage(wz_reader);
        Ok(())
    })
    .build()
}
