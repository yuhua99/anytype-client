#[tokio::main]
async fn main() -> anyhow::Result<()> {
    anyclient::run().await
}
