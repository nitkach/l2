use anyhow::{Context, Result};
use clap::Parser;
use std::{net::Ipv4Addr, process::ExitCode};
use tracing::debug;

#[derive(Debug, Parser)]
struct Args {
    #[clap(requires = "port")]
    host: Option<String>,

    port: Option<u16>,
}

fn get_address(args: Args) -> Result<(Ipv4Addr, u16)> {
    let address = match (args.host, args.port) {
        (Some(host), Some(port)) => (host.parse::<Ipv4Addr>()?, port),
        (None, None) => {
            let host = std::env::var("HOST")
                .with_context(|| {
                    "pass host and port to app as arguments, or specify HOST and PORT in .env file"
                })?
                .parse::<Ipv4Addr>()?;
            let port = std::env::var("PORT")
                .with_context(|| "specify PORT environment variable in .env file")?
                .parse::<u16>()?;
            (host, port)
        }
        _ => unreachable!(
            "clap restriction on parameters passed; either both are specified or neither"
        ),
    };

    Ok(address)
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    if let Err(err) = dotenvy::dotenv() {
        debug!(".env file: {err}");
    }

    let address = match get_address(args) {
        Ok(address) => address,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = t11::run(address).await {
        eprintln!("{err:?}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}
