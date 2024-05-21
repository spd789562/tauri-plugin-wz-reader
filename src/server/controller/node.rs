use crate::server::extractors::TargetNodeExtractor;
use crate::server::models::GetJsonParam;
use crate::{handlers, Error, Result};

use std::io::{BufWriter, Cursor};

use axum::extract::Query;
use axum::{body::Body, extract::State, http::header, response::IntoResponse};
use image::ImageFormat;
use wz_reader::{property, WzNodeArc, WzNodeCast};

pub async fn get_image(
    State(root): State<WzNodeArc>,
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    let image = handlers::resolve_png(&node, Some(&root))?;

    let mut buf = BufWriter::new(Cursor::new(Vec::new()));
    // maybe use ImageFormat::Webp is better it quicker and smaller.
    image
        .write_to(&mut buf, ImageFormat::WebP)
        .map_err(|_| Error::ImageSendError)?;

    let body = Body::from(buf.into_inner().unwrap().into_inner());

    Ok((
        [
            (header::CONTENT_TYPE, "image/webp"),
            (header::CACHE_CONTROL, "max-age=3600"),
        ],
        body,
    ))
}

pub async fn get_sound(
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    let node_read = node.read().unwrap();

    if let Some(sound) = node_read.try_as_sound() {
        let sound_buf = sound.get_buffer();

        let content_size = sound_buf.len();

        let mini = match sound.sound_type {
            property::WzSoundType::Wav => "audio/wav",
            _ => "audio/mpeg",
        };

        let body = Body::from(sound_buf);

        Ok((
            [
                (header::CONTENT_TYPE, mini.to_string()),
                (header::CONTENT_LENGTH, content_size.to_string()),
            ],
            body,
        ))
    } else {
        Err(Error::NodeTypeMismatch("Sound"))
    }
}

pub async fn get_json(
    Query(param): Query<GetJsonParam>,
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    let is_simple = param.simple.unwrap_or(false);
    let json = if is_simple {
        node.read().unwrap().to_simple_json()
    } else {
        node.read().unwrap().to_json()
    }?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        json.to_string(),
    ))
}
