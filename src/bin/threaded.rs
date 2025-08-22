use std::fs;

use sqlx::Executor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <n_threads> <query_path>", args[0]);
        std::process::exit(1);
    }
    let n_threads: i64 = args[1].parse().expect("<n_threads> must be an integer");
    let query_path: String = args[2].parse().expect("<query_path> must be a string");
    let contents = fs::read_to_string(query_path).expect("Should have been able to read the file");

    println!("Spawning {} threads", n_threads);
    println!("Query:\n{}\n-----------------------------", contents);

    let instant = std::time::Instant::now();

    let mut handles = Vec::new();
    for _thread_id in 0..n_threads {
        let pool = pool.clone();
        let contents = contents.clone();
        let h = tokio::spawn(async move {
            let res = pool
                .execute(contents.as_str())
                .await
                .expect("Failed to execute query");

            return res.rows_affected();
        });
        handles.push(h);
    }

    let mut rows = 0;
    for h in handles {
        rows += h.await.expect("The task being joined has panicked");
    }

    println!("Total rows affected {} in {:?}.", rows, instant.elapsed());

    Ok(())
}
