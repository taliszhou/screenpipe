use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::query("CREATE VIRTUAL TABLE fts USING fts5(text_col, app_name);").execute(&pool).await?;
    
    // Insert some data
    sqlx::query("INSERT INTO fts (text_col, app_name) VALUES ('hello verify', 'my_app');").execute(&pool).await?;

    // Try normal match
    let res: Vec<(String,)> = sqlx::query_as("SELECT text_col FROM fts WHERE fts MATCH ?")
        .bind("\"verify\"")
        .fetch_all(&pool)
        .await?;
    println!("Normal match: {:?}", res);

    // What if the user query was exactly `verify`?
    let q = "\"verify\"";
    match sqlx::query_as::<_, (String,)>("SELECT text_col FROM fts WHERE fts MATCH ?")
        .bind(q)
        .fetch_all(&pool)
        .await {
            Ok(r) => println!("Quoted match: {:?}", r),
            Err(e) => println!("Error: {:?}", e),
        }

    // What about column syntax?
    let q2 = "verify:\"something\"";
    match sqlx::query_as::<_, (String,)>("SELECT text_col FROM fts WHERE fts MATCH ?")
        .bind(q2)
        .fetch_all(&pool)
        .await {
            Ok(r) => println!("Column match: {:?}", r),
            Err(e) => println!("Error: {:?}", e),
        }

    Ok(())
}
