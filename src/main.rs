pub mod argom;
use crate::argom::Args;
use downnd::prelude::*;

use anyhow::Result;
use clap::Parser;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(url) = args.url.as_deref() {
        let response = reqwest::get(url).await?;

        download(response).await?;
    }

    if let Some(path) = args.path.as_deref() {
        let path = read_line(path)?;
        let fetches = futures::stream::iter(path.into_iter().map(|path| async move {
            match reqwest::get(&path).await {
                Ok(response) => {
                    download(response).await.unwrap()
                },
                Err(_) => println!("Error in {}", path),
            }
        }))
        .buffer_unordered(100)
        .collect::<Vec<()>>();

        fetches.await;
    }

    Ok(())
}
