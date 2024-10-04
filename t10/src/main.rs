use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{process::ExitCode, time::Duration};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Parser)]
struct Args {
    host: String,
    port: u16,

    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
}

async fn run(args: Args) -> anyhow::Result<()> {
    let ip = args.host.parse::<Ipv4Addr>()?;
    let socket_address = SocketAddrV4::new(ip, args.port);
    let cancel_token = CancellationToken::new();

    let timeout = Duration::from_secs(args.timeout);

    // try connect to server with timeout
    let tcp_stream = tokio::time::timeout(timeout, TcpStream::connect(socket_address)).await??;

    println!("Successful connected to {socket_address}");

    let (reader, writer) = {
        let (reader, writer) = tcp_stream.into_split();
        (BufReader::new(reader).lines(), writer)
    };

    let token = cancel_token.clone();
    let stdin_to_socket = tokio::spawn(async move {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        let mut writer = writer;

        loop {
            let line = match stdin.next_line().await {
                Ok(Some(line)) => line,
                Ok(None) => {
                    println!("there are no more lines in stdin");
                    break;
                }
                Err(err) => {
                    eprintln!("failed to read new line from stdin: {err}");
                    break;
                }
            };

            if let Err(err) = writer.write_all(line.as_bytes()).await {
                eprintln!("failed to write to socket: {err}");
                break;
            }

            if token.is_cancelled() {
                break;
            }
        }
    });

    let token = cancel_token.clone();
    let socket_to_stdout = tokio::spawn(async move {
        let mut reader = reader;

        loop {
            let line = match reader.next_line().await {
                Ok(Some(line)) => line,
                Ok(None) => {
                    println!("there are no more lines to read from socket");
                    break;
                }
                Err(err) => {
                    eprintln!("failed to read from socket: {err}");
                    break;
                }
            };

            println!("Received: {line}");

            if token.is_cancelled() {
                break;
            }
        }
    });

    tokio::select! {
        _ = stdin_to_socket => {},
        _ = socket_to_stdout => {},
    };

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    if let Err(err) = run(args).await {
        eprintln!("{err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
