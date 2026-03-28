use anyhow::Result;
use axum::{Router, extract::Path, http::StatusCode, response::Html, routing::get};
use chrono::{Local, NaiveDate};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(date_handler))
        .route("/streak", get(streak_handler))
        // :date_str としてパスパラメータを受け取る
        .route("/{date_str}", get(specific_date_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Launched server at http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn date_handler() -> Html<String> {
    let data_str = Local::now().format("%Y-%m-%d").to_string();
    generate_date_html(&data_str)
}

async fn streak_handler() -> Html<String> {
    let start_date = NaiveDate::from_ymd_opt(2026, 3, 23).unwrap();
    let current_data = Local::now().date_naive();

    let duration = current_data - start_date;
    let streak_days = duration.num_days() + 1;

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="ja">
        <head>
            <meta charset="UTF-8">
            <title>継続日数</title>
            <style>
                body {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }}
                .streak-display {{
                    font-size: 64pt;
                    font-weight: bold;
                }}
            </style>
        </head>
        <body>
            <div class="streak-display">{}日目</div>
        </body>
        </html>
        "#,
        streak_days
    );

    Html(html)
}

// 新しく追加したハンドラー
async fn specific_date_handler(
    Path(date_str): Path<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    // 受け取った文字列が yyyy-mm-dd 形式の正しい日付か検証する
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(_) => {
            // 正しい日付ならHTMLを返す
            Ok(generate_date_html(&date_str))
        }
        Err(_) => {
            // フォーマットが不正な場合は400エラーを返す
            Err((
                StatusCode::BAD_REQUEST,
                "無効な日付フォーマットです。YYYY-MM-DD 形式で指定してください。".to_string(),
            ))
        }
    }
}

// HTMLの生成処理が重複するため関数化しました
fn generate_date_html(date_str: &str) -> Html<String> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="ja">
        <head>
            <meta charset="UTF-8">
            <title>指定の日付</title>
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
        </html>
        "#,
        date_str
    );

    Html(html)
}
