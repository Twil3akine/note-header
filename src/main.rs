use axum::{
    response::Html,
    routing::get,
    Router,
};
use chrono::Local;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Launched server at http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<String> {
    let data_str = Local::now().format("%Y-%m-%d").to_string();
    let html = format!(r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <title>現在の日付</title>
    <style>
        body {{
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }}
        .date-display {{
            font-size: 64pt;
            font-weight: bold;
        }}
    </style>
</head>
<body>
    <div class="date-display">{}</div>
</body>
</html>"#, data_str);

    Html(html)
}
