use std::{collections::HashMap, process::ExitCode, sync::Arc};

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use itertools::Itertools;

#[derive(Debug, Parser)]
struct Args {
    input_path: Utf8PathBuf,

    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

async fn run(args: Args) -> Result<()> {
    let contents = std::fs::read_to_string(args.input_path)?;
    let contents = contents.to_lowercase();
    let contents_len = contents.len();

    if contents_len == 0 {
        return Err(anyhow::anyhow!("content is empty"));
    }

    let map = Arc::new(dashmap::DashMap::new());

    let chunk_size = (contents_len + args.threads - 1) / args.threads;
    let chunks = contents
        .chars()
        .chunks(chunk_size)
        .into_iter()
        .map(std::iter::Iterator::collect)
        .collect::<Vec<String>>();

    let mut handles = Vec::new();

    let now = std::time::Instant::now();

    for chunk in chunks {
        let local_map = Arc::clone(&map);

        let join_handle = tokio::spawn(async move {
            for char in chunk.chars() {
                if char.is_ascii_alphabetic() {
                    *local_map.entry(char).or_insert(0) += 1;
                }
            }
        });

        handles.push(join_handle);
    }

    for handle in handles {
        handle.await?;
    }

    let elapsed = format!("{:.3} seconds", now.elapsed().as_secs_f64());

    let map = Arc::into_inner(map)
        .expect("all tasks joined, only one strong reference exist")
        .into_iter()
        .collect::<HashMap<_, _>>();

    let json = serde_json::json!({
        "elapsed": elapsed,
        "result": map,
    });

    println!("{json:#}");

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
