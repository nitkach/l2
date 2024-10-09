use anyhow::Result;
use std::net::Ipv4Addr;

mod app;
mod model;
mod repository;

pub async fn run(address: (Ipv4Addr, u16)) -> Result<()> {
    app::start(address).await?;

    Ok(())
}
