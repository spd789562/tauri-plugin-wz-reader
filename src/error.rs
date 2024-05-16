use serde::{ser::Serializer, Serialize};
use wz_reader::node;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("init wz failed")]
    InitWzFailed,
    #[error("node error: {0}")]
    NodeError(#[from] node::Error),
    #[error("node not found")]
    NodeNotFound,
    #[error("json parse error")]
    JsonParseError(#[from] serde_json::Error),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
