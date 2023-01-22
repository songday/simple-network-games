use std::collections::HashMap;

use axum::http::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;

const ALL_ASSET_FILES: &[(&str, &[u8], &str)] = &include!("assets.txt");

lazy_static! {
    static ref AEESTS_MAP: HashMap<&'static str, usize> = {
        let mut assets = HashMap::with_capacity(10);
        let mut idx = 0usize;
        for (name, _data, _mime) in ALL_ASSET_FILES {
            assets.insert(*name, idx);
            idx += 1;
        }
        assets
    };
}

pub(crate) async fn assets_handler(uri: Uri) -> Response {
    let p = uri.path();
    let asset = AEESTS_MAP.get(p);
    if asset.is_some() {
        let f = ALL_ASSET_FILES[*asset.unwrap()];
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static(f.2),
        );
        headers.insert(
            HeaderName::from_static("content-length"),
            HeaderValue::from(f.1.len()),
        );
        headers.insert(
            HeaderName::from_static("content-encoding"),
            HeaderValue::from_static("gzip"),
        );
        // let builder = axum::response::Response::builder();
        // builder.header("Content-Length", f.1.len())
        // .header("Content-Encoding", "gzip")
        // .status(StatusCode::OK).body(axum::body::Bytes::from_ref(f.1)).unwrap()
        (StatusCode::OK, headers, axum::body::Bytes::from_static(f.1)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "404").into_response()
    }
}
