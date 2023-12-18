use crate::prelude::*;
use axum::{extract, response::Html};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RenderUnsafeHtmlReq {
    content: String,
}

pub async fn render_unsafe_html(
    extract::Json(payload): extract::Json<RenderUnsafeHtmlReq>,
) -> Html<String> {
    tracing::debug!("render_unsafe_html: {:?}", payload);

    Html(f!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        payload.content
    ))
}

pub async fn render_safe_html(
    extract::Json(payload): extract::Json<RenderUnsafeHtmlReq>,
) -> Html<String> {
    tracing::debug!("render_unsafe_html: {:?}", payload);

    let encoded_content = html_escape::encode_double_quoted_attribute(payload.content.as_str());

    Html(f!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        encoded_content
    ))
}
