use std::{net::SocketAddr, sync::Arc};


use flybin_common::paste::Paste;

use sqlx::SqlitePool;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use tracing::{debug, info};
use tracing_subscriber::prelude::*;

mod error;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = Arc::new(SqlitePool::connect(&dotenvy::var("DATABASE_URL").unwrap()).await?);

    let cloned_pool = pool.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:9999").await.unwrap();
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            tokio::spawn(handle_connection(stream, addr, cloned_pool.clone()));
        }
    });

    server::run(pool).await?;
    Ok(())
}

async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    pool: Arc<SqlitePool>,
) -> anyhow::Result<()> {
    info!("Accepted connection from {}", addr.ip());
    let mut buf = vec![0; 4096];
    let bytes_read = stream.read(&mut buf).await?;

    let paste = Paste::new(
        String::from_utf8_lossy(&buf[..bytes_read]).to_string(),
        addr.ip().to_string(),
    );
    debug!("{:?}", paste);
    paste.save(&pool).await;

    _ = stream.write(paste.get_response_str().as_bytes()).await?;

    Ok(())
}
