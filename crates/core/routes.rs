use axum::http::header;
use axum::response::{Html, IntoResponse};
use serde_json::json;

pub async fn sandbox() -> Html<String> {
    let body = r#"
        <body style="margin: 0;">
            <div style="width: 100%; height: 100%;" id='embedded-sandbox'></div>
            <script src="https://embeddable-sandbox.cdn.apollographql.com/_latest/embeddable-sandbox.umd.production.min.js"></script>
            <script>
              new window.EmbeddedSandbox({
                target: '#embedded-sandbox',
                initialEndpoint: window.location.href.split('?')[0],
              });
            </script>
        </body>
    "#;

    Html(body.into())
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}

pub async fn root() -> impl IntoResponse {
    let content = json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "rust-version": env!("CARGO_PKG_RUST_VERSION"),
        "authors": env!("CARGO_PKG_AUTHORS")
    })
    .to_string();

    ([(header::CONTENT_TYPE, "application/json")], content)
}
