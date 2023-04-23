#[async_trait::async_trait]
trait ConnectionTrait {
    const CONN_URL: Option<String>;
    async fn query<P, R>(statement: String, params: P) -> anyhow::Result<R>;
    async fn close() -> anyhow::Result<()>;
}
