use crate::Error;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use wz_reader::{property::WzValue, WzNodeArc, WzObjectType};

pub async fn root_check_middleware(
    State(root): State<WzNodeArc>,
    req: Request,
    next: Next,
) -> Response {
    {
        let root_read = root.read().unwrap();
        if matches!(root_read.object_type, WzObjectType::Value(WzValue::Null)) {
            return Error::NotInitialized.into_response();
        }
    }

    next.run(req).await
}
