use anyhow::Result;
use axum::{Router, response::Html, routing::get};
use chrono::{Local, NaiveDate};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(date_handler))
        .route("/streak", get(streak_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Launched server at http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn date_handler() -> Html<String> {
    let data_str = Local::now().format("%Y-%m-%d").to_string();
    let html = format!(
        r#"
				<!DOCTYPE html>
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
				</html>
    "#,
        data_str
    );

    Html(html)
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
