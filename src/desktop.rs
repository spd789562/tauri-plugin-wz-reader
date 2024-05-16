use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use wz_reader::util::resolve_base;
use wz_reader::version::WzMapleVersion;
use wz_reader::{property::WzValue, WzNode, WzNodeArc, WzNodeCast, WzNodeName, WzObjectType};

#[cfg(not(feature = "axum-server"))]
pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    root: WzNodeArc,
) -> crate::Result<WzReader<R>> {
    Ok(WzReader {
        _app: app.clone(),
        node: root,
    })
}
#[cfg(feature = "axum-server")]
pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    root: WzNodeArc,
    port: u16,
) -> crate::Result<WzReader<R>> {
    Ok(WzReader {
        _app: app.clone(),
        node: root,
        port,
    })
}

/// Access to the wz-reader APIs.
pub struct WzReader<R: Runtime> {
    _app: AppHandle<R>,
    #[cfg(feature = "axum-server")]
    pub port: u16,
    pub node: WzNodeArc,
}

impl<R: Runtime> WzReader<R> {
    pub fn is_empty(&self) -> bool {
        matches!(
            self.node.read().unwrap().object_type,
            WzObjectType::Value(WzValue::Null)
        )
    }
    pub fn replace_root(&self, another: &WzNodeArc) {
        let mut node = self.node.write().unwrap();
        std::mem::swap(&mut *node, &mut *another.write().unwrap());
    }
    pub fn init_root(&self, path: &str, version: Option<WzMapleVersion>) -> crate::Result<()> {
        let root = resolve_base(path, version).map_err(|_| crate::Error::InitWzFailed)?;

        self.replace_root(&root);

        Ok(())
    }
}
