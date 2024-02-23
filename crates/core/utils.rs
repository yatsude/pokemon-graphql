use axum::response::Html;

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
