use sqlx::{query_as, types::chrono};

#[derive(Debug)]
struct Tweet {
    id: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    text: String,
    owner_id: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;

    let test_id = 1;

    // check that inserted todo is visible outside the transaction after commit
    let tweets = query_as!(
        Tweet,
        r#"SELECT id, created_at, text, owner_id FROM tweet WHERE owner_id = $1"#,
        test_id
    )
    .fetch_all(&pool)
    .await;

    assert!(tweets.is_ok());

    println!("Tweets: {:?}", tweets.unwrap());

    Ok(())
}
