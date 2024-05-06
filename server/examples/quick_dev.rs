use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/hello?name=Richard").await?.print().await?;

    hc.do_get("/hello2/Richard").await?.print().await?;
    
    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
