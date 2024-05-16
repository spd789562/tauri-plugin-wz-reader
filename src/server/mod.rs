use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query, Request, State},
    http::{header, request::Parts, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    serve, Router,
};
use serde::Deserialize;
use wz_reader::{node, property::WzValue, WzNodeArc, WzObjectType};

pub async fn app(node: WzNodeArc, port: u16) -> std::io::Result<()> {
    let layer_state = node.clone();
    let app = Router::new()
        .route("/", get(hello))
        .route("/node/*path", get(get_print_full_path))
        .route_layer(middleware::from_fn_with_state(
            layer_state,
            root_check_middleware,
        ))
        .with_state(node);

    let host = format!("127.0.0.1:{port}");

    println!("You enable the axum-server feature, Listening on http://{host}");

    let listener = tokio::net::TcpListener::bind(host).await?;

    serve(listener, app).await
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn get_print_full_path(
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<String, Response> {
    Ok(node.read().unwrap().get_full_path())
}

enum NodeFindError {
    Uninitialized,
    NotFound,
    TypeMismatch,
    ServerError,
    ParseError,
}
impl IntoResponse for NodeFindError {
    fn into_response(self) -> Response {
        match self {
            NodeFindError::Uninitialized => (
                StatusCode::BAD_REQUEST,
                "wz uninitialized, please use init command first",
            )
                .into_response(),
            NodeFindError::NotFound => (StatusCode::NOT_FOUND, "node not found").into_response(),
            NodeFindError::TypeMismatch => {
                (StatusCode::BAD_REQUEST, "node type can't use on this route").into_response()
            }
            NodeFindError::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something wrong when parsing data",
            )
                .into_response(),
            NodeFindError::ParseError => {
                (StatusCode::BAD_REQUEST, "node parse error").into_response()
            }
        }
    }
}
impl From<node::Error> for NodeFindError {
    fn from(e: node::Error) -> Self {
        match e {
            node::Error::NodeNotFound => NodeFindError::NotFound,
            _ => NodeFindError::ParseError,
        }
    }
}

#[derive(Deserialize)]
struct GetJsonParam {
    simple: Option<bool>,
    force_parse: Option<bool>,
    sort: Option<bool>,
}

async fn root_check_middleware(
    State(root): State<WzNodeArc>,
    req: Request,
    next: Next,
) -> Response {
    {
        let root_read = root.read().unwrap();
        if matches!(root_read.object_type, WzObjectType::Value(WzValue::Null)) {
            return NodeFindError::Uninitialized.into_response();
        }
    }

    next.run(req).await
}

pub struct TargetNodeExtractor(WzNodeArc);

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
            root.at_path_parsed(&path).map_err(NodeFindError::from)
        } else {
            root.at_path(&path).ok_or(NodeFindError::NotFound)
        };

        target
            .map(|node| TargetNodeExtractor(node))
            .map_err(|e| e.into_response())
    }
}
