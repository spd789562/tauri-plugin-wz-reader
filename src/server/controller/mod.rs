use axum::{routing::get, Router};
use wz_reader::WzNodeArc;

type State = WzNodeArc;

pub mod mapping;

pub fn mapping_router() -> Router<State> {
    Router::new()
        .route("/smap", get(mapping::get_smap))
        .route("/zmap", get(mapping::get_zmap))
        .route("/images", get(mapping::get_images))
}
