use crate::{handlers, utils, Error, Result, WzReader};
use tauri::{async_runtime, command, AppHandle, Runtime, State, Window};
use wz_reader::node::parse_node;
use wz_reader::version::WzMapleVersion;

#[cfg(feature = "axum-server")]
#[command]
pub(crate) async fn get_server_url<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, WzReader<R>>,
) -> Result<String> {
    Ok(format!("http://127.0.0.1:{}", state.port))
}

#[command]
pub(crate) async fn execute<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, WzReader<R>>,
) -> Result<String> {
    let node = state.node.read().unwrap();

    let node = node
        .at_path("Etc/BossLucid.img")
        .ok_or(Error::NodeNotFound)?;

    parse_node(&node)?;

    let json = node.read().unwrap().to_simple_json()?;

    Ok(json.to_string())
}

#[command]
pub(crate) async fn init<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, WzReader<R>>,
    path: String,
    version: Option<String>,
) -> Result<()> {
    let version = version.map(|s| match s.as_str() {
        "GMS" => WzMapleVersion::GMS,
        "EMS" => WzMapleVersion::EMS,
        "BMS" => WzMapleVersion::BMS,
        _ => WzMapleVersion::UNKNOWN,
    });

    let base_node = utils::resolve_base(&path, version)
        .await
        .map_err(|_| crate::Error::InitWzFailed)?;

    state.replace_root(&base_node);

    Ok(())
}
