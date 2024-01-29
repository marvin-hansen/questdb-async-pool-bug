use deadpool_postgres::{Client, Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::GenericClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Configure the connection pool with default values
    // https://questdb.io/docs/develop/connect/
    let mut cfg = Config::new();
    cfg.host = Some("127.0.0.1".to_string());
    cfg.dbname = Some("qdb".to_string());
    cfg.user = Some("admin".to_string());
    cfg.password = Some("quest".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    // RecyclingMethod::Fast does not validate the connection on creation.

    // Create the connection pool
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .expect("Failed to create PG/Quest connection pool");

    // Get a client from the pool
    let client: Client = pool.get().await.expect("Failed to get a client from the pool");

    // Check if the connection actually works
    let connected = client.client().is_closed();

    assert_eq!(connected, true);

    // Failed to get a client from the pool:
    // Backend(Error { kind: Connect, cause: Some(Os { code: 61, kind: ConnectionRefused, message: "Connection refused" }) })

    Ok(())
}