use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use reqwest::{Method, Url};
use scraper::{Html, Selector};
use std::{
    collections::{HashSet, VecDeque},
    process::ExitCode,
};

/// Utility to download entire websites.
#[derive(Debug, Parser)]
struct Args {
    /// Website URL to download
    url: Url,

    /// Path to store downloaded pages
    #[arg(long)]
    save_path: Option<Utf8PathBuf>,
}

async fn run(args: Args) -> Result<()> {
    if is_file_to_skip(&args.url) {
        return Err(anyhow!("cannot download files, only webpages"));
    }

    let save_root = args.save_path.unwrap_or(get_domain(&args.url)?.into());
    println!("Save root: {save_root}");

    let client = reqwest::ClientBuilder::new()
        .user_agent("wget utility")
        .build()?;

    let mut visited_urls = HashSet::new();
    let mut deque: VecDeque<Url> = VecDeque::new();
    deque.push_back(args.url);

    while let Some(current_url) = deque.pop_front() {
        if visited_urls.contains(&current_url) {
            continue;
        }

        if is_file_to_skip(&current_url) {
            continue;
        }

        // download site
        println!("Downloading {current_url}...");
        let now = std::time::Instant::now();
        let Ok(html) = download(current_url.clone(), &client).await else {
            continue;
        };
        println!("Done! {:.3} seconds", now.elapsed().as_secs_f64());

        // save to directory
        let mut path = save_root.join(current_url.path().trim_start_matches('/'));

        let now = std::time::Instant::now();

        let format = format!("Saved file: {path}");
        tokio::fs::create_dir_all(&path).await?;
        path.push("page.html");

        tokio::fs::write(path, &html).await?;

        let elapsed = now.elapsed().as_secs_f64();
        println!("({elapsed:.3} secs) {format}");

        // add to visited urls
        visited_urls.insert(current_url.clone());

        // get urls, push to deque
        println!("Getting links");
        let links = get_links(&html, &current_url);

        for link in links {
            if !visited_urls.contains(&link) {
                deque.push_back(link);
            }
        }
    }

    Ok(())
}

fn get_links(html: &str, current_url: &Url) -> HashSet<Url> {
    let mut links = HashSet::new();
    let document = Html::parse_document(html);
    let links_selector = Selector::parse("a, script, link").expect("valid str for selector");

    for element in document.select(&links_selector) {
        if let Some(url) = element.attr("href").or(element.attr("src")) {
            if let Ok(url) = current_url.join(url) {
                if is_file_to_skip(&url) {
                    continue;
                }
                links.insert(url);
            }
        }
    }

    links
}

async fn download(url: Url, client: &reqwest::Client) -> Result<String> {
    let request = client.request(Method::GET, url).build()?;
    let response = client.execute(request).await?;
    let text = response.text().await?;
    Ok(text)
}

fn get_domain(url: &Url) -> Result<&str> {
    url.domain()
        .ok_or_else(|| anyhow!("incorrect domain: {url}"))
}

fn is_file_to_skip(url: &Url) -> bool {
    let extensions = [
        "png", "jpg", "jpeg", "gif", "pdf", "doc", "docx", "xls", "xlsx", "zip", "tar", "gz",
    ];

    if let Some(last) = url.path().split('.').last() {
        return extensions.contains(&last);
    }

    false
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
