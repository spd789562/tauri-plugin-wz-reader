use super::models::GetJsonParam;
use crate::Error;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use wz_reader::{node, WzNodeArc};

pub struct TargetNodeExtractor(pub WzNodeArc);

#[async_trait]
impl<S> FromRequestParts<S> for TargetNodeExtractor
where
    WzNodeArc: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;
        let query = Query::<GetJsonParam>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        let root = WzNodeArc::from_ref(state);

        let root = root.read().unwrap();

        let force_parse = query.force_parse.unwrap_or(false);

        let target = if force_parse {
            root.at_path_parsed(&path).map_err(|e| match e {
                node::Error::NodeNotFound => Error::NodeNotFound,
                _ => Error::NodeError(e),
            })
        } else {
            root.at_path(&path).ok_or(Error::NodeNotFound)
        };

        target
            .map(|node| TargetNodeExtractor(node))
            .map_err(|e| e.into_response())
    }
}

/// extract target node from request, but if any error occurs, instead of rejecting the request, it will return None
pub struct TargetNodeOptionExtractor(pub Option<WzNodeArc>);

#[async_trait]
impl<S> FromRequestParts<S> for TargetNodeOptionExtractor
where
    WzNodeArc: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;
        let query = Query::<GetJsonParam>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        let root = WzNodeArc::from_ref(state);

        let root = root.read().unwrap();

        let force_parse = query.force_parse.unwrap_or(false);

        let target = if force_parse {
            root.at_path_parsed(&path).map_err(|e| match e {
                node::Error::NodeNotFound => Error::NodeNotFound,
                _ => Error::NodeError(e),
            })
        } else {
            root.at_path(&path).ok_or(Error::NodeNotFound)
        };

        Ok(TargetNodeOptionExtractor(target.ok()))
    }
}
