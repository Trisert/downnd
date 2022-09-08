pub mod argom;
use crate::argom::Args;

use anyhow::Result;
use clap::Parser;
use futures::stream::StreamExt;
use reqwest::Response;
use std::io::{BufRead, Cursor};
use std::path::Path;
use tokio::fs::File;
use tokio::io::copy;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(url) = args.url.as_deref() {
        let response = reqwest::get(url).await?;

        let mut dest = create_file(&response).await?;

        let mut content = Cursor::new(response.bytes().await?);
        copy(&mut content, &mut dest).await?;
    }

    if let Some(path) = args.path.as_deref() {
        let path = read_line(path)?;
        let fetches = futures::stream::iter(path.into_iter().map(|path| async move {
            match reqwest::get(&path).await {
                Ok(resp) => {
                    let mut dest = create_file(&resp).await.unwrap();
                    let mut content = Cursor::new(resp.bytes().await.unwrap());
                    copy(&mut content, &mut dest).await.unwrap();
                }
                Err(_) => println!("Error in {}", path),
            }
        }))
        .buffer_unordered(100)
        .collect::<Vec<()>>();

        fetches.await;
    }

    Ok(())
}

async fn create_file(resp: &Response) -> Result<File> {
    let file = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|f| f.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("Temp");

        println!("File da scaricare: {fname} ({:.2} MB)", match resp.content_length() {
            Some(num) => num as f64 / (1024 * 1024) as f64,
            None => 0 as f64,
        });

        File::create(fname).await?
    };
    Ok(file)
}

fn read_line(path: &Path) -> Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(file);

    Ok(buf.lines().filter_map(Result::ok).collect())
}
