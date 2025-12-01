use anyhow::{Context, Result};
use clap::Parser;
use std::net::SocketAddr;

mod model;
mod speech;
mod web;

#[derive(Debug, Parser, Clone)]
struct Cli {
    #[clap(long, env)]
    #[clap(default_value = "0.0.0.0:3000")]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    speech::init().expect("Failed to initialize speech engine");

    let cli = Cli::parse();
    let listen = cli.listen;

    let listener = tokio::net::TcpListener::bind(listen)
        .await
        .with_context(|| format!("Failed to bind address {listen}"))?;

    tracing::info!("Listening on {}", listen);

    web::serve(listener).await?;

    Ok(())
}
